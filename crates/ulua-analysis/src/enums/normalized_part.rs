//! `NormalizedType` 部件名单的单点。
//!
//! cpp `NormalizedType` 有 13 个 `hasTops()/hasBooleans()/…` 访问器，各 `isX()`
//! 谓词与 `types.*` 归约入口把它们逐字串成十条到十三条的判定链，同一份「排除名单」
//! 于是在 `is_exactly_number`/`is_subtype_of_string`/`is_nil`/`is_falsy` 与
//! `keyof`/`index`/`setmetatable` 里各抄一遍，且因 `hasIntegers()` 自身已按
//! `LuauIntegerType2` 门控（关旗标时恒 `false`）而额外背上一份旗标双分支。
//! 本模块把部件收成枚举 + 顺序表，名单化判定只需给出**允许**的部件。

/// 归一化视图的一个部件，对应 cpp 一个 `hasX()` 访问器。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NormalizedPart {
  Tops,
  Booleans,
  ExternTypes,
  Errors,
  Nils,
  Numbers,
  Integers,
  Strings,
  Threads,
  Buffers,
  Tables,
  Functions,
  Tyvars,
}

/// 全部部件，顺序与 cpp 判定链里 `hasX()` 的书写顺序一致（`Integers` 紧邻
/// `Numbers`）。各 `hasX()` 均无副作用，故遍历顺序不影响结果。
pub const ALL_PARTS: [NormalizedPart; 13] = [
  NormalizedPart::Tops,
  NormalizedPart::Booleans,
  NormalizedPart::ExternTypes,
  NormalizedPart::Errors,
  NormalizedPart::Nils,
  NormalizedPart::Numbers,
  NormalizedPart::Integers,
  NormalizedPart::Strings,
  NormalizedPart::Threads,
  NormalizedPart::Buffers,
  NormalizedPart::Tables,
  NormalizedPart::Functions,
  NormalizedPart::Tyvars,
];

/// `keyof`/`index` 的允许部件：仅接受归一化成 table 或 extern type（或其联合）。
/// 含 `Integers` 是因为原判定链未列 `hasIntegers()`——整数部件不参与拒绝。
pub const TABLE_OR_EXTERN_PARTS: [NormalizedPart; 3] = [
  NormalizedPart::Tables,
  NormalizedPart::ExternTypes,
  NormalizedPart::Integers,
];

/// `setmetatable` 目标的允许部件：仅接受纯 table（原判定链把 extern type 也列为
/// 拒绝项，同样不列 `hasIntegers()`）。
pub const TABLE_ONLY_PARTS: [NormalizedPart; 2] =
  [NormalizedPart::Tables, NormalizedPart::Integers];
