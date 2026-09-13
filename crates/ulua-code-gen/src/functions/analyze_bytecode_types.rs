use core::{ffi::c_char, mem::transmute};

use ulua_common::{
  FFlag::LuauCodegenRegTag2,
  enums::{
    luau_builtin_function::LuauBuiltinFunction, luau_bytecode_type::*, luau_opcode::LuauOpcode,
  },
  macros::{
    luau_insn_a::LUAU_INSN_A, luau_insn_aux_a::LUAU_INSN_AUX_A, luau_insn_aux_b::LUAU_INSN_AUX_B,
    luau_insn_aux_kv_16::LUAU_INSN_AUX_KV16, luau_insn_b::LUAU_INSN_B, luau_insn_c::LUAU_INSN_C,
    luau_insn_d::LUAU_INSN_D, luau_insn_op::LUAU_INSN_OP,
  },
};
use ulua_vm::{
  macros::{gco_2_ts::gco2ts, getstr::getstr},
  records::{g_cheader::GCheader, proto::Proto, t_string::tstring},
};

use crate::{
  enums::host_metamethod::HostMetamethod,
  functions::{
    apply_builtin_call::apply_builtin_call, get_bytecode_constant_tag::get_bytecode_constant_tag,
    get_op_length::get_op_length, get_reg_tag::get_reg_tag,
    is_custom_userdata_bytecode_type::is_custom_userdata_bytecode_type,
    opcode_to_host_metamethod::opcode_to_host_metamethod,
    prepare_reg_type_info_lookups::prepare_reg_type_info_lookups, refine_reg_type::refine_reg_type,
    refine_upvalue_type::refine_upvalue_type,
  },
  records::{bytecode_types::BytecodeTypes, host_ir_hooks::HostIrHooks, ir_function::IrFunction},
};

