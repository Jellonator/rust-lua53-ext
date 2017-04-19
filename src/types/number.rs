use lua::{Index, ToLua, FromLua, State};
use types::{LuaStackable};
use context::Context;

/// Represents a floating-point number on the Lua Stack
pub struct LuaNumber {
    index: Index
}

impl LuaNumber {
    /// Create a new LuaNumber given an index
    pub fn new(i: Index) -> LuaNumber {
        LuaNumber {
            index: i
        }
    }

    /// Get the value of this number
    pub fn get(&self, context: &mut Context) -> f64 {
        context.get_state().to_number(self.get_pos())
    }
}

impl LuaStackable for LuaNumber {
    fn get_pos(&self) -> Index {
        self.index
    }
}

impl ToLua for LuaNumber {
    fn to_lua(&self, state: &mut State) {
        state.push_value(self.get_pos());
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
