//! C++ `extern const luau_FastFunction LUAU_F_TABLE[256]` (lbuiltins.h:9).
//! Source: `VM/src/lbuiltins.cpp:2589-2781`
//!
//! 表的下标就是 `LuauBuiltinFunction`（`Common/include/Luau/Bytecode.h:573`），
//! 本仓库该枚举与上游逐项同序，故这里只按变体名装表、不写魔法数字。
use ulua_common::enums::luau_builtin_function::LuauBuiltinFunction;

use crate::{
  functions::{
    luau_f_byte::luau_f_byte,
    luau_f_extract::luau_f_extract,
    luau_f_missing::luau_f_missing,
    luau_f_modf::luau_f_modf,
    luau_f_rawequal::luau_f_rawequal,
    luau_f_readinteger::{luau_f_bufferreadlong, luau_f_readinteger},
    luau_f_vectormin::luau_f_vectormin,
    luau_f_writeinteger::luau_f_writeinteger,
  },
  type_aliases::luau_fast_function::LuauFastFunction,
};

/// 表长与上游 `LUAU_F_TABLE[256]` 一致（`pub` 供 ulua-capi 导出壳复用）。
///
/// 上游注释（lbuiltins.cpp:2762-2764）：真实项之后是 64 个 `luauF_missing`
/// dummy 槽，让不支持新 builtin 的旧 runtime 自动回退到慢路径；再往后的槽位
/// 由 C++ 零初始化成 `NULL`（调用即崩溃）。256 也正是 `LOP_FASTCALL*` 的 A
/// 字段（8 bit）全域，索引无需再判越界，故 Rust 侧把整段尾部统一成
/// `luau_f_missing` —— 上游 dummy 槽意图的安全等价形态。
pub const TABLE_LEN: usize = 256;

/// 未移植槽位的兜底项：`luau_f_missing` 恒返回 -1，VM 据此回退到慢路径。
/// 落成 fn 指针形态，既供装表复用，也让测试能比较地址（fn item 各自类型不同）。
const MISSING: LuauFastFunction = Some(luau_f_missing);

/// 已移植的 `luauF_*` 按上游 `LUAU_F_TABLE` 字面量的下标装入表，未移植的保持
/// `luau_f_missing`（返回 -1，VM 据此走 FASTCALL 之后既有的慢路径）。
pub const fn build_table() -> [LuauFastFunction; TABLE_LEN] {
  // 上游尾部的 MISSING8 展开：先全量兜底，再逐项覆盖已移植的槽位
  let mut table = [MISSING; TABLE_LEN];

  // math.modf -> (integer, fraction)
  table[LuauBuiltinFunction::LbfMathModf as usize] = Some(luau_f_modf);
  // bit32.extract
  table[LuauBuiltinFunction::LbfBit32Extract as usize] = Some(luau_f_extract);
  // string.byte
  table[LuauBuiltinFunction::LbfStringByte as usize] = Some(luau_f_byte);
  // rawequal
  table[LuauBuiltinFunction::LbfRawequal as usize] = Some(luau_f_rawequal);

  // buffer.read*/write*（小端专有，模板项按上游逐类型单态化）
  table[LuauBuiltinFunction::LbfBufferReadi8 as usize] = Some(luau_f_readinteger::<i8>);
  table[LuauBuiltinFunction::LbfBufferReadu8 as usize] = Some(luau_f_readinteger::<u8>);
  table[LuauBuiltinFunction::LbfBufferWriteu8 as usize] = Some(luau_f_writeinteger::<u8>);
  table[LuauBuiltinFunction::LbfBufferReadi16 as usize] = Some(luau_f_readinteger::<i16>);
  table[LuauBuiltinFunction::LbfBufferReadu16 as usize] = Some(luau_f_readinteger::<u16>);
  table[LuauBuiltinFunction::LbfBufferWriteu16 as usize] = Some(luau_f_writeinteger::<u16>);
  table[LuauBuiltinFunction::LbfBufferReadi32 as usize] = Some(luau_f_readinteger::<i32>);
  table[LuauBuiltinFunction::LbfBufferReadu32 as usize] = Some(luau_f_readinteger::<u32>);
  table[LuauBuiltinFunction::LbfBufferWriteu32 as usize] = Some(luau_f_writeinteger::<u32>);
  // buffer.readinteger（int64_t）；writeinteger 的 `luauF_bufferwritelong` 尚未移植
  table[LuauBuiltinFunction::LbfBufferReadinteger as usize] = Some(luau_f_bufferreadlong);

  // vector.min
  table[LuauBuiltinFunction::LbfVectorMin as usize] = Some(luau_f_vectormin);

  // LBF_NONE：上游此项为 NULL，快速调用不会用到它
  table[LuauBuiltinFunction::LbfNone as usize] = None;

  table
}

pub static LUAU_F_TABLE: [LuauFastFunction; TABLE_LEN] = build_table();
