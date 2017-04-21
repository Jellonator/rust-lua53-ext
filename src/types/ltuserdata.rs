use lua::{Index, ToLua, FromLua, State};
use types::LuaStackable;

/// Represents a pointer on the Lua stack
///
/// Note that you can not retrive the value of light userdata from Lua.
pub struct LuaLightUserdata {
    index: Index
}

impl LuaLightUserdata {
    /// Create a new LuaLightUserdata at the given index
    pub fn new(i: Index) -> LuaLightUserdata {
        LuaLightUserdata {
            index: i
        }
    }
}

impl LuaStackable for LuaLightUserdata {
    fn get_pos(&self) -> Index {
        self.index
    }
}

impl ToLua for LuaLightUserdata {
    fn to_lua(&self, state: &mut State) {
        state.push_value(self.get_pos());
    }
}

impl FromLua for LuaLightUserdata {
    fn from_lua(state: &mut State, index: Index) -> Option<LuaLightUserdata> {
        if state.is_light_userdata(index) {
            Some(LuaLightUserdata::new(index))
        } else {
            None
        }
    }
}
