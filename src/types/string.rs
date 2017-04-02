use lua::{Index, FromLua, State};
use types::{LuaStackable};
use context::Context;

pub struct LuaString {
    index: Index
}

impl LuaString {
    pub fn new(i: Index) -> LuaString {
        LuaString {
            index: i
        }
    }

    pub fn get<'a>(&self, context: &'a mut Context) -> &'a str {
        context.get_state().to_str(self.get_pos()).unwrap()
    }
}

impl LuaStackable for LuaString {
    fn get_pos(&self) -> Index {
        self.index
    }
}

impl FromLua for LuaString {
    fn from_lua(state: &mut State, index: Index) -> Option<LuaString> {
        if state.is_string(index) {
            Some(LuaString::new(index))
        } else {
            None
        }
    }
}
