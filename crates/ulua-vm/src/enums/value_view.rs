//! `TValue` 的栈上安全读视图（tag → payload 的 match 化入口）。
//!
//! §11 路线图的迁移载体：最终目标是把 C 形 `TValue`（tag + union，JIT 经
//! `offset_of!` 消费其布局）整体替换为惯用 Rust enum。本视图是 pass B1 的第一
//! 刀——零 ABI 风险：`lua_TValue` 的内存布局、栈表示与一切 C-ABI/JIT 可见结构
//! 均不变，仅把读侧的 `ttis*!(x)` if 链收敛为 `match ValueView::from_tvalue(x)`。
//! 后续 pass 再把存储本身迁入 enum，届时本视图即其原型。

use core::ffi::c_void;

use crate::{
  enums::lua_type::LuaType,
  macros::{
    classvalue::classvalue, lightuserdatatag::lightuserdatatag, lu_tag_iterator::LU_TAG_ITERATOR,
    lua_vector_size::LUA_VECTOR_SIZE, lvalue::lvalue, objectvalue::objectvalue, pvalue::pvalue,
    upvalue::upvalue,
  },
  records::{
    closure::Closure, lua_state::LuaState, lua_table::LuaTable, luau_buffer::LuauBuffer,
    luau_class::LuauClass, luau_object::LuauObject, t_string::tstring, udata::Udata, up_val::UpVal,
  },
  type_aliases::t_value::TValue,
};

