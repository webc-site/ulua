//! 本文件由 `crates/ulua-capi/tools/gen_capi.py` 自动生成（源：ulua-vm/src/functions/lua_v_doarithimpl.rs）。
//! 每个导出壳与 ulua-vm 对应函数签名一致，仅做逐参数透传，零逻辑。
//! 8 个算术 TM 变体导出（签名与逐参数契约完全同形，仅变体名不同）由宏
//! `arith_tm_exports!` 单模板生成，与 vm 侧 `tm_exports!` 先例同构；导出符号与
//! 函数签名与逐壳手写时逐字节一致。
use ulua_vm::{
  functions::lua_v_doarithimpl,
  records::lua_state::lua_State,
  type_aliases::{stk_id::StkId, t_value::TValue},
};

/// 生成一个算术 TM 导出壳：`ulua_luaV_doarithimpl_<变体>` 透传至 ulua-vm 同名实现。
/// 第三参数为可选的 rb/rc 契约补注（一元 TmUnm 用）。
macro_rules! arith_tm_exports {
  ($( ( $variant:ident, $name:ident $(, $unary_note:literal)? ) ),+ $(,)?) => {
    $(
      #[doc = concat!(
        "# Safety\n",
        "C ABI 导出壳（符号 `ulua_luaV_doarithimpl_", stringify!($variant), "`），仅逐参数透传至 `lua_v_doarithimpl::", stringify!($name), "(l, ra, rb, rc)`，零逻辑，本帧不解引用任何指针。调用方须保证：\n",
        "- `l`：指向由本 VM 创建的合法 `lua_State`，非空、对齐，整个调用期间存活，且与对该状态的其它访问单线程驱动（不得跨 OS 线程并发）；\n",
        "- `ra`（`StkId`）：可写栈槽指针，指向当前帧栈界内预留的结果槽，调用期间不迁移（运算结果写入该槽）；\n",
        "- `rb`/`rc`（`*const TValue`）：指向合法、地址稳定（栈槽或被 GC 持有）的 `TValue` 操作数，调用期间只读存活",
        $( $unary_note, )?
        "；\n",
        "- TM 调用协议（结果槽、栈余量、受保护帧）与被调函数的 `# Safety` 契约一致。"
      )]
      #[unsafe(export_name = concat!("ulua_luaV_doarithimpl_", stringify!($variant)))]
      pub unsafe extern "C-unwind" fn $name(
        l: *mut lua_State,
        ra: StkId,
        rb: *const TValue,
        rc: *const TValue,
      ) {
        // Safety: 宏按 TM 变体生成 C ABI 导出壳，由 C 宿主按 Lua/C API 约定调用：l 为有效 lua_State*，ra 为可写结果槽指针，rb/rc 为只读操作数 TValue 指针，调用期间存活。本壳不解引用任何指针、不在本帧重建引用，仅原样转调 ulua-vm 同名实现，故不存在别名/悬挂窗口；参数合法性前提即该实现 /// # Safety 所列契约。
        unsafe { lua_v_doarithimpl::$name(l, ra, rb, rc) }
      }
    )+
  };
}

arith_tm_exports! {
  (TmAdd, lua_v_doarithimpl_tm_add),
  (TmDiv, lua_v_doarithimpl_tm_div),
  (TmIDiv, lua_v_doarithimpl_tm_idiv),
  (TmMod, lua_v_doarithimpl_tm_mod),
  (TmMul, lua_v_doarithimpl_tm_mul),
  (TmPow, lua_v_doarithimpl_tm_pow),
  (TmSub, lua_v_doarithimpl_tm_sub),
  (
    TmUnm,
    lua_v_doarithimpl_tm_unm,
    "（一元取负仅读 `rb`，`rc` 按协议仍须为合法 TValue）"
  ),
}
