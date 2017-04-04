use lua::{Index, Type, ToLua, FromLua, State};
use types::{LuaStackable};
use context::Context;

pub struct LuaGeneric {
    index: Index
}

impl LuaGeneric {
    pub fn new(i: Index) -> LuaGeneric {
        LuaGeneric {
            index: i
        }
    }

    pub fn type_of(&self, context: &mut Context) -> Type {
        context.get_state().type_of(self.index).unwrap()
    }

    pub fn get_value<T: FromLua>(&self, context: &mut Context) -> Option<T> {
        context.get_state().to_type(self.index)
    }
}

impl LuaStackable for LuaGeneric {
    fn get_pos(&self) -> Index {
        self.index
    }
}

impl ToLua for LuaGeneric {
    fn to_lua(&self, state: &mut State) {
        state.push_value(self.get_pos());
    }
}

impl FromLua for LuaGeneric {
    fn from_lua(_: &mut State, index: Index) -> Option<LuaGeneric> {
        Some(LuaGeneric::new(index))
    }
}