/// 从 [`TValue`] 借出的只读视图：变体与本 fork 的 `LuaType` tag 一一对应，
/// payload 即各读取宏（`nvalue!`/`tsvalue!`/`hvalue!`…）的返回形状。
///
/// 寿命契约：视图经 [`ValueView::from_tvalue`] 自一个共享 `TValue` 借出，只应在
/// 消费它的 `match` 语句内存活——期间不得有 GC、栈写或对象变更穿插（与既有宏
/// 链的同址读写窗口等价）。这就是 B1 阶段"零 ABI 风险"的边界。
/// 变体载荷全部为 `Copy`（引用/裸指针/Copy 标量），派生 `Clone, Copy` 让调用点
/// 可把一个视图绑定为局部量后在 `if let` 判据链中多次复用（lvmexecute 算术族
/// 的 `(rb, rc)` 双视图分派），不改变视图的寿命契约。
#[derive(Clone, Copy)]
pub enum ValueView<'a> {
  Nil,
  /// `bvalue!` 原样 i32：`setbvalue!` 可存入任意非零 int，`l_isfalse!` 以 `== 0`
  /// 判假，故不折算为 `bool`（折算即发明新行为）。
  Boolean(i32),
  Number(f64),
  /// 本 fork 在 Luau 之上扩展的 `LUA_TINTEGER`（`lvalue!`，i64）。
  Integer(i64),
  /// 指向槽内 `value + extra` 连续 `LUA_VECTOR_SIZE` 个 f32 分量的数组视图
  /// （`vvalue!` 同形；指针跨栈重分配失效）。
  Vector(&'a [f32; LUA_VECTOR_SIZE as usize]),
  String(&'a tstring),
  Table(&'a LuaTable),
  /// tag 系统只有一个 `Function` tag，L/C 闭包共用 [`Closure`]，以 `is_c` 区分
  /// （`iscfunction!` 同款判据），故不拆成两个变体。
  Function(&'a Closure),
  /// `thvalue!` 同款裸指针：`LuaState` 在执行中被持续改写，给出 `&` 会谎称
  /// 排他/非别名（Stacked Borrows），读侧仅透传指针本身。
  Thread(*mut LuaState),
  /// `uvalue!` 同款 `*const Udata`（该宏注释明确拒绝返回引用：`marked` 由 GC
  /// 改写、`metatable` 由 `lua_setmetatable` 改写）。
  Userdata(*const Udata),
  /// `pvalue!` + `lightuserdatatag!`（extra 槽携带的 user tag）双 payload，
  /// 对应 `lua_tolightuserdatatagged` 一类需要 tag 比较的读点。内建迭代器的
  /// 进行中游标（`LU_TAG_ITERATOR` + 非零整数载荷）也落在此变体，读点仅按
  /// light userdata 原样带出指针值，与收敛前宏链行为逐位一致。
  LightUserdata {
    pointer: *mut c_void,
    tag: i32,
  },
  /// FORGPREP*/FORGLOOP 内建迭代协议的「无更多值」标记槽（`luau_execute` 与
  /// codegen 慢路径写在 ra+2）：base tag LightUserData + `LU_TAG_ITERATOR` +
  /// null 载荷。识别收口在此（构造收口在
  /// [`crate::functions::set_iterator_done`]），读侧 `match` 本变体即可，
  /// 不再逐点 tag 比较；内存布局与 tag 数值不变（JIT `offset_of!` 契约无涉）。
  IteratorDone,
  /// `bufvalue!` 同款裸指针：`LuauBuffer` 的 `len`/`capacity`/数据区均会被写。
  Buffer(*mut LuauBuffer),
  /// `classvalue!` 同款裸指针（本 fork 扩展的 `LUA_TCLASS`）：`luaV_equalval` 的
  /// `==` 分支实际读取该 payload 作对象同一性比较，故为一级变体而非逃生舱。
  Class(*const LuauClass),
  /// `objectvalue!` 同款裸指针（本 fork 扩展的 `LUA_TOBJECT`）：`luaV_equalval` 需
  /// 沿 `lclass->instancemetatable` 取比较元方法（与 metatable 同规则），是有真实
  /// 字段读取的 payload，故为一级变体。刻意保持裸指针：实例成员由 VM 原地改写。
  Object(*const LuauObject),
  /// `upvalue!` 同款 `*mut UpVal`（本 fork 与上游一致的 `LUA_TUPVAL`）：刻意保持
  /// 裸指针而非 `&`——`UpVal` 在 open/close 两态间被并发改写（`v` 指针随 close 换
  /// 指向、`u.value` 写入收口值，且对象由 GC 沿闭包/线程链表搬动），给引用会谎称
  /// 排他/非别名；GETUPVAL 读 `uv->v`、SETUPVAL 经 `uv->v` 写并触发 `lua_c_barrier`
  /// 都需要这个写出口，故与 `Userdata`/`Buffer` 同轨给裸指针（读写双轨先例：Table
  /// 变体供 `&LuaTable` 只读、写侧仍走 `hvalue!`；此处 UpVal 因 open/close 切换必有
  /// 写出口而直接给 `*mut`，读点仅透传 `uv->v` 指针本身）。
  UpVal(*mut UpVal),
  /// `Proto`/`DeadKey` 等内部 tag 的逃生舱（`Upval` 已于三波簇1升为一级变体，不再落入本舱）：
  /// 既有宏链对它们没有专用读取宏，调用点沿用 `iscollectable!`/tag 值自行分支
  /// （与 `check_exp` 下 LUAU_ASSERT 的"不可能 tag 不新增行为"原则一致）。
  Other(u32),
}

impl<'a> ValueView<'a> {
  /// 按 `t.tt()` 的 base tag 构造只读视图。未知/内部 tag 归入 [`ValueView::Other`]
  /// 并原样带出 tag 值，不在构造处 panic——读宏链对非命中 tag 的默认分支语义。
  ///
  /// 本函数为 safe：唯一 unsafe 块内的解引用全部经由既有读取宏，其共同前提——
  /// `tt` 是 VM 维护的有效 tag 且与 payload union 成员同步设置（各 `set*value!`
  /// 宏的不变量）、GC payload 指针指向存活对象——正是整个 VM 已在依赖的 tag
  /// 有效性不变量；共享引用 `t` 保证槽本身可读。
  pub fn from_tvalue(t: &'a TValue) -> Self {
    // Safety: 见上文——tag 有效性不变量由 VM 全局维护，各宏自带的 check_exp
    // （LUAU_ASSERT）在 debug 构建下继续校验指针形状。
    unsafe {
      match LuaType::from_c_int(t.tt()) {
        Some(LuaType::Nil) => Self::Nil,
        Some(LuaType::Boolean) => Self::Boolean(t.as_boolean_raw()),
        Some(LuaType::Number) => Self::Number(t.as_number()),
        Some(LuaType::Integer) => Self::Integer(lvalue!(t)),
        Some(LuaType::Vector) => Self::Vector(t.as_vector_ref()),
        Some(LuaType::String) => Self::String(t.as_string()),
        Some(LuaType::Table) => Self::Table(t.as_table()),
        Some(LuaType::Function) => Self::Function(t.as_closure()),
        Some(LuaType::Thread) => Self::Thread(t.as_thread_ptr()),
        Some(LuaType::UserData) => Self::Userdata(t.as_userdata_ptr()),
        Some(LuaType::LightUserData) => {
          let pointer = pvalue!(t);
          let tag = lightuserdatatag!(t);
          if tag == LU_TAG_ITERATOR && pointer.is_null() {
            Self::IteratorDone
          } else {
            Self::LightUserdata { pointer, tag }
          }
        }
        Some(LuaType::Buffer) => Self::Buffer(t.as_buffer_ptr()),
        Some(LuaType::Class) => Self::Class(classvalue!(t).cast_const()),
        Some(LuaType::Object) => Self::Object(objectvalue!(t).cast_const()),
        Some(LuaType::Upval) => Self::UpVal(upvalue!(t)),
        // Proto/DeadKey、None(-1) 及一切未知 tag：原样带出（Upval 已于三波簇1升为一级变体）。
        _ => Self::Other(t.tt() as u32),
      }
    }
  }
}
