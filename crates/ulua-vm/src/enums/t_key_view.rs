//! `TKey` 的栈上安全读视图（table 节点键 tag → payload 的 match 化入口）。
//!
//! §11 路线图的迁移载体：与 pass B1 的 [`ValueView`](crate::enums::value_view::ValueView)
//! 同形、同"零 ABI 风险"边界——`TKey` 的内存布局（`tt`/`next` 4+28 位打包字，JIT 经
//! `offset_of!` 消费）与一切写入路径（`set_tt`/`set_next`/`setttype!`/`getnodekey!`）
//! 均不变，仅把读侧对节点键的 `ttis*!(gkey!(n))`/`ttype!(gkey!(n)) == LUA_TDEADKEY`
//! if 链收敛为 `match TKeyView::from_tkey(..)`。与 B1 的关键差异：
//! [`LuaType::DeadKey`](crate::enums::lua_type::LuaType::DeadKey)（`removeentry` 把被摘除
//! 节点的 collectable 键就地改写 tag 产生的特殊 tt）在本视图里是一等变体
//! [`TKeyView::DeadKey`]，调用点不再手写魔法数字 tag 比较。后续 pass 把 `TValue`/`TKey`
//! 的存储本身迁入 enum 时，本视图与 `ValueView` 即其共同原型。

use core::ffi::c_void;

use crate::{
  enums::lua_type::LuaType,
  macros::{
    gcvalue::gcvalue, lightuserdatatag::lightuserdatatag, lua_vector_size::LUA_VECTOR_SIZE,
    lvalue::lvalue, pvalue::pvalue,
  },
  records::{
    closure::Closure, gc_object::GCObject, lua_state::LuaState, lua_table::LuaTable,
    luau_buffer::LuauBuffer, t_key::TKey, t_string::tstring, udata::Udata,
  },
};

