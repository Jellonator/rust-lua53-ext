use lua::{Index, ToLua, FromLua, State, MULTRET};
use types::{LuaStackable, LuaGeneric};
use context::Context;
use error;

/// Reperesents a callable function on the Lua stack
///
/// Functions that can be called may be defined in Lua, Rust, C, or any other language with Lua
/// bindings.
///
/// # Examples
///
/// ```
/// # use luaext::lua::State;
/// # use luaext::context::Context;
/// # use luaext::types::function::LuaFunction;
/// # let mut state = State::new();
/// # let mut context = Context::new(&mut state);
/// context.do_string("function double(x) return x * 2 end").unwrap();
/// let lua_double: LuaFunction = context.push_global("double")
///     .get_value(&mut context).unwrap();
/// let result = lua_double.call_singleret(&mut context, &[&4])
///     .and_then(|v| v.get_value(&mut context));
/// assert_eq!(result, Some(8));
/// ```
pub struct LuaFunction {
    index: Index
}

fn get_callback_index(val: &Option<&LuaFunction>) -> Index {
    match *val {
        Some(val) => val.get_pos(),
        None => 0
    }
}

impl LuaFunction {
    /// Create a new LuaFunction given an index
    pub fn new(i: Index) -> LuaFunction {
        LuaFunction {
            index: i
        }
    }

    /// Protected function call, errors return an Err
    /// If the function call is successful, returns a list of all return values (with a maximum
    /// size nresults)
    ///
    /// # Errors
    ///
    /// Returns an Error if Lua encounters a runtime error while running this function.
    /// If an errfunc is given, then the given function will be called with the error message as
    /// an argument, and the function should return a new error message.
    /// errfunc will not be called if no error was encountered, the error that occured was an
    /// out of memeory error, or if another error occurred while running errfunc.
    pub fn pcall(&self, context: &mut Context, args: &[&ToLua], errfunc: Option<&LuaFunction>,
            nresults: i32) -> error::Result<Vec<LuaGeneric>> {
        let top_prev = context.get_state().get_top();
        context.get_state().push_value(self.get_pos());
        for arg in args {
            arg.to_lua(context.get_state());
        }
        let threadstatus = context.get_state().pcall(
            args.len() as i32, nresults, get_callback_index(&errfunc));
        match error::get_status_from_threadstatus(threadstatus) {
            Err(status) => {
                error::new_luaresult_err(status, error::pop_error_from_state(context.get_state()))
            },
            Ok(_) => {
                let top_post = context.get_state().get_top();
                error::new_luaresult_ok((top_prev..top_post)
                    .map(|i| LuaGeneric::new(i+1))
                    .collect())
            }
        }
    }

    /// Same as pcall, but returns all return values
    pub fn pcall_multiret(&self, context: &mut Context, args: &[&ToLua],
            errfunc: Option<&LuaFunction>) -> error::Result<Vec<LuaGeneric>> {
        self.pcall(context, args, errfunc, MULTRET)
    }

    /// Same as pcall, but only returns at most one return value.
    pub fn pcall_singleret(&self, context: &mut Context, args: &[&ToLua],
            errfunc: Option<&LuaFunction>) -> error::Result<Option<LuaGeneric>> {
        self.pcall(context, args, errfunc, 1)
            .map(|mut v| {
                match v.len() {
                    0 => None,
                    _ => Some(v.swap_remove(0))
                }
            })
    }

    /// Same as pcall, but returns nothing.
    pub fn pcall_noret(&self, context: &mut Context, args: &[&ToLua],
            errfunc: Option<&LuaFunction>) -> error::Result<()> {
        self.pcall(context, args, errfunc, 0)
            .map(|_|())
    }

    /// Call this function, returns a list of return values (with a maximum
    /// size nresults).
    ///
    /// # Panics
    ///
    /// This function will panic if Lua encounters a runtime error. It's not really a panic, it's
    /// actually a longjmp, but it might as well be a panic.
    pub fn call(&self, context: &mut Context, args: &[&ToLua], nresults: i32) -> Vec<LuaGeneric> {
        let top_prev = context.get_state().get_top();
        context.get_state().push_value(self.get_pos());
        for arg in args {
            arg.to_lua(context.get_state());
        }
        context.get_state().call(args.len() as i32, nresults);
        let top_post = context.get_state().get_top();
        (top_prev..top_post)
            .map(|i| LuaGeneric::new(i+1))
            .collect()
    }

    /// Same as call, but returns at most one return value.
    pub fn call_singleret(&self, context: &mut Context, args: &[&ToLua]) -> Option<LuaGeneric> {
        let mut result = self.call(context, args, 1);
        match result.len() {
            0 => None,
            _ => Some(result.swap_remove(0))
        }
    }

    /// Same as call, but returns all return values.
    pub fn call_multiret(&self, context: &mut Context, args: &[&ToLua]) -> Vec<LuaGeneric> {
        self.call(context, args, MULTRET)
    }

    /// Same as call, but does not return any return values.
    pub fn call_noret(&self, context: &mut Context, args: &[&ToLua]) {
        self.call(context, args, 0);
    }
}

impl LuaStackable for LuaFunction {
    fn get_pos(&self) -> Index {
        self.index
    }
}

impl ToLua for LuaFunction {
    fn to_lua(&self, state: &mut State) {
        state.push_value(self.get_pos());
    }
}

impl FromLua for LuaFunction {
    fn from_lua(state: &mut State, index: Index) -> Option<LuaFunction> {
        if state.is_fn(index) {
            Some(LuaFunction::new(index))
        } else {
            None
        }
    }
}
