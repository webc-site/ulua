use ulua_ast::records::ast_name::AstName;
use ulua_common::enums::luau_bytecode_type::LuauBytecodeType;

/// 类型关键字单值分发：LLVM 编译为紧凑的长度预判与整数比较状态机（DFA），
/// 零哈希计算、零 UTF-8 校验扫描、零内存寻址开销。
#[inline]
pub(crate) fn get_primitive_type(name: AstName) -> LuauBytecodeType {
  if name.is_null() {
    return LuauBytecodeType::LBC_TYPE_INVALID;
  }
  match name.as_bytes() {
    b"nil" => LuauBytecodeType::LBC_TYPE_NIL,
    b"boolean" => LuauBytecodeType::LBC_TYPE_BOOLEAN,
    b"number" => LuauBytecodeType::LBC_TYPE_NUMBER,
    b"integer" => LuauBytecodeType::LBC_TYPE_INTEGER,
    b"string" => LuauBytecodeType::LBC_TYPE_STRING,
    b"thread" => LuauBytecodeType::LBC_TYPE_THREAD,
    b"buffer" => LuauBytecodeType::LBC_TYPE_BUFFER,
    b"vector" => LuauBytecodeType::LBC_TYPE_VECTOR,
    b"any" | b"unknown" => LuauBytecodeType::LBC_TYPE_ANY,
    _ => LuauBytecodeType::LBC_TYPE_INVALID,
  }
}
