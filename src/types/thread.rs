use lua::{Index, ToLua, FromLua, State};
use types::LuaStackable;
use context::Context;

/// Represents a Lua thread on the Lua stack
pub struct LuaThread {
    index: Index
}

impl LuaThread {
    /// Create a new LuaThread at the given index
    pub fn new(i: Index) -> LuaThread {
        LuaThread {
            index: i
        }
    }

    /// Get the contained State
    pub fn as_state(&self, context: &mut Context) -> State {
        // Unwrapping here is safe because LuaThread is guaranteed to refer to a State
        context.get_state().to_thread(self.index).unwrap()
    }
}

impl LuaStackable for LuaThread {
    fn get_pos(&self) -> Index {
        self.index
    }
}

impl ToLua for LuaThread {
    fn to_lua(&self, state: &mut State) {
        state.push_value(self.get_pos());
    }
}

impl FromLua for LuaThread {
    fn from_lua(state: &mut State, index: Index) -> Option<LuaThread> {
        if state.is_thread(index) {
            Some(LuaThread::new(index))
        } else {
            None
        }
    }
}
