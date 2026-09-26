//! 内置函数 fastcall id 识别表。
//!
//! 行为 oracle：`cpp/Compiler/src/Builtins.cpp:70-387` 的 `getBuiltinFunctionId`
//! （cpp 侧历史版本亦称 `getBuiltins`/`BuiltinFndfns` 一族的识别段）。cpp 用一列
//! `if (builtin.object == "…") if (builtin.method == "…")` 线性 strcmp 扫描，
//! 每次调用最坏 O(153) 次串比较；本端口把同一识别集合收口为三张编译期预排序的
//! 静态表（`TABLE` 96 项、`INTEGER_TABLE` 37 项、`BUFFER_FASTCALL_TABLE` 2 项，
//! 共 135 项，与 cpp 逐条对齐），运行期按 (库名, 方法名) 字节字典序二分，
//! 复杂度降为 O(log n)。表序由 `const` 自检在编译期锁定，失序即编译失败。

use core::cmp::Ordering;

use ulua_common::{enums::luau_builtin_function::*, fflag};

use crate::records::{builtin::Builtin, compile_options::CompileOptions};

/// 表项：`(库名, 方法名, fastcall id)`，均为不含 NUL 的字节串；空串库名表示
/// 全局函数（cpp `builtin.isGlobal(...)`，对应 `object` 为空名）。
pub type BuiltinEntry = (&'static [u8], &'static [u8], LuauBuiltinFunction);

