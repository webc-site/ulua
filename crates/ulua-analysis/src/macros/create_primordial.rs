//! TypeFunction 原始类型（primordial）create_* 构造骨架单点
//! （`TypeFunctionRuntime.cpp`）。
//!
//! C++ 侧 `createAny/createNever/createUnknown/createNumber/createString/
//! createBoolean/createBuffer/createThread` 是同一形状：现场构造一枚
//! `TypeFunctionTypeVariant` 交给 `alloc_type_user_data` 压栈后固定返回 1。
//! 本仓库原先把这副骨架在 8 个模块里各手抄一遍（含逐字重复的 import 块与
//! `# Safety` 契约文本）。[`create_primordial!`] 保留全部对外路径、函数名、
//! 签名与构造语义不变，`# Safety` 契约文本单点维护在宏内，调用点只剩「cpp 出处
//! + 函数名 + 变体表达式」。

/// 生成一枚「构造指定 `TypeFunctionTypeVariant` 变体并 `alloc_type_user_data`
/// 压栈、返回 1」的 `pub unsafe fn` 入口。
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
macro_rules! create_primordial {
  ($(#[$attr:meta])* $name:ident, $variant:expr $(,)?) => {
    $(#[$attr])*
    ///
    /// # Safety
    /// `l` 必须是 Lua VM 在本次原生函数调用中传入、且在该调用全程有效的 `lua_State*`：VM 已把
    /// 实参压入栈顶，本函数只借用不持有该地址、返回前不跨调用保存；调用期间单线程独占 VM 栈与
    /// 类型运行期数据。
    pub unsafe fn $name(
      l: *mut crate::type_aliases::lua_state::LuaState,
    ) -> i32 {
      // Safety: l 是 Lua VM 调注册闭包时传入的存活 lua_State，原样转发给 alloc_type_user_data，
      // 其前置条件（活状态、栈可增长）由该 VM 调用约定满足。
      unsafe {
        crate::functions::alloc_type_user_data::alloc_type_user_data(l, $variant, false)
      };

      1
    }
  };
}

pub(crate) use create_primordial;
