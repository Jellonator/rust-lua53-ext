use lua::{Index, FromLua, State};
use types::{LuaStackable};

pub struct LuaNil {
    index: Index
}

impl LuaNil {
    pub fn new(i: Index) -> LuaNil {
        LuaNil {
            index: i
        }
    }
}

impl LuaStackable for LuaNil {
    fn get_pos(&self) -> Index {
        self.index
    }
}

impl FromLua for LuaNil {
    fn from_lua(state: &mut State, index: Index) -> Option<LuaNil> {
        if state.is_nil(index) {
            Some(LuaNil::new(index))
        } else {
            None
        }
    }
}