/// 无条件识别段：不依赖任何 FastFlag。分组序即表内字节字典序：
/// 全局(:72-104) → bit32(:176-208) → buffer(:230-257) → math(:106-174) →
/// string(:210-220) → table(:222-228) → vector(:264-292)。
/// `writei8/writeu8` 等写别名归并到 `WRITEU8/U16/U32` 与 cpp :236/:242/:248 的
/// `||` 分支一致；同法名可映射同一 id，但 (库, 法) 键在全表唯一。
pub const TABLE: &[BuiltinEntry] = &[
  // 全局函数（cpp:72-104）
  (b"", b"assert", LBF_ASSERT),
  (b"", b"getmetatable", LBF_GETMETATABLE),
  (b"", b"rawequal", LBF_RAWEQUAL),
  (b"", b"rawget", LBF_RAWGET),
  (b"", b"rawlen", LBF_RAWLEN),
  (b"", b"rawset", LBF_RAWSET),
  (b"", b"select", LBF_SELECT_VARARG),
  (b"", b"setmetatable", LBF_SETMETATABLE),
  (b"", b"tonumber", LBF_TONUMBER),
  (b"", b"tostring", LBF_TOSTRING),
  (b"", b"type", LBF_TYPE),
  (b"", b"typeof", LBF_TYPEOF),
  (b"", b"unpack", LBF_TABLE_UNPACK),
  // bit32（cpp:176-208）
  (b"bit32", b"arshift", LBF_BIT32_ARSHIFT),
  (b"bit32", b"band", LBF_BIT32_BAND),
  (b"bit32", b"bnot", LBF_BIT32_BNOT),
  (b"bit32", b"bor", LBF_BIT32_BOR),
  (b"bit32", b"btest", LBF_BIT32_BTEST),
  (b"bit32", b"bxor", LBF_BIT32_BXOR),
  (b"bit32", b"byteswap", LBF_BIT32_BYTESWAP),
  (b"bit32", b"countlz", LBF_BIT32_COUNTLZ),
  (b"bit32", b"countrz", LBF_BIT32_COUNTRZ),
  (b"bit32", b"extract", LBF_BIT32_EXTRACT),
  (b"bit32", b"lrotate", LBF_BIT32_LROTATE),
  (b"bit32", b"lshift", LBF_BIT32_LSHIFT),
  (b"bit32", b"replace", LBF_BIT32_REPLACE),
  (b"bit32", b"rrotate", LBF_BIT32_RROTATE),
  (b"bit32", b"rshift", LBF_BIT32_RSHIFT),
  // buffer（cpp:230-257）
  (b"buffer", b"readf32", LBF_BUFFER_READF32),
  (b"buffer", b"readf64", LBF_BUFFER_READF64),
  (b"buffer", b"readi16", LBF_BUFFER_READI16),
  (b"buffer", b"readi32", LBF_BUFFER_READI32),
  (b"buffer", b"readi8", LBF_BUFFER_READI8),
  (b"buffer", b"readu16", LBF_BUFFER_READU16),
  (b"buffer", b"readu32", LBF_BUFFER_READU32),
  (b"buffer", b"readu8", LBF_BUFFER_READU8),
  (b"buffer", b"writef32", LBF_BUFFER_WRITEF32),
  (b"buffer", b"writef64", LBF_BUFFER_WRITEF64),
  (b"buffer", b"writei16", LBF_BUFFER_WRITEU16),
  (b"buffer", b"writei32", LBF_BUFFER_WRITEU32),
  (b"buffer", b"writei8", LBF_BUFFER_WRITEU8),
  (b"buffer", b"writeu16", LBF_BUFFER_WRITEU16),
  (b"buffer", b"writeu32", LBF_BUFFER_WRITEU32),
  (b"buffer", b"writeu8", LBF_BUFFER_WRITEU8),
  // math（cpp:106-174）
  (b"math", b"abs", LBF_MATH_ABS),
  (b"math", b"acos", LBF_MATH_ACOS),
  (b"math", b"asin", LBF_MATH_ASIN),
  (b"math", b"atan", LBF_MATH_ATAN),
  (b"math", b"atan2", LBF_MATH_ATAN2),
  (b"math", b"ceil", LBF_MATH_CEIL),
  (b"math", b"clamp", LBF_MATH_CLAMP),
  (b"math", b"cos", LBF_MATH_COS),
  (b"math", b"cosh", LBF_MATH_COSH),
  (b"math", b"deg", LBF_MATH_DEG),
  (b"math", b"exp", LBF_MATH_EXP),
  (b"math", b"floor", LBF_MATH_FLOOR),
  (b"math", b"fmod", LBF_MATH_FMOD),
  (b"math", b"frexp", LBF_MATH_FREXP),
  (b"math", b"isfinite", LBF_MATH_ISFINITE),
  (b"math", b"isinf", LBF_MATH_ISINF),
  (b"math", b"isnan", LBF_MATH_ISNAN),
  (b"math", b"ldexp", LBF_MATH_LDEXP),
  (b"math", b"lerp", LBF_MATH_LERP),
  (b"math", b"log", LBF_MATH_LOG),
  (b"math", b"log10", LBF_MATH_LOG10),
  (b"math", b"max", LBF_MATH_MAX),
  (b"math", b"min", LBF_MATH_MIN),
  (b"math", b"modf", LBF_MATH_MODF),
  (b"math", b"pow", LBF_MATH_POW),
  (b"math", b"rad", LBF_MATH_RAD),
  (b"math", b"round", LBF_MATH_ROUND),
  (b"math", b"sign", LBF_MATH_SIGN),
  (b"math", b"sin", LBF_MATH_SIN),
  (b"math", b"sinh", LBF_MATH_SINH),
  (b"math", b"sqrt", LBF_MATH_SQRT),
  (b"math", b"tan", LBF_MATH_TAN),
  (b"math", b"tanh", LBF_MATH_TANH),
  // string（cpp:210-220）
  (b"string", b"byte", LBF_STRING_BYTE),
  (b"string", b"char", LBF_STRING_CHAR),
  (b"string", b"len", LBF_STRING_LEN),
  (b"string", b"sub", LBF_STRING_SUB),
  // table（cpp:222-228）
  (b"table", b"insert", LBF_TABLE_INSERT),
  (b"table", b"unpack", LBF_TABLE_UNPACK),
  // vector（cpp:264-292）
  (b"vector", b"abs", LBF_VECTOR_ABS),
  (b"vector", b"ceil", LBF_VECTOR_CEIL),
  (b"vector", b"clamp", LBF_VECTOR_CLAMP),
  (b"vector", b"create", LBF_VECTOR),
  (b"vector", b"cross", LBF_VECTOR_CROSS),
  (b"vector", b"dot", LBF_VECTOR_DOT),
  (b"vector", b"floor", LBF_VECTOR_FLOOR),
  (b"vector", b"lerp", LBF_VECTOR_LERP),
  (b"vector", b"magnitude", LBF_VECTOR_MAGNITUDE),
  (b"vector", b"max", LBF_VECTOR_MAX),
  (b"vector", b"min", LBF_VECTOR_MIN),
  (b"vector", b"normalize", LBF_VECTOR_NORMALIZE),
  (b"vector", b"sign", LBF_VECTOR_SIGN),
];

