//! builtin 公共骨架坍缩宏（对照 cpp `IrTranslateBuiltins.cpp` 顶部的 static 助手段）。
//!
//! 三组函数各自体内仅差字面量（cmd 名 / 常量种类 / Lua 类型标签），逐位保持语义：
//! - `builtin_load_impls!`：常量直通 + 单 op `Load*`（cpp `builtinLoadDouble` :38-44、
//!   `builtinLoadInt64` :54-60）；
//! - `builtin_check_impls!`：常量臂断言种类 + 寄存器臂按标签校验（cpp `builtinCheckDouble`
//!   :30-36、`builtinCheckInt64` :46-52；比对方式与 cpp 一致走 `constOp(arg).kind`）；
//! - `builtin_store_impls!`：结果写回 ra 并无条件补打标签（cpp 各翻译器尾部
//!   `STORE_*, vmReg(ra)` + `STORE_TAG, constTag(..)` 成对出现，如 :431-432/:1524-1525）。
//!
//! 每次调用同时生成一张 `*_ROWS` 对账表（行 token 与生成体同源，构造即一致），
//! 由 tests/builtin_linearop_table_matches_cpp.rs 与 cpp 逐格核对。

/// load 臂：常量直接返回，否则发一条单 op 装载指令。
#[macro_export]
macro_rules! builtin_load_impls {
  ($( $meth:ident => ( $cmd:expr ); )+) => {
    /// 方法名 × IR cmd 对账表（与下方生成体同源）。
    pub const BUILTIN_LOAD_ROWS: &[(&str, $crate::enums::ir_cmd::IrCmd)] =
      &[ $( (stringify!($meth), $cmd), )+ ];

    $(
      #[inline]
      pub(crate) fn $meth(
        build: &mut $crate::records::ir_builder::IrBuilder,
        arg: $crate::records::ir_op::IrOp,
      ) -> $crate::records::ir_op::IrOp {
        if arg.kind() == $crate::enums::ir_op_kind::IrOpKind::Constant {
          return arg;
        }

        build.inst_ir_cmd_ir_op($cmd, arg)
      }
    )+
  };
}

/// check 臂：常量臂 CODEGEN_ASSERT 常量种类，寄存器臂 vmExit 兜底 + 按标签校验。
#[macro_export]
macro_rules! builtin_check_impls {
  ($( $meth:ident => ( $kind:expr, $tag:expr ); )+) => {
    /// 方法名 × 期望常量种类 × Lua 标签对账表（与下方生成体同源）。
    pub const BUILTIN_CHECK_ROWS: &[(&str, $crate::enums::ir_const_kind::IrConstKind, ulua_vm::enums::lua_type::LuaType)] =
      &[ $( (stringify!($meth), $kind, $tag), )+ ];

    $(
      #[inline]
      pub(crate) fn $meth(
        build: &mut $crate::records::ir_builder::IrBuilder,
        arg: $crate::records::ir_op::IrOp,
        pcpos: i32,
      ) {
        if arg.kind() == $crate::enums::ir_op_kind::IrOpKind::Constant {
          $crate::macros::codegen_assert::CODEGEN_ASSERT!(
            build.function.const_op(arg).kind() == $kind
          );
        } else {
          let exit = build.vm_exit(pcpos as u32);
          build.load_and_check_tag(arg, $tag as u8, exit);
        }
      }
    )+
  };
}

/// store 尾：结果按 cmd 写回 ra 寄存器，并无条件补写标签。
#[macro_export]
macro_rules! builtin_store_impls {
  ($( $meth:ident => ( $cmd:expr, $tag:expr ); )+) => {
    /// 方法名 × store cmd × Lua 标签对账表（与下方生成体同源）。
    pub const BUILTIN_STORE_ROWS: &[(&str, $crate::enums::ir_cmd::IrCmd, ulua_vm::enums::lua_type::LuaType)] =
      &[ $( (stringify!($meth), $cmd, $tag), )+ ];

    $(
      #[inline]
      pub(crate) fn $meth(
        build: &mut $crate::records::ir_builder::IrBuilder,
        ra: i32,
        value: $crate::records::ir_op::IrOp,
      ) {
        let ra_reg = build.vm_reg(ra as u8);
        build.inst_ir_cmd_ir_op_ir_op($cmd, ra_reg, value);
        build.store_tag(ra_reg, $tag as u8);
      }
    )+
  };
}

pub use builtin_check_impls;
pub use builtin_load_impls;
pub use builtin_store_impls;
