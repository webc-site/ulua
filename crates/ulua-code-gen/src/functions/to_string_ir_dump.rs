use alloc::string::String;

use ulua_common::functions::format_g::format_g;
use ulua_vm::records::proto::Proto;

use crate::{
  enums::{
    include_cfg_info::IncludeCfgInfo, include_reg_flow_info::IncludeRegFlowInfo,
    include_use_info::IncludeUseInfo, ir_block_kind::IrBlockKind, ir_condition::IrCondition,
    ir_op_kind::IrOpKind,
  },
  functions::{
    append::append,
    append_vm_constant::append_vm_constant,
    get_block_kind_name::get_block_kind_name,
    get_bytecode_type_name::{UserdataTypes, get_bytecode_type_name},
    get_tag_name::get_tag_name,
    has_result::has_result,
    is_pseudo::is_pseudo,
    to_string_detailed_ir_dump::{
      to_string_detailed as to_string_detailed_inst, to_string_detailed_block,
    },
    to_string_ir_dump::to_string_ir_to_string_context_ir_block_u32 as to_string_block,
    vm_const_op::vm_const_op,
    vm_exit_op::vm_exit_op,
    vm_reg_op::vm_reg_op,
    vm_upvalue_op::vm_upvalue_op,
  },
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{
    bytecode_types::{BytecodeTypes, LBC_TYPE_ANY},
    ir_block::{IrBlock, K_BLOCK_FLAG_SAFE_ENV_CHECK},
    ir_const::IrConst,
    ir_function::IrFunction,
    ir_inst::IrInst,
    ir_op::IrOp,
    ir_to_string_context::IrToStringContext,
  },
};

pub fn to_string_inst(ctx: &mut IrToStringContext, inst: &IrInst, index: u32) {
  ctx.result.push_str("  ");

  // 有结果的指令显示目标虚拟寄存器
  if has_result(inst.cmd) {
    append(ctx.result, format_args!("%{} = ", index));
  }

  // 指令名走 IrCmd 的 strum IntoStaticStr 编译期镜像（原 get_cmd_name 手写 217 臂 match）
  ctx.result.push_str(inst.cmd.into());

  for (i, &op) in inst.ops.as_slice().iter().enumerate() {
    if op.kind() == IrOpKind::None {
      continue;
    }

    ctx.result.push_str(if i == 0 { " " } else { ", " });
    to_string_op(ctx, op);
  }
}

pub fn to_string_ir_to_string_context_ir_block_u32(
  ctx: &mut IrToStringContext,
  block: &IrBlock,
  index: u32,
) {
  append(
    ctx.result,
    format_args!("{}_{}", get_block_kind_name(block.kind), index),
  );
}

static TEXT_FOR_CONDITION: [&str; IrCondition::Count as usize] = [
  "eq", "not_eq", "lt", "not_lt", "le", "not_le", "gt", "not_gt", "ge", "not_ge", "u_lt", "u_le",
  "u_gt", "u_ge",
];

const K_VM_EXIT_ENTRY_GUARD_PC: u32 = (1u32 << 28) - 1;

pub fn to_string_op(ctx: &mut IrToStringContext, op: IrOp) {
  match op.kind() {
    IrOpKind::None => {}
    IrOpKind::Undef => append(ctx.result, format_args!("undef")),
    IrOpKind::Constant => {
      let constant = ctx.constants[op.index() as usize];
      to_string_const(ctx.result, ctx.proto, constant);
    }
    IrOpKind::Condition => {
      CODEGEN_ASSERT!(op.index() < IrCondition::Count as u32);
      ctx.result.push_str(TEXT_FOR_CONDITION[op.index() as usize]);
    }
    IrOpKind::Inst => append(ctx.result, format_args!("%{}", op.index())),
    IrOpKind::Block => {
      let name = get_block_kind_name(ctx.blocks[op.index() as usize].kind);
      append(ctx.result, format_args!("{}_{}", name, op.index()));
    }
    IrOpKind::VmReg => append(ctx.result, format_args!("R{}", vm_reg_op(op))),
    IrOpKind::VmConst => {
      append(ctx.result, format_args!("K{}", vm_const_op(op)));

      if let Some(proto) = ctx.proto {
        append(ctx.result, format_args!(" ("));
        // `vm_const_op(op)` 是 IR 构建期依 `proto->sizek` 校验过的合法常量下标
        append_vm_constant(ctx.result, proto, vm_const_op(op));
        append(ctx.result, format_args!(")"));
      }
    }
    IrOpKind::VmUpvalue => append(ctx.result, format_args!("U{}", vm_upvalue_op(op))),
    IrOpKind::VmExit => {
      if vm_exit_op(op) == K_VM_EXIT_ENTRY_GUARD_PC {
        append(ctx.result, format_args!("exit(entry)"));
      } else {
        append(ctx.result, format_args!("exit({})", vm_exit_op(op)));
      }
    }
  }
}

