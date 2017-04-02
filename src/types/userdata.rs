use lua::{Index, FromLua, State};
use types::{LuaStackable};
use context::Context;

pub struct LuaUserdata {
    index: Index
}

impl LuaUserdata {
    pub fn new(i: Index) -> LuaUserdata {
        LuaUserdata {
            index: i
        }
    }

    pub unsafe fn get_value<'a, T>(&self, context: &'a mut Context) -> Option<&'a mut T> {
        context.get_state().to_userdata_typed(self.index)
    }
}

impl LuaStackable for LuaUserdata {
    fn get_pos(&self) -> Index {
        self.index
    }
}

impl FromLua for LuaUserdata {
    fn from_lua(state: &mut State, index: Index) -> Option<LuaUserdata> {
        if state.is_nil(index) {
            Some(LuaUserdata::new(index))
        } else {
            None
        }
    }
}
