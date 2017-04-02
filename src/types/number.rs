use lua::{Index, FromLua, State};
use types::{LuaStackable};
use context::Context;

pub struct LuaNumber {
    index: Index
}

impl LuaNumber {
    pub fn new(i: Index) -> LuaNumber {
        LuaNumber {
            index: i
        }
    }

    pub fn get(&self, context: &mut Context) -> f64 {
        context.get_state().to_number(self.get_pos())
    }
}

impl LuaStackable for LuaNumber {
    fn get_pos(&self) -> Index {
        self.index
    }
}

impl FromLua for LuaNumber {
    fn from_lua(state: &mut State, index: Index) -> Option<LuaNumber> {
        if state.is_number(index) {
            Some(LuaNumber::new(index))
        } else {
            None
        }
    }
}
