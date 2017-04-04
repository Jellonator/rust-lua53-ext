use lua::{Index, ToLua, FromLua, State};
use types::{LuaStackable, LuaTable};
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

    pub fn set_metatable(&self, context: &mut Context, meta: &LuaTable) {
        context.get_state().push_value(meta.get_pos());
        context.get_state().set_metatable(self.get_pos());
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
