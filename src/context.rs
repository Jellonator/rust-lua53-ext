use lua::{State, Index, ToLua, FromLua, Function, REGISTRYINDEX};
use types;
use std::ptr;
use error;

/// A wrapper around a Lua State.
///
/// Contains its own section of a Lua Stack; when the context goes out of scope, any value pushed
/// using this context is popped.
pub struct Context<'a> {
    state: &'a mut State,
    target_pos: Index,
}

/// __gc metamethod used to clean up Rust types that implements the Drop trait.
pub fn userdata_drop<T>(state: &mut State) -> i32 {
    use std::ptr;
    let mut context = Context::new(state);
    unsafe {
        // should be safe as long as types match
        let userdata = context.get_arg_typed::<types::LuaUserdata>(1).unwrap();
        let entity = userdata.get_value::<T>(&mut context).unwrap();
        ptr::drop_in_place(entity as *mut T);
    };
    0
}

impl<'a> Context<'a> {
    /// Creates a new Context using an existing state.
    pub fn new(state: &mut State) -> Context {
        let pos = state.get_top();
        Context {
            state: state,
            target_pos: pos,
        }
    }

    /// Get this context's contained state.
    pub fn get_state(&mut self) -> &mut State {
        self.state
    }

    /// Push a floating point number onto the stack.
    pub fn push_number(&mut self, value: f64) -> types::LuaNumber {
        self.state.push_number(value);
        let i = self.state.get_top();
        types::LuaNumber::new(i)
    }

    /// Push a string onto the stack.
    pub fn push_string(&mut self, value: &str) -> types::LuaString {
        self.state.push_string(value);
        let i = self.state.get_top();
        types::LuaString::new(i)
    }

    /// Create a new table and push it into the stack.
    pub fn push_table(&mut self) -> types::LuaTable {
        self.state.new_table();
        let i = self.state.get_top();
        types::LuaTable::new(i)
    }

    /// Push a boolean value onto the stack.
    pub fn push_bool(&mut self, value: bool) -> types::LuaBool {
        self.state.push_bool(value);
        let i = self.state.get_top();
        types::LuaBool::new(i)
    }

    /// Push a C function onto the stack.
    pub fn push_function(&mut self, func: Function) -> types::LuaFunction {
        self.state.push_fn(func);
        let i = self.state.get_top();
        types::LuaFunction::new(i)
    }

    /// Push an integer onto the stack.
    pub fn push_integer(&mut self, value: i64) -> types::LuaInteger {
        self.state.push_integer(value);
        let i = self.state.get_top();
        types::LuaInteger::new(i)
    }

    /// Push a nil value onto the stack.
    pub fn push_nil(&mut self) -> types::LuaNil {
        self.state.push_nil();
        let i = self.state.get_top();
        types::LuaNil::new(i)
    }

    /// Push a user-defined value onto the stack.
    pub fn push_userdata<T>(&mut self, value: T) -> types::LuaUserdata {
        unsafe { ptr::write(self.state.new_userdata_typed(), value); }
        let i = self.state.get_top();
        types::LuaUserdata::new(i)
    }

    /// Push a user-defined value onto the stack, and give it the metatable named 'name.'
    pub fn push_userdata_named<T>(&mut self, value: T, name: &str) -> types::LuaUserdata {
        let entity_object = self.push_userdata(value);
        let entity_object_meta = self.metatable_get(&name).unwrap();
        entity_object.set_metatable(self, &entity_object_meta);
        entity_object
    }

    /// Push a Lua thread onto the stack
    ///
    /// You will need to do stuff with the Lua thread in a separate thread if you want two Lua
    /// threads to run concurrently, Lua doesn't do any actual multithreading itself.
    pub fn push_thread(&mut self) -> types::LuaThread {
        self.state.new_thread();
        let i = self.state.get_top();
        types::LuaThread::new(i)
    }

    /// Create a library using an array of Functions, and push the library table onto the stack.,
    pub fn create_lib(&mut self, lib: &[(&str, Function)]) -> types::LuaTable {
        self.state.new_lib(lib);
        let i = self.state.get_top();
        types::LuaTable::new(i)
    }

    /// Push a global value onto the stack.
    pub fn push_global(&mut self, key: &str) -> types::LuaGeneric {
        self.state.get_global(key);
        let i = self.state.get_top();
        types::LuaGeneric::new(i)
    }

    /// Set a value in the global Lua namespace.
    pub fn set_global(&mut self, key: &str, value: &ToLua) {
        value.to_lua(self.state);
        self.state.set_global(key);
    }

    /// Get a value from the Lua registry.
    pub fn get_from_registry(&mut self, key: &ToLua) -> types::LuaGeneric {
        key.to_lua(self.state);
        self.state.get_table(REGISTRYINDEX);
        let i = self.state.get_top();
        types::LuaGeneric::new(i)
    }

    /// Get a value from the Lua registry using a type.
    pub fn get_from_registry_typed<T: FromLua>(&mut self, key: &ToLua) -> Option<T> {
        key.to_lua(self.state);
        self.state.get_table(REGISTRYINDEX);
        let i = self.state.get_top();
        T::from_lua(&mut self.state, i)
    }

    /// Set a value in the Lua registry.
    pub fn set_in_registry(&mut self, key: &ToLua, value: &ToLua) {
        key.to_lua(self.state);
        value.to_lua(self.state);
        self.state.set_table(REGISTRYINDEX);
    }

    /// Get an argument from this context.
    pub fn get_arg(&mut self, arg: Index) -> Option<types::LuaGeneric> {
        if self.state.is_none(arg) {
            None
        } else {
            Some(types::LuaGeneric::new(arg))
        }
    }

    /// Get an argument from this context.
    pub fn get_arg_typed<T: FromLua>(&mut self, arg: Index) -> Option<T> {
        T::from_lua(&mut self.state, arg)
    }

    /// Get an argument from this context.
    ///
    /// If the argument does not exist, return a default value.
    pub fn get_arg_typed_or<T: FromLua>(&mut self, arg: Index, value: T) -> Option<T> {
        if self.state.is_none_or_nil(arg) {
            Some(value)
        } else {
            T::from_lua(&mut self.state, arg)
        }
    }

    /// Execute valid Lua code.
    ///
    /// # Errors
    /// Returns an error if the string is not valid Lua,
    /// or a runtime error occurs during execution.
    pub fn do_string(&mut self, s: &str) -> error::Result<()> {
        let threadstatus = self.state.do_string(&s);
        match error::get_status_from_threadstatus(threadstatus) {
            Ok(_) => {
                error::new_luaresult_ok(())
            },
            Err(status) => {
                error::new_luaresult_err(status, error::pop_error_from_state(&mut self.state))
            }
        }
    }

    /// Push a new context on top of the current context.
    ///
    /// New values can not be pushed onto the old context until the new context goes out of scope;
    /// however, values pushed by the old context can still be used by the new context.
    pub fn push_context(&mut self) -> Context {
        Context::new(&mut self.state)
    }

    /// Returns a list of values to Lua.
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

    /// Register a new metatable in the registry.
    ///
    /// Returns a tuple; the first value is if this metatable should be initialized, and the
    /// second value is the metatable itself.
    pub fn metatable_register(&mut self, name: &str) -> (bool, types::LuaTable) {
        let ret = self.state.new_metatable(name);
        let i = self.state.get_top();
        let table = types::LuaTable::new(i);
        (ret, table)
    }

    /// Get a metatable from the registry.
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

    /// Register a named metatable into the Lua registry.
    ///
    /// Takes a list of member functions, a list of metamethods, and a unique name for this
    /// metatable.
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
