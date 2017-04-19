use std::error;
use std::fmt;
use std::result;
use lua;

/// The type of error that occured
#[derive(Copy, Clone, Debug)]
pub enum LuaErrorType {
    RuntimeError,
    SyntaxError,
    MemoryError,
    GcError,
    MessageHandlerError,
    FileError,
}

/// A lua error
#[derive(Debug)]
pub struct LuaError {
    status: LuaErrorType,
    message: String
}

impl LuaError {
    /// Get the type of this error
    pub fn get_type(&self) -> LuaErrorType {
        self.status
    }

    /// Get the error message
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

/// Result from a Lua call
pub type Result<T> = result::Result<T, LuaError>;

/// Create a new Okay Lua result from a given value
pub fn new_luaresult_ok<T>(value: T) -> self::Result<T> {
    Ok(value)
}

/// Create a new Lua Error result with a type and an error
pub fn new_luaresult_err<T>(status: LuaErrorType, message: String) -> self::Result<T> {
    Err(LuaError{
        status: status,
        message: message
    })
}

/// uses a ThreadStatus to determine if Lua encountered an error or not
pub fn get_status_from_threadstatus(status: lua::ThreadStatus)
        -> result::Result<(), LuaErrorType> {
    match status {
        lua::ThreadStatus::Yield => Ok(()),
        lua::ThreadStatus::Ok    => Ok(()),
        lua::ThreadStatus::RuntimeError => Err(LuaErrorType::RuntimeError),
        lua::ThreadStatus::SyntaxError  => Err(LuaErrorType::SyntaxError),
        lua::ThreadStatus::MemoryError  => Err(LuaErrorType::MemoryError),
        lua::ThreadStatus::FileError    => Err(LuaErrorType::FileError),
        lua::ThreadStatus::GcError      => Err(LuaErrorType::GcError),
        lua::ThreadStatus::MessageHandlerError => Err(LuaErrorType::MessageHandlerError),
    }
}

/// Get the error message from a lua state (always the last value on the stack)
pub fn pop_error_from_state(state: &mut lua::State) -> String {
    let ret = state.to_str(-1).unwrap().into();
    state.pop(1);
    ret
}