pub fn analyze_bytecode_types(function: &mut IrFunction, host_hooks: &HostIrHooks) {
  let proto = function.proto;
  crate::macros::codegen_assert::CODEGEN_ASSERT!(!proto.is_null());

  let bc_type_info = &mut function.bc_type_info;
  prepare_reg_type_info_lookups(bc_type_info);

  let mut reg_tags = [LBC_TYPE_ANY.0 as u8; 256];

  unsafe {
    function
      .bc_types
      .resize((*proto).sizecode as usize, BytecodeTypes::default());
  }

  let bc_blocks = function.bc_blocks.clone();

  for block in bc_blocks {
    crate::macros::codegen_assert::CODEGEN_ASSERT!(block.startpc != -1);
    crate::macros::codegen_assert::CODEGEN_ASSERT!(block.finishpc != -1);

    for (i, et) in bc_type_info.argument_types.iter().copied().enumerate() {
      reg_tags[i] = et & !(LBC_TYPE_OPTIONAL_BIT.0 as u8);
    }

    let (numparams, maxstacksize, code) = unsafe {
      (
        (*proto).numparams as i32,
        (*proto).maxstacksize as i32,
        (*proto).code,
      )
    };

    for i in numparams..maxstacksize {
      reg_tags[i as usize] = LBC_TYPE_ANY.0 as u8;
    }

    let mut known_next_call_result = LBC_TYPE_ANY.0 as u8;
    let mut i = block.startpc;

    while i <= block.finishpc {
      let pc = unsafe { code.add(i as usize) };
      let insn = unsafe { *pc };
      let op = LuauOpcode::from(LUAU_INSN_OP(insn) as u8);

      if !LuauCodegenRegTag2.get() {
        for el in &bc_type_info.reg_types {
          if el.r#type != LBC_TYPE_ANY.0 as u8 && i >= el.startpc && i < el.endpc {
            reg_tags[el.reg as usize] = el.r#type;
          }
        }
      }

      let mut bc_type = BytecodeTypes::default();

      match op {
        LuauOpcode::LOP_NOP => {}
        LuauOpcode::LOP_LOADNIL => {
          let ra = LUAU_INSN_A(insn) as usize;
          reg_tags[ra] = LBC_TYPE_NIL.0 as u8;
          bc_type.result = reg_tags[ra];
        }
        LuauOpcode::LOP_LOADB => {
          let ra = LUAU_INSN_A(insn) as usize;
          reg_tags[ra] = LBC_TYPE_BOOLEAN.0 as u8;
          bc_type.result = reg_tags[ra];
          refine_reg_type(bc_type_info, ra as u8, i, bc_type.result);
        }
        LuauOpcode::LOP_LOADN => {
          let ra = LUAU_INSN_A(insn) as usize;
          reg_tags[ra] = LBC_TYPE_NUMBER.0 as u8;
          bc_type.result = reg_tags[ra];
          refine_reg_type(bc_type_info, ra as u8, i, bc_type.result);
        }
        LuauOpcode::LOP_LOADK => {
          let ra = LUAU_INSN_A(insn) as usize;
          let kb = LUAU_INSN_D(insn) as u32;
          bc_type.a = get_bytecode_constant_tag(proto, kb);
          reg_tags[ra] = bc_type.a;
          bc_type.result = reg_tags[ra];
          refine_reg_type(bc_type_info, ra as u8, i, bc_type.result);
        }
        LuauOpcode::LOP_LOADKX => {
          let ra = LUAU_INSN_A(insn) as usize;
          let kb = unsafe { *pc.add(1) };
          bc_type.a = get_bytecode_constant_tag(proto, kb);
          reg_tags[ra] = bc_type.a;
          bc_type.result = reg_tags[ra];
          refine_reg_type(bc_type_info, ra as u8, i, bc_type.result);
        }
        LuauOpcode::LOP_MOVE => {
          let ra = LUAU_INSN_A(insn) as usize;
          let rb = LUAU_INSN_B(insn) as u8;
          bc_type.a = get_reg_tag(&mut reg_tags, bc_type_info, rb, i);
          reg_tags[ra] = bc_type.a;
          bc_type.result = reg_tags[ra];
          refine_reg_type(bc_type_info, ra as u8, i, bc_type.result);
        }
        LuauOpcode::LOP_GETTABLE => {
          let ra = LUAU_INSN_A(insn) as usize;
          let rb = LUAU_INSN_B(insn) as u8;
          let rc = LUAU_INSN_C(insn) as u8;
          bc_type.a = get_reg_tag(&mut reg_tags, bc_type_info, rb, i);
          bc_type.b = get_reg_tag(&mut reg_tags, bc_type_info, rc, i);
          reg_tags[ra] = LBC_TYPE_ANY.0 as u8;
          bc_type.result = reg_tags[ra];
        }
        LuauOpcode::LOP_SETTABLE => {
          let rb = LUAU_INSN_B(insn) as u8;
          let rc = LUAU_INSN_C(insn) as u8;
          bc_type.a = get_reg_tag(&mut reg_tags, bc_type_info, rb, i);
          bc_type.b = get_reg_tag(&mut reg_tags, bc_type_info, rc, i);
        }
        LuauOpcode::LOP_GETTABLEKS | LuauOpcode::LOP_GETUDATAKS => {
          let ra = LUAU_INSN_A(insn) as usize;
          let rb = LUAU_INSN_B(insn) as u8;
          let kc = if op == LuauOpcode::LOP_GETUDATAKS {
            LUAU_INSN_AUX_KV16(unsafe { *pc.add(1) })
          } else {
            unsafe { *pc.add(1) }
          };

          bc_type.a = get_reg_tag(&mut reg_tags, bc_type_info, rb, i);
          bc_type.b = get_bytecode_constant_tag(proto, kc);

          reg_tags[ra] = LBC_TYPE_ANY.0 as u8;

          let (field, len) = unsafe { proto_constant_string(proto, kc) };

          if bc_type.a == LBC_TYPE_VECTOR.0 as u8 {
            if len == 1 {
              let ch = unsafe { *field } as u8 | b' ';
              if ch == b'x' || ch == b'y' || ch == b'z' {
                reg_tags[ra] = LBC_TYPE_NUMBER.0 as u8;
              }
            }

            if reg_tags[ra] == LBC_TYPE_ANY.0 as u8
              && let Some(hook) = host_hooks.vector_access_bytecode_type
            {
              reg_tags[ra] = unsafe { hook(field, len) };
            }
          } else if is_custom_userdata_bytecode_type(bc_type.a)
            && reg_tags[ra] == LBC_TYPE_ANY.0 as u8
            && let Some(hook) = host_hooks.userdata_access_bytecode_type
          {
            reg_tags[ra] = unsafe { hook(bc_type.a, field, len) };
          }

          bc_type.result = reg_tags[ra];
        }
        LuauOpcode::LOP_SETTABLEKS | LuauOpcode::LOP_SETUDATAKS => {
          let rb = LUAU_INSN_B(insn) as u8;
          bc_type.a = get_reg_tag(&mut reg_tags, bc_type_info, rb, i);
          bc_type.b = LBC_TYPE_STRING.0 as u8;
        }
        LuauOpcode::LOP_GETTABLEN => {
          let ra = LUAU_INSN_A(insn) as usize;
          let rb = LUAU_INSN_B(insn) as u8;
          reg_tags[ra] = LBC_TYPE_ANY.0 as u8;
          bc_type.a = get_reg_tag(&mut reg_tags, bc_type_info, rb, i);
          bc_type.b = LBC_TYPE_NUMBER.0 as u8;
          bc_type.result = reg_tags[ra];
        }
        LuauOpcode::LOP_SETTABLEN => {
          let rb = LUAU_INSN_B(insn) as u8;
          bc_type.a = get_reg_tag(&mut reg_tags, bc_type_info, rb, i);
          bc_type.b = LBC_TYPE_NUMBER.0 as u8;
        }
        LuauOpcode::LOP_ADD | LuauOpcode::LOP_SUB => {
          let ra = LUAU_INSN_A(insn) as usize;
          let rb = LUAU_INSN_B(insn) as u8;
          let rc = LUAU_INSN_C(insn) as u8;
          bc_type.a = get_reg_tag(&mut reg_tags, bc_type_info, rb, i);
          bc_type.b = get_reg_tag(&mut reg_tags, bc_type_info, rc, i);
          reg_tags[ra] = binary_add_sub_type(host_hooks, bc_type.a, bc_type.b, op);
          bc_type.result = reg_tags[ra];
        }
        LuauOpcode::LOP_MUL | LuauOpcode::LOP_DIV | LuauOpcode::LOP_IDIV => {
          let ra = LUAU_INSN_A(insn) as usize;
          let rb = LUAU_INSN_B(insn) as u8;
          let rc = LUAU_INSN_C(insn) as u8;
          bc_type.a = get_reg_tag(&mut reg_tags, bc_type_info, rb, i);
          bc_type.b = get_reg_tag(&mut reg_tags, bc_type_info, rc, i);
          reg_tags[ra] = binary_mul_div_type(host_hooks, bc_type.a, bc_type.b, op);
          bc_type.result = reg_tags[ra];
        }
        LuauOpcode::LOP_MOD | LuauOpcode::LOP_POW => {
          let ra = LUAU_INSN_A(insn) as usize;
          let rb = LUAU_INSN_B(insn) as u8;
          let rc = LUAU_INSN_C(insn) as u8;
          bc_type.a = get_reg_tag(&mut reg_tags, bc_type_info, rb, i);
          bc_type.b = get_reg_tag(&mut reg_tags, bc_type_info, rc, i);
          reg_tags[ra] = binary_number_or_userdata_type(host_hooks, bc_type.a, bc_type.b, op);
          bc_type.result = reg_tags[ra];
        }
        LuauOpcode::LOP_ADDK | LuauOpcode::LOP_SUBK => {
          let ra = LUAU_INSN_A(insn) as usize;
          let rb = LUAU_INSN_B(insn) as u8;
          let kc = LUAU_INSN_C(insn);
          bc_type.a = get_reg_tag(&mut reg_tags, bc_type_info, rb, i);
          bc_type.b = get_bytecode_constant_tag(proto, kc);
          reg_tags[ra] = binary_add_sub_type(host_hooks, bc_type.a, bc_type.b, op);
          bc_type.result = reg_tags[ra];
        }
        LuauOpcode::LOP_MULK | LuauOpcode::LOP_DIVK | LuauOpcode::LOP_IDIVK => {
          let ra = LUAU_INSN_A(insn) as usize;
          let rb = LUAU_INSN_B(insn) as u8;
          let kc = LUAU_INSN_C(insn);
          bc_type.a = get_reg_tag(&mut reg_tags, bc_type_info, rb, i);
          bc_type.b = get_bytecode_constant_tag(proto, kc);
          reg_tags[ra] = binary_mul_div_type(host_hooks, bc_type.a, bc_type.b, op);
          bc_type.result = reg_tags[ra];
        }
        LuauOpcode::LOP_MODK | LuauOpcode::LOP_POWK => {
          let ra = LUAU_INSN_A(insn) as usize;
          let rb = LUAU_INSN_B(insn) as u8;
          let kc = LUAU_INSN_C(insn);
          bc_type.a = get_reg_tag(&mut reg_tags, bc_type_info, rb, i);
          bc_type.b = get_bytecode_constant_tag(proto, kc);
          reg_tags[ra] = binary_number_or_userdata_type(host_hooks, bc_type.a, bc_type.b, op);
          bc_type.result = reg_tags[ra];
        }
        LuauOpcode::LOP_SUBRK => {
          let ra = LUAU_INSN_A(insn) as usize;
          let kb = LUAU_INSN_B(insn);
          let rc = LUAU_INSN_C(insn) as u8;
          bc_type.a = get_bytecode_constant_tag(proto, kb);
          bc_type.b = get_reg_tag(&mut reg_tags, bc_type_info, rc, i);
          reg_tags[ra] = binary_add_sub_type(host_hooks, bc_type.a, bc_type.b, op);
          bc_type.result = reg_tags[ra];
        }
        LuauOpcode::LOP_DIVRK => {
          let ra = LUAU_INSN_A(insn) as usize;
          let kb = LUAU_INSN_B(insn);
          let rc = LUAU_INSN_C(insn) as u8;
          bc_type.a = get_bytecode_constant_tag(proto, kb);
          bc_type.b = get_reg_tag(&mut reg_tags, bc_type_info, rc, i);
          reg_tags[ra] = binary_mul_div_type(host_hooks, bc_type.a, bc_type.b, op);
          bc_type.result = reg_tags[ra];
        }
        LuauOpcode::LOP_NOT => {
          let ra = LUAU_INSN_A(insn) as usize;
          let rb = LUAU_INSN_B(insn) as u8;
          bc_type.a = get_reg_tag(&mut reg_tags, bc_type_info, rb, i);
          reg_tags[ra] = LBC_TYPE_BOOLEAN.0 as u8;
          bc_type.result = reg_tags[ra];
        }
        LuauOpcode::LOP_MINUS => {
          let ra = LUAU_INSN_A(insn) as usize;
          let rb = LUAU_INSN_B(insn) as u8;
          bc_type.a = get_reg_tag(&mut reg_tags, bc_type_info, rb, i);
          reg_tags[ra] = LBC_TYPE_ANY.0 as u8;
          if bc_type.a == LBC_TYPE_NUMBER.0 as u8 {
            reg_tags[ra] = LBC_TYPE_NUMBER.0 as u8;
          } else if bc_type.a == LBC_TYPE_VECTOR.0 as u8 {
            reg_tags[ra] = LBC_TYPE_VECTOR.0 as u8;
          } else if let Some(hook) = host_hooks.userdata_metamethod_bytecode_type
            && is_custom_userdata_bytecode_type(bc_type.a)
          {
            reg_tags[ra] = unsafe { hook(bc_type.a, LBC_TYPE_ANY.0 as u8, HostMetamethod::Minus) };
          }
          bc_type.result = reg_tags[ra];
        }
        LuauOpcode::LOP_LENGTH => {
          let ra = LUAU_INSN_A(insn) as usize;
          let rb = LUAU_INSN_B(insn) as u8;
          bc_type.a = get_reg_tag(&mut reg_tags, bc_type_info, rb, i);
          reg_tags[ra] = LBC_TYPE_NUMBER.0 as u8;
          bc_type.result = reg_tags[ra];
        }
        LuauOpcode::LOP_NEWTABLE | LuauOpcode::LOP_DUPTABLE => {
          let ra = LUAU_INSN_A(insn) as usize;
          reg_tags[ra] = LBC_TYPE_TABLE.0 as u8;
          bc_type.result = reg_tags[ra];
        }
        LuauOpcode::LOP_FASTCALL => {
          let bfid = LUAU_INSN_A(insn) as u8;
          let skip = LUAU_INSN_C(insn) as i32;
          let call = unsafe { *pc.add(skip as usize + 1) };
          crate::macros::codegen_assert::CODEGEN_ASSERT!(
            LuauOpcode::from(LUAU_INSN_OP(call) as u8) == LuauOpcode::LOP_CALL
          );
          let ra = LUAU_INSN_A(call) as usize;
          apply_builtin_call(builtin_function(bfid), &mut bc_type);
          reg_tags[ra + 1] = bc_type.a;
          reg_tags[ra + 2] = bc_type.b;
          reg_tags[ra + 3] = bc_type.c;
          reg_tags[ra] = bc_type.result;
          refine_reg_type(bc_type_info, ra as u8, i, bc_type.result);
          i += skip;
        }
        LuauOpcode::LOP_FASTCALL1 | LuauOpcode::LOP_FASTCALL2K => {
          let bfid = LUAU_INSN_A(insn) as u8;
          let skip = LUAU_INSN_C(insn) as i32;
          let call = unsafe { *pc.add(skip as usize + 1) };
          crate::macros::codegen_assert::CODEGEN_ASSERT!(
            LuauOpcode::from(LUAU_INSN_OP(call) as u8) == LuauOpcode::LOP_CALL
          );
          let ra = LUAU_INSN_A(call) as usize;
          apply_builtin_call(builtin_function(bfid), &mut bc_type);
          reg_tags[LUAU_INSN_B(insn) as usize] = bc_type.a;
          reg_tags[ra] = bc_type.result;
          refine_reg_type(bc_type_info, ra as u8, i, bc_type.result);
          i += skip;
        }
        LuauOpcode::LOP_FASTCALL2 => {
          let bfid = LUAU_INSN_A(insn) as u8;
          let skip = LUAU_INSN_C(insn) as i32;
          let call = unsafe { *pc.add(skip as usize + 1) };
          crate::macros::codegen_assert::CODEGEN_ASSERT!(
            LuauOpcode::from(LUAU_INSN_OP(call) as u8) == LuauOpcode::LOP_CALL
          );
          let ra = LUAU_INSN_A(call) as usize;
          apply_builtin_call(builtin_function(bfid), &mut bc_type);
          reg_tags[LUAU_INSN_B(insn) as usize] = bc_type.a;
          reg_tags[unsafe { *pc.add(1) } as usize] = bc_type.b;
          reg_tags[ra] = bc_type.result;
          refine_reg_type(bc_type_info, ra as u8, i, bc_type.result);
          i += skip;
        }
        LuauOpcode::LOP_FASTCALL3 => {
          let bfid = LUAU_INSN_A(insn) as u8;
          let skip = LUAU_INSN_C(insn) as i32;
          let aux = unsafe { *pc.add(1) };
          let call = unsafe { *pc.add(skip as usize + 1) };
          crate::macros::codegen_assert::CODEGEN_ASSERT!(
            LuauOpcode::from(LUAU_INSN_OP(call) as u8) == LuauOpcode::LOP_CALL
          );
          let ra = LUAU_INSN_A(call) as usize;
          apply_builtin_call(builtin_function(bfid), &mut bc_type);
          reg_tags[LUAU_INSN_B(insn) as usize] = bc_type.a;
          reg_tags[LUAU_INSN_AUX_A(aux) as usize] = bc_type.b;
          reg_tags[LUAU_INSN_AUX_B(aux) as usize] = bc_type.c;
          reg_tags[ra] = bc_type.result;
          refine_reg_type(bc_type_info, ra as u8, i, bc_type.result);
          i += skip;
        }
        LuauOpcode::LOP_FORNPREP => {
          let ra = LUAU_INSN_A(insn) as usize;
          reg_tags[ra] = LBC_TYPE_NUMBER.0 as u8;
          reg_tags[ra + 1] = LBC_TYPE_NUMBER.0 as u8;
          reg_tags[ra + 2] = LBC_TYPE_NUMBER.0 as u8;
          refine_reg_type(bc_type_info, ra as u8, i, reg_tags[ra]);
          refine_reg_type(bc_type_info, (ra + 1) as u8, i, reg_tags[ra + 1]);
          refine_reg_type(bc_type_info, (ra + 2) as u8, i, reg_tags[ra + 2]);
        }
        LuauOpcode::LOP_FORNLOOP => {
          let ra = LUAU_INSN_A(insn) as usize;
          reg_tags[ra] = LBC_TYPE_NUMBER.0 as u8;
          reg_tags[ra + 1] = LBC_TYPE_NUMBER.0 as u8;
          reg_tags[ra + 2] = LBC_TYPE_NUMBER.0 as u8;
        }
        LuauOpcode::LOP_CONCAT => {
          let ra = LUAU_INSN_A(insn) as usize;
          reg_tags[ra] = LBC_TYPE_STRING.0 as u8;
          bc_type.result = reg_tags[ra];
        }
        LuauOpcode::LOP_NEWCLOSURE | LuauOpcode::LOP_DUPCLOSURE => {
          let ra = LUAU_INSN_A(insn) as usize;
          reg_tags[ra] = LBC_TYPE_FUNCTION.0 as u8;
          bc_type.result = reg_tags[ra];
        }
        LuauOpcode::LOP_NAMECALL | LuauOpcode::LOP_NAMECALLUDATA => {
          let ra = LUAU_INSN_A(insn) as usize;
          let rb = LUAU_INSN_B(insn) as u8;
          let kc = if op == LuauOpcode::LOP_NAMECALLUDATA {
            LUAU_INSN_AUX_KV16(unsafe { *pc.add(1) })
          } else {
            unsafe { *pc.add(1) }
          };
          bc_type.a = get_reg_tag(&mut reg_tags, bc_type_info, rb, i);
          bc_type.b = get_bytecode_constant_tag(proto, kc);
          reg_tags[ra] = LBC_TYPE_FUNCTION.0 as u8;
          reg_tags[ra + 1] = bc_type.a;
          bc_type.result = LBC_TYPE_FUNCTION.0 as u8;

          let (field, len) = unsafe { proto_constant_string(proto, kc) };
          if bc_type.a == LBC_TYPE_VECTOR.0 as u8 {
            if let Some(hook) = host_hooks.vector_namecall_bytecode_type {
              known_next_call_result = unsafe { hook(field, len) };
            }
          } else if is_custom_userdata_bytecode_type(bc_type.a)
            && let Some(hook) = host_hooks.userdata_namecall_bytecode_type
          {
            known_next_call_result = unsafe { hook(bc_type.a, field, len) };
          }
        }
        LuauOpcode::LOP_CALLFB | LuauOpcode::LOP_CALL => {
          let ra = LUAU_INSN_A(insn) as usize;
          if known_next_call_result != LBC_TYPE_ANY.0 as u8 {
            bc_type.result = known_next_call_result;
            known_next_call_result = LBC_TYPE_ANY.0 as u8;
            reg_tags[ra] = bc_type.result;
          }
          refine_reg_type(bc_type_info, ra as u8, i, bc_type.result);
        }
        LuauOpcode::LOP_GETUPVAL => {
          let ra = LUAU_INSN_A(insn) as usize;
          let up = LUAU_INSN_B(insn) as usize;
          bc_type.a = LBC_TYPE_ANY.0 as u8;
          if up < bc_type_info.upvalue_types.len() {
            bc_type.a = bc_type_info.upvalue_types[up] & !(LBC_TYPE_OPTIONAL_BIT.0 as u8);
          }
          reg_tags[ra] = bc_type.a;
          bc_type.result = reg_tags[ra];
        }
        LuauOpcode::LOP_SETUPVAL => {
          let ra = LUAU_INSN_A(insn) as usize;
          let up = LUAU_INSN_B(insn) as i32;
          refine_upvalue_type(bc_type_info, up, reg_tags[ra]);
        }
        LuauOpcode::LOP_GETGLOBAL | LuauOpcode::LOP_GETIMPORT => {
          let ra = LUAU_INSN_A(insn) as usize;
          reg_tags[ra] = LBC_TYPE_ANY.0 as u8;
          bc_type.result = reg_tags[ra];
        }
        LuauOpcode::LOP_SETGLOBAL
        | LuauOpcode::LOP_RETURN
        | LuauOpcode::LOP_JUMP
        | LuauOpcode::LOP_JUMPBACK
        | LuauOpcode::LOP_JUMPIF
        | LuauOpcode::LOP_JUMPIFNOT => {}
        LuauOpcode::LOP_JUMPIFEQ
        | LuauOpcode::LOP_JUMPIFLE
        | LuauOpcode::LOP_JUMPIFLT
        | LuauOpcode::LOP_JUMPIFNOTEQ
        | LuauOpcode::LOP_JUMPIFNOTLE
        | LuauOpcode::LOP_JUMPIFNOTLT => {
          let ra = LUAU_INSN_A(insn) as u8;
          let rb = unsafe { *pc.add(1) } as u8;
          bc_type.a = get_reg_tag(&mut reg_tags, bc_type_info, ra, i);
          bc_type.b = get_reg_tag(&mut reg_tags, bc_type_info, rb, i);
        }
        LuauOpcode::LOP_AND | LuauOpcode::LOP_OR => {
          let ra = LUAU_INSN_A(insn) as usize;
          let rb = LUAU_INSN_B(insn) as u8;
          let rc = LUAU_INSN_C(insn) as u8;
          bc_type.a = get_reg_tag(&mut reg_tags, bc_type_info, rb, i);
          bc_type.b = get_reg_tag(&mut reg_tags, bc_type_info, rc, i);
          reg_tags[ra] = LBC_TYPE_ANY.0 as u8;
          bc_type.result = reg_tags[ra];
        }
        LuauOpcode::LOP_ANDK | LuauOpcode::LOP_ORK => {
          let ra = LUAU_INSN_A(insn) as usize;
          let rb = LUAU_INSN_B(insn) as u8;
          let kc = LUAU_INSN_C(insn);
          bc_type.a = get_reg_tag(&mut reg_tags, bc_type_info, rb, i);
          bc_type.b = get_bytecode_constant_tag(proto, kc);
          reg_tags[ra] = LBC_TYPE_ANY.0 as u8;
          bc_type.result = reg_tags[ra];
        }
        LuauOpcode::LOP_JUMPX
        | LuauOpcode::LOP_JUMPXEQKNIL
        | LuauOpcode::LOP_JUMPXEQKB
        | LuauOpcode::LOP_JUMPXEQKN
        | LuauOpcode::LOP_JUMPXEQKS
        | LuauOpcode::LOP_SETLIST
        | LuauOpcode::LOP_CLOSEUPVALS
        | LuauOpcode::LOP_FORGLOOP
        | LuauOpcode::LOP_FORGPREP_NEXT
        | LuauOpcode::LOP_FORGPREP_INEXT
        | LuauOpcode::LOP_COVERAGE
        | LuauOpcode::LOP_CAPTURE
        | LuauOpcode::LOP_PREPVARARGS
        | LuauOpcode::LOP_GETVARARGS
        | LuauOpcode::LOP_FORGPREP
        | LuauOpcode::LOP_NEWCLASSMEMBER => {}
        _ => crate::macros::codegen_assert::CODEGEN_ASSERT!(false),
      }

      function.bc_types[i as usize] = bc_type;
      i += get_op_length(op);
    }
  }
}

