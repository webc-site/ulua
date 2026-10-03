//! TypeFunction 原始类型（primordial）create_* 构造骨架单点
//! （`TypeFunctionRuntime.cpp`）。
//!
//! C++ 侧 `createAny/createNever/createUnknown/createNumber/createString/
//! createBoolean/createBuffer/createThread` 是同一形状：现场构造一枚
//! `TypeFunctionTypeVariant` 交给 `alloc_type_user_data` 压栈后固定返回 1。
//! 本仓库原先把这副骨架在 8 个模块里各手抄一遍（含逐字重复的 import 块与
//! `# Safety` 契约文本）。[`create_primordial!`] 保留全部对外路径、函数名、
//! 签名与构造语义不变，契约文本单点维护在宏内，调用点只剩「cpp 出处
//! + 函数名 + 变体表达式」。

/// 生成一枚「构造指定 `TypeFunctionTypeVariant` 变体并 `alloc_type_user_data`
/// 压栈、返回 1」的 `pub fn` 入口。r16-v42 起首参收形为独占 `&mut LuaState`
/// （本宏 8 枚调用点全数随形，不留裸指针旧臂——零消费者的臂既不参与展开检验，
/// 又属死文本，违 review.md §7），蹦床侧须同用 `c_thunk!` 的 `, @ref` 形。
///
/// 用法：
/// ```ignore
/// create_primordial!(
///   /// 对应 C++ 原生 `static int createNumber(lua_State* L)`
///   /// （`cpp/Analysis/src/TypeFunctionRuntime.cpp:498`）。
///   create_number,
///   TypeFunctionTypeVariant::Primitive(TypeFunctionPrimitiveType::new(Type::Number))
/// );
/// ```
use ulua_vm::records::lua_state::LuaState;
macro_rules! create_primordial {
  ($(#[$attr:meta])* $name:ident, $variant:expr $(,)?) => {
    $(#[$attr])*
    ///
    /// 调用序契约（正确性，非内存安全——`l` 的存活/独占前提已由 `&mut` 接收者类型承载）：
    /// `l` 须是 Lua VM 在本次原生函数调用中传入、且在该调用全程有效的状态：VM 已把实参
    /// 压入栈顶，本函数只借用不持有该地址、返回前不跨调用保存；调用期间单线程独占 VM 栈与
    /// 类型运行期数据。栈需可增 2 槽（`alloc_type_user_data` 语义），可触发分配与 GC。
    pub(crate) fn $name(
      l: &mut crate::type_aliases::lua_state::LuaState,
    ) -> i32 {
      // SAFETY: `l` 由 `c_thunk!` 的 `@ref` 蹦床在本次调用帧内从 VM 传入的存活 `lua_State*`
      // 重建而来，此处把同一独占借用原样交还 `alloc_type_user_data`（r16-v45 起该核心同收
      // `&mut`，体内裸操作收口在它一侧），本函数只剩一次不安全 fn 转调；其前置条件（活状态、
      // 栈可增长）由该 VM 调用约定满足；借用窗止于本语句。
      unsafe {
        crate::functions::alloc_type_user_data::alloc_type_user_data(&mut *l, $variant, false)
      };

      1
    }
  };
}

pub(crate) use create_primordial;
