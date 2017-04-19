use lua::{Index, ToLua, FromLua, State};
use types::{LuaStackable};
use context::Context;

/// Represents an integer on the Lua Stack
pub struct LuaInteger {
    index: Index
}

impl LuaInteger {
    /// Create a new LuaInteger given an index
    pub fn new(i: Index) -> LuaInteger {
        LuaInteger {
            index: i
        }
    }

    /// Get the value of this integer
    pub fn get(&self, context: &mut Context) -> i64 {
        context.get_state().to_integer(self.get_pos())
    }
}

impl LuaStackable for LuaInteger {
    fn get_pos(&self) -> Index {
        self.index
    }
}

impl ToLua for LuaInteger {
    fn to_lua(&self, state: &mut State) {
        state.push_value(self.get_pos());
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
