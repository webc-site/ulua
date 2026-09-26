use ulua_common::enums::luau_bytecode_type::LuauBytecodeType;
use ulua_vm::{enums::lua_type::LuaType, records::proto::Proto};

use crate::functions::proto_views::constants;
const LUA_TNIL: i32 = LuaType::Nil as i32;
const LUA_TBOOLEAN: i32 = LuaType::Boolean as i32;
const LUA_TLIGHTUSERDATA: i32 = LuaType::LightUserData as i32;
const LUA_TNUMBER: i32 = LuaType::Number as i32;
const LUA_TINTEGER: i32 = LuaType::Integer as i32;
const LUA_TVECTOR: i32 = LuaType::Vector as i32;
const LUA_TSTRING: i32 = LuaType::String as i32;
const LUA_TTABLE: i32 = LuaType::Table as i32;
const LUA_TFUNCTION: i32 = LuaType::Function as i32;
const LUA_TUSERDATA: i32 = LuaType::UserData as i32;
const LUA_TTHREAD: i32 = LuaType::Thread as i32;
const LUA_TBUFFER: i32 = LuaType::Buffer as i32;

const LBC_TYPE_NIL: u8 = LuauBytecodeType::LBC_TYPE_NIL.0 as u8;
const LBC_TYPE_BOOLEAN: u8 = LuauBytecodeType::LBC_TYPE_BOOLEAN.0 as u8;
const LBC_TYPE_NUMBER: u8 = LuauBytecodeType::LBC_TYPE_NUMBER.0 as u8;
const LBC_TYPE_STRING: u8 = LuauBytecodeType::LBC_TYPE_STRING.0 as u8;
const LBC_TYPE_TABLE: u8 = LuauBytecodeType::LBC_TYPE_TABLE.0 as u8;
const LBC_TYPE_FUNCTION: u8 = LuauBytecodeType::LBC_TYPE_FUNCTION.0 as u8;
const LBC_TYPE_USERDATA: u8 = LuauBytecodeType::LBC_TYPE_USERDATA.0 as u8;
const LBC_TYPE_THREAD: u8 = LuauBytecodeType::LBC_TYPE_THREAD.0 as u8;
const LBC_TYPE_VECTOR: u8 = LuauBytecodeType::LBC_TYPE_VECTOR.0 as u8;
const LBC_TYPE_BUFFER: u8 = LuauBytecodeType::LBC_TYPE_BUFFER.0 as u8;
const LBC_TYPE_INTEGER: u8 = LuauBytecodeType::LBC_TYPE_INTEGER.0 as u8;
const LBC_TYPE_ANY: u8 = LuauBytecodeType::LBC_TYPE_ANY.0 as u8;

/// 字节码常量 `k[ki]` 的 LBC 类型 tag（cpp `getBytecodeConstantTag`）。
///
/// 契约：`proto` 指向存活 Proto；`ki` 越界（字节码已验证时不可触发）按 `LBC_TYPE_ANY`
/// 处理，取安全方向而非 cpp 的越界读。
pub(crate) fn get_bytecode_constant_tag(proto: &Proto, ki: u32) -> u8 {
  // 常量表经 proto_views 的安全切片视图读出，越界即降级为 ANY（cpp 为未定义行为）。
  let Some(protok) = constants(proto).get(ki as usize) else {
    return LBC_TYPE_ANY;
  };

  // 对已读出的 TValue 标签做纯内存匹配（safe）。
  match protok.tt {
    LUA_TNIL => LBC_TYPE_NIL,
    LUA_TBOOLEAN => LBC_TYPE_BOOLEAN,
    LUA_TLIGHTUSERDATA => LBC_TYPE_USERDATA,
    LUA_TNUMBER => LBC_TYPE_NUMBER,
    LUA_TINTEGER => LBC_TYPE_INTEGER,
    LUA_TVECTOR => LBC_TYPE_VECTOR,
    LUA_TSTRING => LBC_TYPE_STRING,
    LUA_TTABLE => LBC_TYPE_TABLE,
    LUA_TFUNCTION => LBC_TYPE_FUNCTION,
    LUA_TUSERDATA => LBC_TYPE_USERDATA,
    LUA_TTHREAD => LBC_TYPE_THREAD,
    LUA_TBUFFER => LBC_TYPE_BUFFER,
    _ => LBC_TYPE_ANY,
  }
}
