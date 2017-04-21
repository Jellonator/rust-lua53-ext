use lua::{Index};

pub mod number;
pub mod string;
pub mod table;
pub mod generic;
pub mod nil;
pub mod integer;
pub mod boolean;
pub mod function;
pub mod userdata;
pub mod ltuserdata;

pub use self::number::LuaNumber;
pub use self::string::LuaString;
pub use self::table::LuaTable;
pub use self::generic::LuaGeneric;
pub use self::nil::LuaNil;
pub use self::integer::LuaInteger;
pub use self::boolean::LuaBool;
pub use self::function::LuaFunction;
pub use self::userdata::LuaUserdata;
pub use self::ltuserdata::LuaLightUserdata;

/// Any value that can represent an Index on a Lua Stack
pub trait LuaStackable {
    /// Get the position of this value on the stack
    fn get_pos(&self) -> Index;
}
