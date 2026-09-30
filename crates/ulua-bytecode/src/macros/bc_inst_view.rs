//! `bc_inst_view!`：BcInstHelper 视图记录的骨架单源宏。
//!
//! 背景：`records/` 下 10 个指令视图结构体（`BcCall`/`BcMove`/`BcJump`/……）
//! 逐字段同形——同一 `base: BcInstHelper` 字段、同一 `create`/`from` 构造样板、
//! 同一 `op`/`append_to`/`prepend_to` 委托、同一 `BcInstHelperCreate` 常量 impl，
//! 各文件只差类型名与 `LuauOpcode` 变体（cpp `BytecodeOps.h` 每个 `BcX` 结构体
//! 的对应形态）。本宏把骨架收敛为单源，各文件只保留语义各异的域访问器。
//!
//! 能力表（canonical 顺序无关，逐项独立生成一个 impl 块）：
//! - `create`：`pub fn create(graph)`——新增本 opcode 指令并包成视图；
//! - `from`：`pub fn from(graph, inst)`——包装既有指令句柄；
//! - `op`：`pub fn op()`——回指本指令的 `BcOp`；
//! - `append_to` / `prepend_to`：`pub(crate)` 块挂载委托。

/// 单类型骨架生成：`bc_inst_view!(pub(crate) BcMove = LOP_MOVE, create, op, append_to);`
macro_rules! bc_inst_view {
  ($vis:vis $name:ident = $opcode:ident $(, $cap:ident)*) => {
    #[derive(Debug)]
    $vis struct $name<'a, 'f> {
      pub(crate) base: $crate::records::bc_inst_helper::BcInstHelper<'a, 'f>,
    }

    impl $crate::records::bc_inst_helper::BcInstHelperCreate for $name<'_, '_> {
      const OPCODE: ulua_common::enums::luau_opcode::LuauOpcode =
        ulua_common::enums::luau_opcode::LuauOpcode::$opcode;
    }

    $(bc_inst_view!(@cap $name, $cap);)*
  };

  (@cap $name:ident, create) => {
    impl<'a, 'f> $name<'a, 'f> {
      /// cpp `BcX::create(graph)`：新增一条本视图 opcode 的指令并持有图的唯一可变借用。
      pub fn create(graph: &'a mut $crate::records::bc_function::BcFunction<'f>) -> Self {
        Self {
          base: $crate::records::bc_inst_helper::BcInstHelper::create::<Self>(graph),
        }
      }
    }
  };

  (@cap $name:ident, from) => {
    impl<'a, 'f> $name<'a, 'f> {
      /// cpp `BcX(graph, inst)`：持有图的唯一可变借用 + 指令 `BcOp` 句柄
      ///（旧的 `*mut BcFunction` + `BcRef` 双借用构造会构成别名冲突，属 UB）。
      pub fn from(
        graph: &'a mut $crate::records::bc_function::BcFunction<'f>,
        inst: $crate::records::bc_op::BcOp,
      ) -> Self {
        Self {
          base: $crate::records::bc_inst_helper::BcInstHelper::new(graph, inst),
        }
      }
    }
  };

  (@cap $name:ident, op) => {
    impl<'a, 'f> $name<'a, 'f> {
      /// cpp `BcX::op()`：本视图所指指令的 `BcOp` 句柄。
      pub fn op(&self) -> $crate::records::bc_op::BcOp {
        self.base.op()
      }
    }
  };

  (@cap $name:ident, append_to) => {
    impl<'a, 'f> $name<'a, 'f> {
      /// cpp `BcX::appendTo(block)`：挂到目标块尾。
      pub(crate) fn append_to(&mut self, block: $crate::records::bc_op::BcOp) {
        self.base.append_to(block);
      }
    }
  };

  (@cap $name:ident, prepend_to) => {
    impl<'a, 'f> $name<'a, 'f> {
      /// cpp `BcX::prependTo(block)`：插到目标块首。
      pub(crate) fn prepend_to(&mut self, block: $crate::records::bc_op::BcOp) {
        self.base.prepend_to(block);
      }
    }
  };
}

pub(crate) use bc_inst_view;
