use lua::{Index, FromLua, State};
use types::{LuaStackable};
use context::Context;

pub struct LuaInteger {
    index: Index
}

impl LuaInteger {
    pub fn new(i: Index) -> LuaInteger {
        LuaInteger {
            index: i
        }
    }

    pub fn get(&self, context: &mut Context) -> i64 {
        context.get_state().to_integer(self.get_pos())
    }
}

impl LuaStackable for LuaInteger {
    fn get_pos(&self) -> Index {
        self.index
    }
}

impl FromLua for LuaInteger {
    fn from_lua(state: &mut State, index: Index) -> Option<LuaInteger> {
        if state.is_integer(index) {
            Some(LuaInteger::new(index))
        } else {
            None
        }
    }
}
