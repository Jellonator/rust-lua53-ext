use lua::{Index, ToLua, FromLua, State};
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

    pub fn get(&self, context: &mut Context, key: &ToLua) -> LuaGeneric {
        key.to_lua(context.get_state());
        context.get_state().get_table(self.get_pos());
        LuaGeneric::new(context.get_state().get_top())
    }

    pub fn get_typed<T: FromLua>(&self, context: &mut Context, key: &ToLua) -> Option<T> {
        key.to_lua(context.get_state());
        context.get_state().get_table(self.get_pos());
        let top = context.get_state().get_top();
        let ret = context.get_state().to_type(top);
        context.get_state().pop(1);
        ret
    }
}

impl LuaStackable for LuaTable {
    fn get_pos(&self) -> Index {
        self.index
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
