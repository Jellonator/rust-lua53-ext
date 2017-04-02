use lua::{State, Index, ToLua, FromLua, Function, REGISTRYINDEX};
use types;
use std::ptr;

pub struct Context<'a> {
    state: &'a mut State,
    target_pos: Index,
}

impl<'a> Context<'a> {
    pub fn new(state: &mut State) -> Context {
        let pos = state.get_top();
        Context {
            state: state,
            target_pos: pos,
        }
    }

    pub fn get_state(&mut self) -> &mut State {
        self.state
    }

    // Push Values
    pub fn push_number(&mut self, value: f64) -> types::LuaNumber {
        self.state.push_number(value);
        let i = self.state.get_top();
        types::LuaNumber::new(i)
    }

    pub fn push_string(&mut self, value: &str) -> types::LuaString {
        self.state.push_string(value);
        let i = self.state.get_top();
        types::LuaString::new(i)
    }

    pub fn push_table(&mut self) -> types::LuaTable {
        self.state.new_table();
        let i = self.state.get_top();
        types::LuaTable::new(i)
    }

    pub fn push_bool(&mut self, value: bool) -> types::LuaBool {
        self.state.push_bool(value);
        let i = self.state.get_top();
        types::LuaBool::new(i)
    }

    pub fn push_function(&mut self, func: Function) -> types::LuaFunction {
        self.state.push_fn(func);
        let i = self.state.get_top();
        types::LuaFunction::new(i)
    }

    pub fn push_integer(&mut self, value: i64) -> types::LuaInteger {
        self.state.push_integer(value);
        let i = self.state.get_top();
        types::LuaInteger::new(i)
    }

    pub fn push_nil(&mut self) -> types::LuaNil {
        self.state.push_nil();
        let i = self.state.get_top();
        types::LuaNil::new(i)
    }

    pub fn push_userdata<T>(&mut self, value: T) -> types::LuaUserdata {
        unsafe { ptr::write(self.state.new_userdata_typed(), value); }
        let i = self.state.get_top();
        types::LuaUserdata::new(i)
    }

    // Global
    pub fn push_global(&mut self, key: &str) -> types::LuaGeneric {
        self.state.get_global(key);
        let i = self.state.get_top();
        types::LuaGeneric::new(i)
    }

    pub fn get_global<T: FromLua>(&mut self, key: &str) -> Option<T> {
        self.state.get_global(key);
        let top = self.state.get_top();
        let ret = self.state.to_type(top);
        self.state.pop(1);
        ret
    }

    pub fn set_global(&mut self, key: &str, value: &ToLua) {
        value.to_lua(self.state);
        self.state.set_global(key);
    }

    // Registry
    pub fn get_from_registry(&mut self, key: &ToLua) -> types::LuaGeneric {
        key.to_lua(self.state);
        self.state.get_table(REGISTRYINDEX);
        let i = self.state.get_top();
        types::LuaGeneric::new(i)
    }

    pub fn set_in_registry(&mut self, key: &ToLua, value: &ToLua) {
        key.to_lua(self.state);
        value.to_lua(self.state);
        self.state.set_table(REGISTRYINDEX);
    }

    // Execution
    pub fn do_string(&mut self, s: &str) -> Result<(), String> {
        let result = self.state.do_string(&s);
        match result.is_err() {
            false => Ok(()),
            true => {
                let error_idx = self.state.get_top();
                let ret = self.state.opt_string(error_idx, "Unknown Lua Error.").into();
                self.state.pop(1);
                Err(ret)
            }
        }
    }

    // Extra
    pub fn push_context(&mut self) -> Context {
        Context::new(&mut self.state)
    }
}

impl<'a> Drop for Context<'a> {
    fn drop(&mut self) {
        self.state.set_top(self.target_pos);
    }
}
