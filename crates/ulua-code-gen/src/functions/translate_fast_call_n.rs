use ulua_common::{
  enums::luau_opcode::LuauOpcode,
  fflag,
  functions::get_op_length::get_op_length,
  macros::luau_insn_ops::{luau_insn_a, luau_insn_b, luau_insn_c, luau_insn_op},
};
use ulua_vm::{enums::lua_type::LuaType, macros::lua_multret::LUA_MULTRET};

use crate::{
  enums::{
    builtin_impl_type::BuiltinImplType, ir_block_kind::IrBlockKind, ir_cmd::IrCmd,
    ir_op_kind::IrOpKind,
  },
  functions::{
    proto_view::{constant_integer, constant_number, with_constant_value},
    translate_builtin::translate_builtin,
    vm_const_op::vm_const_op,
  },
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{builtin_args::BuiltinArgs, ir_builder::IrBuilder, ir_op::IrOp},
  type_aliases::instruction_ir_builder::Instruction,
};

/// 翻译 LOP_FASTCALL 系列（FASTCALL* + 收尾 LOP_CALL）。
///
/// 前置约定（字节码流以切片访问，越界即 panic，不构成 UB）：
/// 字节码结构（与 C++ 参考实现一致）：切片 `code` 于 `pcpos` 处含一条界内的
/// FASTCALL 指令，其 C 操作数 `skip` 使 `code[pcpos+skip+1]` 落在序列末尾的 LOP_CALL 指令
/// （由 `CODEGEN_ASSERT` 复核 opcode）；`build.function.proto` 非空，`vm_const_op(args)` 为合法常量表下标
/// （常量池读取经 `with_constant_value` 门面收口）。
pub(crate) fn translate_fast_call_n(
  build: &mut IrBuilder,
  code: &[Instruction],
  pcpos: i32,
  custom_params: bool,
  custom_param_count: i32,
  custom_args: IrOp,
  custom_arg3: IrOp,
) -> IrOp {
  let opcode = LuauOpcode::from(luau_insn_op(code[pcpos as usize]) as u8);
  // 解码说明:以下为字节码流按 pcpos 索引的安全读取,越界即 panic(字节码结构保证界内)。
  let bfid = luau_insn_a(code[pcpos as usize]) as i32;
  // 同上,C 域 skip 为本 fastcall 序列相对当前字的偏移(字节码结构保证 >0)。
  let skip = luau_insn_c(code[pcpos as usize]) as i32;

  // 按字节码结构 fastcall 序列末尾必为 CALL,`code[pcpos+skip+1]` 界内(由下行断言复核)。
  let call = code[pcpos as usize + skip as usize + 1];
  CODEGEN_ASSERT!(LuauOpcode::from(luau_insn_op(call) as u8) == LuauOpcode::LOP_CALL);
  let ra = luau_insn_a(call) as i32;

  let nparams = if custom_params {
    custom_param_count
  } else {
    (luau_insn_b(call) as i32) - 1
  };
  let nresults = (luau_insn_c(call) as i32) - 1;
  let arg = if custom_params {
    // 界内约定:  `code[pcpos]` 为界内当前 FASTCALL 指令(契约),B 域参数起始寄存器号直接解码。
    luau_insn_b(code[pcpos as usize]) as i32
  } else {
    ra + 1
  };
  let args = if custom_params {
    custom_args
  } else {
    build.vm_reg((ra + 2) as u8)
  };

  let mut builtin_args = args;

  if args.kind() == IrOpKind::VmConst {
    CODEGEN_ASSERT!(!build.function.proto.is_null());
    // Proto::k 直读收口进 `with_constant_value` 门面(§2): 判定顺序与 `const_*` 内部化调用
    // 位置保留——Number 走 const_double、Integer(fflag 开)走 const_int_64, 其余不改写参数。
    builtin_args = with_constant_value(build.function.proto, vm_const_op(args) as u32, |tv| {
      let tt = tv.tt;
      if tt == LuaType::Number as i32 {
        build.const_double(constant_number(tv))
      } else if fflag::LuauCodegenInteger3.get() && tt == LuaType::Integer as i32 {
        build.const_int_64(constant_integer(tv))
      } else {
        args
      }
    })
    .expect("translate_fast_call_n: proto/k 非空且 vm_const_op 界内(codegen 契约)");
  }

  let builtin_arg3 = if custom_params {
    custom_arg3
  } else {
    build.vm_reg((ra + 3) as u8)
  };

  let fallback = build.fallback_block(pcpos as u32);

  build.check_safe_env(pcpos + get_op_length(opcode));

  let bargs = BuiltinArgs {
    ra,
    arg,
    args: builtin_args,
    arg3: builtin_arg3,
    nparams,
    nresults,
    pcpos: pcpos + get_op_length(opcode),
  };

  let br = translate_builtin(build, bfid, bargs, fallback);

  if br.r#type != BuiltinImplType::None {
    CODEGEN_ASSERT!(nparams != LUA_MULTRET);

    if nresults == LUA_MULTRET {
      let reg_ra = build.vm_reg(ra as u8);
      let actual_count = build.const_int(br.actual_result_count);
      build.inst_ir_cmd_ir_op_ir_op(IrCmd::AdjustStackToReg, reg_ra, actual_count);
    } else {
      let reg_ra_next = build.vm_reg((ra + 1) as u8);
      let dead_count = build.const_int(-1);
      build.inst_ir_cmd_ir_op_ir_op(IrCmd::MarkDead, reg_ra_next, dead_count);
    }

    if br.r#type != BuiltinImplType::UsesFallback {
      let block = build.function.block_op(fallback);

      block.kind = IrBlockKind::Dead;

      return build.undef();
    }
  } else {
    let arg3 = if custom_params {
      custom_arg3
    } else {
      build.undef()
    };

    let savedpc = build.const_uint((pcpos + get_op_length(opcode)) as u32);
    build.inst_ir_cmd_ir_op(IrCmd::SetSavedpc, savedpc);

    let bfid_op = build.const_uint(bfid as u32);
    let reg_ra = build.vm_reg(ra as u8);
    let reg_arg = build.vm_reg(arg as u8);
    let nparams_op = build.const_int(nparams);
    let nresults_op = build.const_int(nresults);
    let res = build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op_ir_op_ir_op(
      IrCmd::InvokeFastcall,
      bfid_op,
      reg_ra,
      reg_arg,
      args,
      arg3,
      nparams_op,
      nresults_op,
    );
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::CheckFastcallRes, res, fallback);

    if nresults == LUA_MULTRET {
      let reg_ra = build.vm_reg(ra as u8);
      build.inst_ir_cmd_ir_op_ir_op(IrCmd::AdjustStackToReg, reg_ra, res);
    } else if nparams == LUA_MULTRET {
      build.inst_ir_cmd(IrCmd::AdjustStackToTop);
    }
  }

  fallback
}
