use lua::{Index, FromLua, State};
use types::{LuaStackable};
use context::Context;

pub struct LuaBool {
    index: Index
}

impl LuaBool {
    pub fn new(i: Index) -> LuaBool {
        LuaBool {
            index: i
        }
    }

    pub fn get_value(&self, context: &mut Context) -> bool {
        context.get_state().to_bool(self.index)
    }
}

impl LuaStackable for LuaBool {
    fn get_pos(&self) -> Index {
        self.index
    }
}

impl FromLua for LuaBool {
    fn from_lua(state: &mut State, index: Index) -> Option<LuaBool> {
        if state.is_bool(index) {
            Some(LuaBool::new(index))
        } else {
            None
        }
    }
}
