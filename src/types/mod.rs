use lua::{State, Index, ToLua};

pub mod number;
pub mod string;
pub mod table;
pub mod generic;
pub mod nil;
pub mod integer;
pub mod boolean;
pub mod function;
pub mod userdata;

pub use self::number::LuaNumber;
pub use self::string::LuaString;
pub use self::table::LuaTable;
pub use self::generic::LuaGeneric;
pub use self::nil::LuaNil;
pub use self::integer::LuaInteger;
pub use self::boolean::LuaBool;
pub use self::function::LuaFunction;
pub use self::userdata::LuaUserdata;

pub trait LuaStackable {
    fn get_pos(&self) -> Index;
}

impl ToLua for LuaStackable {
    fn to_lua(&self, state: &mut State) {
        state.push_value(self.get_pos());
    }
}