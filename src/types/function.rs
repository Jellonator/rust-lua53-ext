use lua::{Index, ToLua, FromLua, State, MULTRET};
use types::{LuaStackable, LuaGeneric};
use context::Context;

pub struct LuaFunction {
    index: Index
}

impl LuaFunction {
    pub fn new(i: Index) -> LuaFunction {
        LuaFunction {
            index: i
        }
    }

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

    pub fn call_multiret(&self, context: &mut Context, args: &[&ToLua]) -> Vec<LuaGeneric> {
        self.call(context, args, MULTRET)
    }
}

impl LuaStackable for LuaFunction {
    fn get_pos(&self) -> Index {
        self.index
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