/// `integer.*` 段（cpp:294-370）：仅当 `LuauIntegerFastcalls` 开启才参与识别
/// （cpp:294 的 `FFlag::LuauIntegerFastcalls &&` 前置）。
pub const INTEGER_TABLE: &[BuiltinEntry] = &[
  (b"integer", b"add", LBF_INTEGER_ADD),
  (b"integer", b"arshift", LBF_INTEGER_ARSHIFT),
  (b"integer", b"band", LBF_INTEGER_BAND),
  (b"integer", b"bnot", LBF_INTEGER_BNOT),
  (b"integer", b"bor", LBF_INTEGER_BOR),
  (b"integer", b"bswap", LBF_INTEGER_BSWAP),
  (b"integer", b"btest", LBF_INTEGER_BTEST),
  (b"integer", b"bxor", LBF_INTEGER_BXOR),
  (b"integer", b"clamp", LBF_INTEGER_CLAMP),
  (b"integer", b"countlz", LBF_INTEGER_COUNTLZ),
  (b"integer", b"countrz", LBF_INTEGER_COUNTRZ),
  (b"integer", b"create", LBF_INTEGER_CREATE),
  (b"integer", b"div", LBF_INTEGER_DIV),
  (b"integer", b"extract", LBF_INTEGER_EXTRACT),
  (b"integer", b"ge", LBF_INTEGER_GE),
  (b"integer", b"gt", LBF_INTEGER_GT),
  (b"integer", b"idiv", LBF_INTEGER_IDIV),
  (b"integer", b"le", LBF_INTEGER_LE),
  (b"integer", b"lrotate", LBF_INTEGER_LROTATE),
  (b"integer", b"lshift", LBF_INTEGER_LSHIFT),
  (b"integer", b"lt", LBF_INTEGER_LT),
  (b"integer", b"max", LBF_INTEGER_MAX),
  (b"integer", b"min", LBF_INTEGER_MIN),
  (b"integer", b"mod", LBF_INTEGER_MOD),
  (b"integer", b"mul", LBF_INTEGER_MUL),
  (b"integer", b"neg", LBF_INTEGER_NEG),
  (b"integer", b"rem", LBF_INTEGER_REM),
  (b"integer", b"rrotate", LBF_INTEGER_RROTATE),
  (b"integer", b"rshift", LBF_INTEGER_RSHIFT),
  (b"integer", b"sub", LBF_INTEGER_SUB),
  (b"integer", b"tonumber", LBF_INTEGER_TONUMBER),
  (b"integer", b"udiv", LBF_INTEGER_UDIV),
  (b"integer", b"uge", LBF_INTEGER_UGE),
  (b"integer", b"ugt", LBF_INTEGER_UGT),
  (b"integer", b"ule", LBF_INTEGER_ULE),
  (b"integer", b"ult", LBF_INTEGER_ULT),
  (b"integer", b"urem", LBF_INTEGER_UREM),
];

/// `buffer.readinteger/writeinteger` 段（cpp:258-261）：需 `LuauIntegerFastcalls`
/// 与 `LuauIntegerBufferFastcalls` 双旗标同时开启。
pub const BUFFER_FASTCALL_TABLE: &[BuiltinEntry] = &[
  (b"buffer", b"readinteger", LBF_BUFFER_READINTEGER),
  (b"buffer", b"writeinteger", LBF_BUFFER_WRITEINTEGER),
];