/// 从 [`TKey`] 借出的只读视图：变体与本 fork 的 `LuaType` tag 一一对应，
/// payload 即各读取宏（`nvalue!`/`tsvalue!`/`gcvalue!`…）作用于 `TKey` 时的返回形状
/// （读取宏经 trait/字段访问对 `TValue`/`TKey` 同形复用）。
///
/// 寿命契约：与 [`ValueView`](crate::enums::value_view::ValueView) 相同——视图借自一个
/// 共享 `TKey`，只应在消费它的 `match` 语句内存活，期间不得有 GC、重哈希或节点写入
/// 穿插（与既有宏链的同址读写窗口等价）。这就是 B2a 阶段"零 ABI 风险"的边界。
pub enum TKeyView<'a> {
  /// 空槽标记（`luaH_clear`/`setnodevector` 复位后的键、dummynode 键）。
  Nil,
  /// `bvalue!` 原样 i32：与 [`ValueView::Boolean`] 同款权宜——不折算为 `bool`。
  Boolean(i32),
  Number(f64),
  /// 本 fork 在 Luau 之上扩展的 `LUA_TINTEGER`（`lvalue!`，i64）。
  Integer(i64),
  /// `vvalue!` 同款数组视图：键为 vector 时按 `value + extra` 连续
  /// `LUA_VECTOR_SIZE` 个 f32 分量读取（本 fork LUA_VECTOR_SIZE = 3）。
  Vector(&'a [f32; LUA_VECTOR_SIZE as usize]),
  String(&'a tstring),
  Table(&'a LuaTable),
  /// tag 系统只有一个 `Function` tag，L/C 闭包共用 [`Closure`]，以 `is_c` 区分
  /// （同 [`ValueView::Function`]，不拆成两个变体）。
  Function(&'a Closure),
  /// `thvalue!` 同款裸指针（同 [`ValueView::Thread`] 的 Stacked Borrows 权宜）。
  Thread(*mut LuaState),
  /// `uvalue!` 同款 `*const Udata`（同 [`ValueView::Userdata`]）。
  Userdata(*const Udata),
  /// `pvalue!` + `lightuserdatatag!`（`extra[0]`）双 payload，对应
  /// `luaH_getp` 一类需要 pointer+tag 两段比较的读点。
  LightUserdata {
    pointer: *mut c_void,
    tag: i32,
  },
  /// `bufvalue!` 同款裸指针（同 [`ValueView::Buffer`]）。
  Buffer(*mut LuauBuffer),
  /// `LUA_TDEADKEY`（tt=14）：`removeentry` 只改写 tag、不清 payload，槽内
  /// `value.gc` 仍存原 collectable 键的死指针——这正是 `luaH_next`/`findindex`
  /// 允许"已死键仍可比较"的 C++ 语义。payload 即 `gcvalue!` 形状的裸指针，
  /// **只做指针比较、绝不解引用**（对象可能已回收）。
  DeadKey(*mut GCObject),
  /// `Class`/`Object`/`Proto` 等内部或扩展 tag 的逃生舱（同 [`ValueView::Other`]）：
  /// 既有宏链对它们没有专用读取宏，调用点沿用 `iscollectable!`/tag 值自行分支。
  /// 注意 `TKey::tt()` 是 4 位字段，`Upval`(16)/`None`(-1) 在本轴不可达。
  Other(u32),
}

impl<'a> TKeyView<'a> {
  /// 按 `k.tt()` 的 4 位 base tag 构造只读视图。`Class`/`Object`/`Proto` 等
  /// 无专用读取宏的 tag 归入 [`TKeyView::Other`] 并原样带出 tag 值，不在构造处
  /// panic——读宏链对非命中 tag 的默认分支语义。
  ///
  /// 本函数为 safe：唯一 unsafe 块内的解引用全部经由既有读取宏，其共同前提——
  /// `tt` 是 VM 维护的有效 tag 且与 payload union 成员同步设置（各 `set_tt`/
  /// `set*value!` 路径的不变量）、GC payload 指针指向存活对象（[`TKeyView::DeadKey`]
  /// 例外：其 payload 按 C++ 语义本就允许是死指针，视图只透传、不解引用）——
  /// 正是整个 VM 已在依赖的 tag 有效性不变量；共享引用 `k` 保证槽本身可读。
  pub fn from_tkey(k: &'a TKey) -> Self {
    // Safety: 见上文——tag 有效性不变量由 VM 全局维护，各宏自带的 check_exp
    // （LUAU_ASSERT）在 debug 构建下继续校验指针形状。
    unsafe {
      match LuaType::from_c_int(k.tt()) {
        Some(LuaType::Nil) => Self::Nil,
        Some(LuaType::Boolean) => Self::Boolean(k.as_boolean_raw()),
        Some(LuaType::Number) => Self::Number(k.as_number()),
        Some(LuaType::Integer) => Self::Integer(lvalue!(k)),
        Some(LuaType::Vector) => Self::Vector(k.as_vector_ref()),
        Some(LuaType::String) => Self::String(k.as_string()),
        Some(LuaType::Table) => Self::Table(k.as_table()),
        Some(LuaType::Function) => Self::Function(k.as_closure()),
        Some(LuaType::Thread) => Self::Thread(k.as_thread_ptr()),
        Some(LuaType::UserData) => Self::Userdata(k.as_userdata_ptr()),
        Some(LuaType::LightUserData) => Self::LightUserdata {
          pointer: pvalue!(k),
          tag: lightuserdatatag!(k),
        },
        Some(LuaType::Buffer) => Self::Buffer(k.as_buffer_ptr()),
        // 死键：透传 value.gc（不解引用），供 findindex 一类"死键比较"读点使用。
        Some(LuaType::DeadKey) => Self::DeadKey(gcvalue!(k)),
        // Class/Object/Proto 及一切未知 tag：原样带出。
        _ => Self::Other(k.tt() as u32),
      }
    }
  }
}
