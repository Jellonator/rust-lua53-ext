use std::error;
use std::fmt;
use std::result;
use lua;

#[derive(Copy, Clone, Debug)]
pub enum LuaErrorType {
    RuntimeError,
    SyntaxError,
    MemoryError,
    GcError,
    MessageHandlerError,
    FileError,
}

#[derive(Debug)]
pub struct LuaError {
    status: LuaErrorType,
    message: String
}

impl LuaError {
    pub fn get_status(&self) -> LuaErrorType {
        self.status
    }
    pub fn get_message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for LuaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl error::Error for LuaError {
    fn description(&self) -> &str {
        &self.message
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

#[derive(Copy, Clone)]
pub enum LuaSuccessType {
    Ok,
    Yield
}

pub struct LuaSuccess<T> {
    status: LuaSuccessType,
    ret: T
}

impl<T> LuaSuccess<T> {
    pub fn get_status(&self) -> LuaSuccessType {
        self.status
    }
    pub fn get_return(&self) -> &T {
        &self.ret
    }
    pub fn get_return_mut(&mut self) -> &mut T {
        &mut self.ret
    }
    pub fn get_return_value(self) -> T {
        self.ret
    }
    pub fn map<U, F>(self, func: F) -> LuaSuccess<U>
    where F: FnOnce(T) -> U {
        LuaSuccess {
            status: self.status,
            ret: func(self.ret)
        }
    }
}

pub type Result<T> = result::Result<LuaSuccess<T>, LuaError>;

pub fn new_luaresult_ok<T>(status: LuaSuccessType, value: T) -> self::Result<T> {
    Ok(LuaSuccess{
        status: status,
        ret: value
    })
}

pub fn new_luaresult_err<T>(status: LuaErrorType, message: String) -> self::Result<T> {
    Err(LuaError{
        status: status,
        message: message
    })
}

pub fn get_status_from_threadstatus(status: lua::ThreadStatus)
        -> result::Result<LuaSuccessType, LuaErrorType> {
    match status {
        lua::ThreadStatus::Yield => Ok(LuaSuccessType::Yield),
        lua::ThreadStatus::Ok    => Ok(LuaSuccessType::Ok),
        lua::ThreadStatus::RuntimeError => Err(LuaErrorType::RuntimeError),
        lua::ThreadStatus::SyntaxError  => Err(LuaErrorType::SyntaxError),
        lua::ThreadStatus::MemoryError  => Err(LuaErrorType::MemoryError),
        lua::ThreadStatus::FileError    => Err(LuaErrorType::FileError),
        lua::ThreadStatus::GcError      => Err(LuaErrorType::GcError),
        lua::ThreadStatus::MessageHandlerError => Err(LuaErrorType::MessageHandlerError),
    }
}

pub fn pop_error_from_state(state: &mut lua::State) -> String {
    let ret = state.to_str(-1).unwrap().into();
    state.pop(1);
    ret
}