/// 编译期字节串字典序比较：`<[u8]>::cmp`（`Ord`）在 const 上下文不可用，
/// 手写等价的无符号逐字节 + 长度尾比较，仅供表自检使用。
const fn cmp_bytes(a: &[u8], b: &[u8]) -> Ordering {
  let mut i = 0;
  while i < a.len() && i < b.len() {
    if a[i] != b[i] {
      return if a[i] < b[i] {
        Ordering::Less
      } else {
        Ordering::Greater
      };
    }
    i += 1;
  }
  if a.len() == b.len() {
    Ordering::Equal
  } else if a.len() < b.len() {
    Ordering::Less
  } else {
    Ordering::Greater
  }
}

/// 表合法性自检：按 (库名, 方法名) 严格升序（同时排除重复键），且不含
/// `LbfNone` 哨兵。二分查找对失序表会静默漏判，故把序约束钉死在编译期。
const fn is_valid_table(table: &[BuiltinEntry]) -> bool {
  let mut i = 0;
  while i < table.len() {
    if matches!(table[i].2, LuauBuiltinFunction::LbfNone) {
      return false;
    }
    if i > 0 {
      let ordered = match cmp_bytes(table[i - 1].0, table[i].0) {
        Ordering::Less => true,
        Ordering::Equal => matches!(cmp_bytes(table[i - 1].1, table[i].1), Ordering::Less),
        Ordering::Greater => false,
      };
      if !ordered {
        return false;
      }
    }
    i += 1;
  }
  true
}

const _: () = assert!(is_valid_table(TABLE));
const _: () = assert!(is_valid_table(INTEGER_TABLE));
const _: () = assert!(is_valid_table(BUFFER_FASTCALL_TABLE));

/// 在预排序表里按 (库名, 方法名) 键二分查找。
fn find(table: &[BuiltinEntry], object: &[u8], method: &[u8]) -> Option<LuauBuiltinFunction> {
  table
    .binary_search_by(|&(entry_object, entry_method, _)| {
      (entry_object, entry_method).cmp(&(object, method))
    })
    .ok()
    .map(|index| table[index].2)
}

/// 把内置调用 `(库名, 方法名)` 识别为 fastcall id；非内置返回 `-1`。
/// 识别集合与返回 id 与 cpp `getBuiltinFunctionId` 逐条一致（出处见各表注释）。
pub fn get_builtin_function_id(builtin: &Builtin, options: &CompileOptions) -> i32 {
  if builtin.method.is_null() {
    return -1;
  }
  let method = builtin.method.as_bytes();
  // cpp `isGlobal` 以 `object == AstName()`（空名/零值）为全局态；本端口沿袭
  // 历史收口：null object 归一为空串键，与表内 `(b"", ...)` 臂直接二分。
  let object = if builtin.object.is_null() {
    b""
  } else {
    builtin.object.as_bytes()
  };

  let integer_fastcalls = fflag::LuauIntegerFastcalls.get();
  let id = find(TABLE, object, method)
    .or_else(|| {
      integer_fastcalls
        .then(|| find(INTEGER_TABLE, object, method))
        .flatten()
    })
    .or_else(|| {
      (integer_fastcalls && fflag::LuauIntegerBufferFastcalls.get())
        .then(|| find(BUFFER_FASTCALL_TABLE, object, method))
        .flatten()
    });

  if let Some(id) = id {
    return id as i32;
  }

  if let Some(vector_ctor) = options.vector_ctor_bytes() {
    if let Some(vector_lib) = options.vector_lib_bytes() {
      if builtin.is_method(vector_lib, vector_ctor) {
        return LBF_VECTOR as i32;
      }
    } else if builtin.is_global(vector_ctor) {
      return LBF_VECTOR as i32;
    }
  }

  -1
}
