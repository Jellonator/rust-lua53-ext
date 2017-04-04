use lua::{Index, ToLua, FromLua, State, MULTRET};
use types::{LuaStackable, LuaGeneric};
use context::Context;
use error;

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
    pub fn new(i: Index) -> LuaFunction {
        LuaFunction {
            index: i
        }
    }

    // Protected function calling; error returns in Result
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
            Ok(status) => {
                let top_post = context.get_state().get_top();
                error::new_luaresult_ok(status, (top_prev..top_post)
                    .map(|i| LuaGeneric::new(i+1))
                    .collect())
            }
        }
    }

    pub fn pcall_multiret(&self, context: &mut Context, args: &[&ToLua],
            errfunc: Option<&LuaFunction>) -> error::Result<Vec<LuaGeneric>> {
        self.pcall(context, args, errfunc, MULTRET)
    }

    pub fn pcall_singleret(&self, context: &mut Context, args: &[&ToLua],
            errfunc: Option<&LuaFunction>) -> error::Result<Option<LuaGeneric>> {
        self.pcall(context, args, errfunc, 1)
            .map(|success| {
                success.map(|mut v| {
                    match v.len() {
                        0 => None,
                        _ => Some(v.swap_remove(0))
                    }
                })
            })
    }

    pub fn pcall_noret(&self, context: &mut Context, args: &[&ToLua],
            errfunc: Option<&LuaFunction>) -> error::Result<()> {
        self.pcall(context, args, errfunc, 0)
            .map(|success| {
                success.map(|_|())
            })
    }

    // Standard function calling; error results in longjmp
    // Consider making these unsafe?
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

    pub fn call_singleret(&self, context: &mut Context, args: &[&ToLua]) -> Option<LuaGeneric> {
        let mut result = self.call(context, args, 1);
        match result.len() {
            0 => None,
            _ => Some(result.swap_remove(0))
        }
    }

    pub fn call_multiret(&self, context: &mut Context, args: &[&ToLua]) -> Vec<LuaGeneric> {
        self.call(context, args, MULTRET)
    }

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
