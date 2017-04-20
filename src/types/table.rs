use lua::{Index, ToLua, FromLua, State, Type};
use types::{LuaStackable, LuaGeneric};
use context::Context;

/// Represents a Lua table on the Lua stack.
pub struct LuaTable {
    index: Index
}

impl LuaTable {
    /// Create a new LuaTable at the given index.
    pub fn new(i: Index) -> LuaTable {
        LuaTable {
            index: i
        }
    }

    /// Set this table's metatable.
    ///
    /// Equivalent to the Lua `setmetatable` function
    pub fn set_metatable(&self, context: &mut Context, meta: &LuaTable) {
        context.get_state().push_value(meta.get_pos());
        context.get_state().set_metatable(self.get_pos());
    }

    /// Set a value in this table
    /// May call the __newindex metamethod
    ///
    /// Equivalent to `table[key] = value` in Lua
    pub fn set(&self, context: &mut Context, key: &ToLua, value: &ToLua) {
        key.to_lua(context.get_state());
        value.to_lua(context.get_state());
        context.get_state().set_table(self.get_pos());
    }

    /// Set a value in this table without invoking metamethods
    ///
    /// Equivalent to the Lua `rawset` function
    pub fn set_raw(&self, context: &mut Context, key: &ToLua, value: &ToLua) {
        key.to_lua(context.get_state());
        value.to_lua(context.get_state());
        context.get_state().raw_set(self.get_pos());
    }

    /// Get a value from this table
    /// May call the __index metamethod
    ///
    /// Equivalent to `table[key]` in Lua
    pub fn get(&self, context: &mut Context, key: &ToLua) -> LuaGeneric {
        key.to_lua(context.get_state());
        context.get_state().get_table(self.get_pos());
        LuaGeneric::new(context.get_state().get_top())
    }

    /// Get a value from this table without invoking metamethods
    ///
    /// Equivalent to the Lua `rawget` function
    pub fn get_raw(&self, context: &mut Context, key: &ToLua) -> LuaGeneric {
        key.to_lua(context.get_state());
        context.get_state().raw_get(self.get_pos());
        LuaGeneric::new(context.get_state().get_top())
    }

    /// Get a value from this table as type T
    pub fn get_typed<T: FromLua>(&self, context: &mut Context, key: &ToLua) -> Option<T> {
        key.to_lua(context.get_state());
        context.get_state().get_table(self.get_pos());
        let top = context.get_state().get_top();
        let ret = context.get_state().to_type(top);
        ret
    }

    /// Count the number of elements in this table as an array
    /// May call the `__len` metamethod
    ///
    /// Equivalent to the Lua `#` operator
    ///
    /// # Panics
    ///
    /// This method will panic if this table has a `__len`
    /// metamethod that does not return an integer
    pub fn len(&self, context: &mut Context) -> i64 {
        context.get_state().len_direct(self.get_pos())
    }

    /// Count the number of elements in this table as an array without calling the `__len` metamethod
    ///
    /// Equivalent to the Lua `rawlen` function
    pub fn len_raw(&self, context: &mut Context) -> usize {
        context.get_state().raw_len(self.get_pos())
    }

    /// Iterate through every (i, value) pair where i is an integer and is continuous from 1 to
    /// this table's length.
    ///
    /// Similar to the Lua `ipairs` function
    pub fn iter_array<F>(&self, context: &mut Context, mut func: F)
            where F: FnMut(Context, i64, LuaGeneric) {
        for key in 1.. {
            let mut new_context = context.push_context();
            let value = self.get(&mut new_context, &key);
            let t = value.type_of(&mut new_context);
            match t {
                Type::Nil => break,
                _ => func(new_context, key, value)
            };
        }
    }

    /// Add an element to the end of the table
    ///
    /// Equivalent to the Lua `table.insert` function
    pub fn append(&self, context: &mut Context, value: &ToLua) {
        let length = self.len_raw(context);
        self.set(context, &(length as i64+1), value);
    }
}

impl LuaStackable for LuaTable {
    fn get_pos(&self) -> Index {
        self.index
    }
}

impl ToLua for LuaTable {
    fn to_lua(&self, state: &mut State) {
        state.push_value(self.get_pos());
    }
}

impl FromLua for LuaTable {
    fn from_lua(state: &mut State, index: Index) -> Option<LuaTable> {
        if state.is_table(index) {
            Some(LuaTable::new(index))
        } else {
            None
        }
    }
}
