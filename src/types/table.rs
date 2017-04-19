use lua::{Index, ToLua, FromLua, State, Type};
use types::{LuaStackable, LuaGeneric};
use context::Context;

pub struct LuaTable {
    index: Index
}

impl LuaTable {
    pub fn new(i: Index) -> LuaTable {
        LuaTable {
            index: i
        }
    }

    pub fn set_metatable(&self, context: &mut Context, meta: &LuaTable) {
        context.get_state().push_value(meta.get_pos());
        context.get_state().set_metatable(self.get_pos());
    }

    pub fn set(&self, context: &mut Context, key: &ToLua, value: &ToLua) {
        key.to_lua(context.get_state());
        value.to_lua(context.get_state());
        context.get_state().set_table(self.get_pos());
    }

    pub fn set_raw(&self, context: &mut Context, key: &ToLua, value: &ToLua) {
        key.to_lua(context.get_state());
        value.to_lua(context.get_state());
        context.get_state().raw_set(self.get_pos());
    }

    pub fn get(&self, context: &mut Context, key: &ToLua) -> LuaGeneric {
        key.to_lua(context.get_state());
        context.get_state().get_table(self.get_pos());
        LuaGeneric::new(context.get_state().get_top())
    }

    pub fn get_raw(&self, context: &mut Context, key: &ToLua) -> LuaGeneric {
        key.to_lua(context.get_state());
        context.get_state().raw_get(self.get_pos());
        LuaGeneric::new(context.get_state().get_top())
    }

    pub fn get_typed<T: FromLua>(&self, context: &mut Context, key: &ToLua) -> Option<T> {
        key.to_lua(context.get_state());
        context.get_state().get_table(self.get_pos());
        let top = context.get_state().get_top();
        let ret = context.get_state().to_type(top);
        ret
    }

    pub fn iter_array<F>(&self, context: &mut Context, mut func: F)
            where F: FnMut(Context, i64, LuaGeneric) {
        let mut key = 1;
        loop {
            let mut new_context = context.push_context();
            let value = self.get(&mut new_context, &key);
            let t = value.type_of(&mut new_context);
            key += 1;
            match t {
                Type::Nil => break,
                _ => func(new_context, key, value)
            };
        }
    }

    pub fn len_raw(&self, context: &mut Context) -> usize {
        context.get_state().raw_len(self.get_pos())
    }

    pub fn len(&self, context: &mut Context) -> i64 {
        context.get_state().len_direct(self.get_pos())
    }

    pub fn append(&self, context: &mut Context, value: &ToLua) {
        let length = self.len(context);
        self.set(context, &(length+1), value);
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
