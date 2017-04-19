use lua::{State, Index, ToLua, FromLua, Function, REGISTRYINDEX};
use types;
use std::ptr;
use error;

pub struct Context<'a> {
    state: &'a mut State,
    target_pos: Index,
}

pub fn userdata_drop<T>(state: &mut State) -> i32 {
    use std::ptr;
    let mut context = Context::new(state);
    unsafe {
        // should be safe as long as types match
        let userdata = context.get_arg::<types::LuaUserdata>(1).unwrap();
        let entity = userdata.get_value::<T>(&mut context).unwrap();
        ptr::drop_in_place(entity as *mut T);
    };
    0
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

    pub fn push_userdata_named<T>(&mut self, value: T, name: &str) -> types::LuaUserdata {
        let entity_object = self.push_userdata(value);
        let entity_object_meta = self.metatable_get(&name).unwrap();
        entity_object.set_metatable(self, &entity_object_meta);
        entity_object
    }

    pub fn create_lib(&mut self, lib: &[(&str, Function)]) -> types::LuaTable {
        self.state.new_lib(lib);
        let i = self.state.get_top();
        types::LuaTable::new(i)
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

    pub fn get_from_registry_typed<T: FromLua>(&mut self, key: &ToLua) -> Option<T> {
        key.to_lua(self.state);
        self.state.get_table(REGISTRYINDEX);
        let i = self.state.get_top();
        T::from_lua(&mut self.state, i)
    }

    pub fn set_in_registry(&mut self, key: &ToLua, value: &ToLua) {
        key.to_lua(self.state);
        value.to_lua(self.state);
        self.state.set_table(REGISTRYINDEX);
    }

    // Arguments
    pub fn get_arg<T: FromLua>(&mut self, arg: Index) -> Option<T> {
        T::from_lua(&mut self.state, arg)
    }

    pub fn get_arg_or<T: FromLua>(&mut self, arg: Index, value: T) -> Option<T> {
        if self.state.is_none_or_nil(arg) {
            Some(value)
        } else {
            T::from_lua(&mut self.state, arg)
        }
    }

    // Execution
    pub fn do_string(&mut self, s: &str) -> error::Result<()> {
        let threadstatus = self.state.do_string(&s);
        match error::get_status_from_threadstatus(threadstatus) {
            Ok(status) => {
                error::new_luaresult_ok(status, ())
            },
            Err(status) => {
                error::new_luaresult_err(status, error::pop_error_from_state(&mut self.state))
            }
        }
    }

    // Extra
    pub fn push_context(&mut self) -> Context {
        Context::new(&mut self.state)
    }

    pub fn return_context(mut self, args: &[&ToLua]) -> Index {
        // push elements in reverse order
        for arg in args.iter().rev() {
            arg.to_lua(&mut self.state);
        }
        for i in 0..args.len() {
            self.state.replace(i as Index + self.target_pos);
        }
        self.target_pos += args.len() as Index;
        args.len() as Index
    }

    // metatable
    pub fn metatable_register(&mut self, name: &str) -> (bool, types::LuaTable) {
        let ret = self.state.new_metatable(name);
        let i = self.state.get_top();
        let table = types::LuaTable::new(i);
        (ret, table)
    }

    pub fn metatable_get(&mut self, name: &str) -> Option<types::LuaTable> {
        self.state.get_metatable_from_registry(name);
        match self.state.is_table(-1) {
            true => {
                let i = self.state.get_top();
                let table = types::LuaTable::new(i);
                Some(table)
            },
            false => {
                self.state.pop(1);
                None
            }
        }
    }

    pub fn metatable_register_named<T>(&mut self, lib: &[(&str, Function)], metamethods: &[(&str, Function)], unique_name: &str)
            -> Option<types::LuaTable> {
        let entity_library_members = self.create_lib(lib);
        let (should_set, entity_metatable) = self.metatable_register(unique_name);
        if should_set {
            for &(ref name, ref value) in metamethods.iter() {
                entity_metatable.set_raw(self, name, value);
            }
            entity_metatable.set_raw(self, &"__index", &entity_library_members);
            entity_metatable.set_raw(self, &"__gc", &lua_func!(userdata_drop<T>));
            // disable getting of metatables
            entity_metatable.set_raw(self, &"__metatable", &0);
        }
        Some(entity_metatable)
    }
}

impl<'a> Drop for Context<'a> {
    fn drop(&mut self) {
        self.state.set_top(self.target_pos);
    }
}
