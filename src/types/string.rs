use lua::{Index, ToLua, FromLua, State};
use types::{LuaStackable};
use context::Context;

/// Represents a String on the Lua Stack
pub struct LuaString {
    index: Index
}

impl LuaString {
    /// Create a new String given an index
    pub fn new(i: Index) -> LuaString {
        LuaString {
            index: i
        }
    }

    /// Get the value of this string
    pub fn get<'a>(&self, context: &'a mut Context) -> &'a str {
        context.get_state().to_str(self.get_pos()).unwrap()
    }
}

impl LuaStackable for LuaString {
    fn get_pos(&self) -> Index {
        self.index
    }
}

impl ToLua for LuaString {
    fn to_lua(&self, state: &mut State) {
        state.push_value(self.get_pos());
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
