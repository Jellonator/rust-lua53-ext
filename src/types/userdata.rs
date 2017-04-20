use lua::{Index, ToLua, FromLua, State};
use types::{LuaStackable, LuaTable};
use context::Context;

/// Represents user-defined data on the Lua stack
///
/// Note that without a __gc metamethod, any data contained in the userdata that implements the
/// Drop trait will not be dropped, which although not unsafe, may result in memory leaks.
/// This includes types such as HashMap and Vec.
pub struct LuaUserdata {
    index: Index
}

impl LuaUserdata {
    /// Create a new LuaUserdata at the given index
    pub fn new(i: Index) -> LuaUserdata {
        LuaUserdata {
            index: i
        }
    }

    /// Set this userdata's metatable.
    pub fn set_metatable(&self, context: &mut Context, meta: &LuaTable) {
        context.get_state().push_value(meta.get_pos());
        context.get_state().set_metatable(self.get_pos());
    }

    /// Get a mutable reference to this userdata's contained data.
    pub unsafe fn get_value<'a, T>(&self, context: &'a mut Context) -> Option<&'a mut T> {
        context.get_state().to_userdata_typed(self.index)
    }

    /// Get a mutable reference to this userdata's contained data, given that its metatable
    /// matches the given name.
    pub unsafe fn get_value_named<'a, T>(&self, context: &'a mut Context, name: &str)
            -> Option<&'a mut T> {
        context.get_state().test_userdata_typed(self.index, name)
    }
}

impl LuaStackable for LuaUserdata {
    fn get_pos(&self) -> Index {
        self.index
    }
}

impl ToLua for LuaUserdata {
    fn to_lua(&self, state: &mut State) {
        state.push_value(self.get_pos());
    }
}

impl FromLua for LuaUserdata {
    fn from_lua(state: &mut State, index: Index) -> Option<LuaUserdata> {
        if state.is_userdata(index) {
            Some(LuaUserdata::new(index))
        } else {
            None
        }
    }
}