/// dump 一个 IR 常量字面量；`proto` 为宿主原型（`None` 时 import 常量只打印编码值）。
pub(crate) fn to_string_const(result: &mut String, proto: Option<&Proto>, constant: IrConst) {
  // 载荷直配，替代原先「match kind + unsafe union 读」的两段式
  match constant {
    IrConst::Int(v) => append(result, format_args!("{}i", v)),
    IrConst::Int64(v) => append(result, format_args!("{}i", v)),
    IrConst::Uint(v) => append(result, format_args!("{}u", v)),
    IrConst::Double(d) => {
      if d.is_nan() {
        append(result, format_args!("nan"));
      } else {
        // C++ 用 printf "%.17g"；Rust 的 `{}` 打印能 round-trip
        // 的最短形式（如 `0.4` vs `0.40000000000000002`）。
        result.push_str(&format_g(d, 17));
      }
    }
    IrConst::Tag(tag) => result.push_str(get_tag_name(tag)),
    IrConst::Import(value_uint) => {
      append(result, format_args!("{}u", value_uint));

      if let Some(proto) = proto {
        append(result, format_args!(" ("));

        let count = (value_uint >> 30) as i32;
        let id0 = if count > 0 {
          ((value_uint >> 20) & 1023) as i32
        } else {
          -1
        };
        let id1 = if count > 1 {
          ((value_uint >> 10) & 1023) as i32
        } else {
          -1
        };
        let id2 = if count > 2 {
          (value_uint & 1023) as i32
        } else {
          -1
        };

        // id0/id1/id2 是 import 编码解出的常量下标，由 codegen 生成时保证为 `proto->k` 界内索引
        if id0 != -1 {
          append_vm_constant(result, proto, id0);
        }

        if id1 != -1 {
          append(result, format_args!("."));
          append_vm_constant(result, proto, id1);
        }

        if id2 != -1 {
          append(result, format_args!("."));
          append_vm_constant(result, proto, id2);
        }

        append(result, format_args!(")"));
      }
    }
  }
}

const LBC_TYPE_OPTIONAL_BIT: u8 = 0x80;

/// 可空标志后缀（cpp `(ty & LBC_TYPE_OPTIONAL_BIT) ? "?" : ""`）。
fn optional_suffix(t: u8) -> &'static str {
  if t & LBC_TYPE_OPTIONAL_BIT != 0 {
    "?"
  } else {
    ""
  }
}

/// cpp `toString(BytecodeTypes)`（IrDump.cpp）的类型行：`result <- a, b[, c]`。
/// 统一走 `format_args!`（`String` 的 `fmt::Write` 路径，见 `append`），
/// 替代旧版逐段 `push_str` 拼接与裸 ffi 指针参数。
pub(crate) fn to_string_bytecode_types(
  result: &mut String,
  bc_types: &BytecodeTypes,
  userdata_types: UserdataTypes<'_>,
) {
  append(
    result,
    format_args!(
      "{}{} <- {}{}, {}{}",
      get_bytecode_type_name(bc_types.result, userdata_types),
      optional_suffix(bc_types.result),
      get_bytecode_type_name(bc_types.a, userdata_types),
      optional_suffix(bc_types.a),
      get_bytecode_type_name(bc_types.b, userdata_types),
      optional_suffix(bc_types.b),
    ),
  );

  if bc_types.c != LBC_TYPE_ANY {
    append(
      result,
      format_args!(
        ", {}{}",
        get_bytecode_type_name(bc_types.c, userdata_types),
        optional_suffix(bc_types.c)
      ),
    );
  }
}

pub fn to_string(function: &mut IrFunction, include_use_info: IncludeUseInfo) -> String {
  let mut result = String::new();

  {
    let mut ctx = IrToStringContext {
      result: &mut result,
      blocks: &function.blocks,
      constants: &function.constants,
      cfg: &function.cfg,
      vm_exit_info: &function.vm_exit_info,
      proto: function.proto_view(),
    };

    for (i, &block) in function.blocks.iter().enumerate() {
      if block.kind == IrBlockKind::Dead {
        continue;
      }

      to_string_detailed_block(
        &mut ctx,
        &block,
        i as u32,
        include_use_info,
        IncludeCfgInfo::Yes,
        IncludeRegFlowInfo::Yes,
      );

      if block.start == !0u32 {
        append(ctx.result, format_args!(" *empty*\n\n"));
        continue;
      }

      if (block.flags & K_BLOCK_FLAG_SAFE_ENV_CHECK) != 0 {
        append(
          ctx.result,
          format_args!("   implicit CHECK_SAFE_ENV exit({})\n", block.startpc),
        );
      }

      let finish_bound = block
        .finish
        .min((function.instructions.len().saturating_sub(1)) as u32);
      if block.start <= finish_bound {
        for index in block.start..=finish_bound {
          let inst = &mut function.instructions[index as usize];

          // 跳过伪指令，除非它们仍被引用
          if is_pseudo(inst.cmd) && inst.use_count == 0 {
            continue;
          }

          append(ctx.result, format_args!(" "));
          to_string_detailed_inst(&mut ctx, &block, i as u32, inst, index, include_use_info);
        }
      }

      if block.expected_next_block != !0u32 {
        append(ctx.result, format_args!("; glued to: "));
        let glued = function.blocks[block.expected_next_block as usize];
        to_string_block(&mut ctx, &glued, block.expected_next_block);
        append(ctx.result, format_args!("\n"));
      }

      append(ctx.result, format_args!("\n"));
    }
  }

  result
}
