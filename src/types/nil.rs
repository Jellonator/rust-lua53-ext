use lua::{Index, ToLua, FromLua, State};
use types::{LuaStackable};

/// Represents a nil value on the Lua stack
pub struct LuaNil {
    index: Index
}

impl LuaNil {
    /// Create a new LuaNil given an index
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

impl ToLua for LuaNil {
    fn to_lua(&self, state: &mut State) {
        state.push_value(self.get_pos());
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