#[repr(C)]
struct tstringHeader {
  hdr: GCheader,
  _padding1: [u8; 1],
  atom: i16,
  _padding2: [u8; 2],
  next: *mut tstring,
  hash: u32,
  len: u32,
  data: [u8; 1],
}

unsafe fn proto_constant_string(proto: *mut Proto, index: u32) -> (*const c_char, usize) {
  unsafe {
    let gc = (*(*proto).k.add(index as usize)).value.gc;
    let ts = gco2ts!(gc) as *const _ as *const tstring;
    let field = getstr(ts);
    let len = (*(ts as *const tstringHeader)).len as usize;
    (field, len)
  }
}

fn binary_add_sub_type(host_hooks: &HostIrHooks, a: u8, b: u8, op: LuauOpcode) -> u8 {
  if a == LBC_TYPE_NUMBER.0 as u8 && b == LBC_TYPE_NUMBER.0 as u8 {
    LBC_TYPE_NUMBER.0 as u8
  } else if a == LBC_TYPE_VECTOR.0 as u8 && b == LBC_TYPE_VECTOR.0 as u8 {
    LBC_TYPE_VECTOR.0 as u8
  } else if let Some(hook) = host_hooks.userdata_metamethod_bytecode_type {
    if is_custom_userdata_bytecode_type(a) || is_custom_userdata_bytecode_type(b) {
      unsafe { hook(a, b, opcode_to_host_metamethod(op)) }
    } else {
      LBC_TYPE_ANY.0 as u8
    }
  } else {
    LBC_TYPE_ANY.0 as u8
  }
}

