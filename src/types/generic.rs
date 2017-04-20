use lua::{Index, Type, ToLua, FromLua, State};
use types::{LuaStackable};
use context::Context;

/// Represents a generic lua value on the Lua stack
///
/// Can represent any possible lua value, and can be converted to a usable value.
///
/// # Examples
///
/// ```
/// # use luaext::lua::State;
/// # use luaext::context::Context;
/// # use luaext::types::generic::LuaGeneric;
/// # let mut state = State::new();
/// # let mut context = Context::new(&mut state);
/// context.push_integer(4);
/// let generic: LuaGeneric = context.get_arg(1).unwrap();
/// assert_eq!(Some(4), generic.get_value(&mut context));
/// ```
pub struct LuaGeneric {
    index: Index
}

impl LuaGeneric {
    /// Create a new LuaGeneric given an index
    pub fn new(i: Index) -> LuaGeneric {
        LuaGeneric {
            index: i
        }
    }

    /// Gets the Lua type of this generic object
    pub fn type_of(&self, context: &mut Context) -> Type {
        context.get_state().type_of(self.index).unwrap()
    }

    /// Convert the contained value of this generic object to a given type
    pub fn get_value<T: FromLua>(&self, context: &mut Context) -> Option<T> {
        context.get_state().to_type(self.index)
    }
}

impl LuaStackable for LuaGeneric {
    fn get_pos(&self) -> Index {
        self.index
    }
}

impl ToLua for LuaGeneric {
    fn to_lua(&self, state: &mut State) {
        state.push_value(self.get_pos());
    }
}

impl FromLua for LuaGeneric {
    fn from_lua(_: &mut State, index: Index) -> Option<LuaGeneric> {
        Some(LuaGeneric::new(index))
    }
}
