// builtin 翻译共享骨架：load/check/store 三组同构函数由宏按字面量表一次性生成
// （逐格对账见 tests/builtin_linearop_table_matches_cpp.rs，骨架说明见
// crate::macros::builtin_linearop_impls）。

use ulua_vm::enums::lua_type::LuaType;

use crate::enums::{ir_cmd::IrCmd, ir_const_kind::IrConstKind};

crate::builtin_load_impls! {
  builtin_load_double => (IrCmd::LoadDouble); // cpp :38-44
  builtin_load_int_64 => (IrCmd::LoadInt64); // cpp :54-60
}

crate::builtin_check_impls! {
  // 常量臂比对 IrConstKind，寄存器臂 loadAndCheckTag（cpp :30-36 / :46-52）
  builtin_check_double => (IrConstKind::Double, LuaType::Number);
  builtin_check_int_64 => (IrConstKind::Int64, LuaType::Integer);
}

crate::builtin_store_impls! {
  // 比较/btest 类布尔结果尾部（cpp :431-432）
  builtin_store_bool_result => (IrCmd::StoreInt, LuaType::Boolean);
  // int64 结果尾部（cpp :1524-1525）
  builtin_store_int_64 => (IrCmd::StoreInt64, LuaType::Integer);
  // number 结果无条件打标尾部（string.len/tonumber/magnitude，cpp :921-922/:1051-1052）
  builtin_store_double_result => (IrCmd::StoreDouble, LuaType::Number);
  // type/typeof 字符串指针结果尾部（cpp :445-446）
  builtin_store_string_result => (IrCmd::StorePointer, LuaType::String);
}