fn builtin_function(id: u8) -> LuauBuiltinFunction {
  unsafe { transmute(id) }
}

fn binary_mul_div_type(host_hooks: &HostIrHooks, a: u8, b: u8, op: LuauOpcode) -> u8 {
  if a == LBC_TYPE_NUMBER.0 as u8 {
    if b == LBC_TYPE_NUMBER.0 as u8 {
      LBC_TYPE_NUMBER.0 as u8
    } else if b == LBC_TYPE_VECTOR.0 as u8 {
      LBC_TYPE_VECTOR.0 as u8
    } else {
      LBC_TYPE_ANY.0 as u8
    }
  } else if a == LBC_TYPE_VECTOR.0 as u8 {
    if b == LBC_TYPE_NUMBER.0 as u8 || b == LBC_TYPE_VECTOR.0 as u8 {
      LBC_TYPE_VECTOR.0 as u8
    } else {
      LBC_TYPE_ANY.0 as u8
    }
  } else if let Some(hook) = host_hooks.userdata_metamethod_bytecode_type {
    if is_custom_userdata_bytecode_type(a) || is_custom_userdata_bytecode_type(b) {
      unsafe { hook(a, b, opcode_to_host_metamethod(op)) }
    } else {
      LBC_TYPE_ANY.0 as u8
    }
  } else {
    LBC_TYPE_ANY.0 as u8
  }
}

fn binary_number_or_userdata_type(host_hooks: &HostIrHooks, a: u8, b: u8, op: LuauOpcode) -> u8 {
  if a == LBC_TYPE_NUMBER.0 as u8 && b == LBC_TYPE_NUMBER.0 as u8 {
    LBC_TYPE_NUMBER.0 as u8
  } else if let Some(hook) = host_hooks.userdata_metamethod_bytecode_type {
    if is_custom_userdata_bytecode_type(a) || is_custom_userdata_bytecode_type(b) {
      unsafe { hook(a, b, opcode_to_host_metamethod(op)) }
    } else {
      LBC_TYPE_ANY.0 as u8
    }
  } else {
    LBC_TYPE_ANY.0 as u8
  }
}
