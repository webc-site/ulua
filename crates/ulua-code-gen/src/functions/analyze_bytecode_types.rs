use core::ffi::c_char;

use ulua_common::{
  enums::{
    luau_builtin_function::LuauBuiltinFunction, luau_bytecode_type::*, luau_opcode::LuauOpcode,
  },
  fflag::LuauCodegenRegTag2,
  functions::get_op_length::get_op_length,
  macros::{
    luau_insn_a::luau_insn_a, luau_insn_aux_a::luau_insn_aux_a, luau_insn_aux_b::luau_insn_aux_b,
    luau_insn_aux_kv_16::luau_insn_aux_kv16, luau_insn_b::luau_insn_b, luau_insn_c::luau_insn_c,
    luau_insn_d::luau_insn_d, luau_insn_op::luau_insn_op,
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
    get_reg_tag::get_reg_tag, is_custom_userdata_bytecode_type::is_custom_userdata_bytecode_type,
    opcode_to_host_metamethod::opcode_to_host_metamethod,
    prepare_reg_type_info_lookups::prepare_reg_type_info_lookups, refine_reg_type::refine_reg_type,
    refine_upvalue_type::refine_upvalue_type,
  },
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{bytecode_types::BytecodeTypes, host_ir_hooks::HostIrHooks, ir_function::IrFunction},
};

// 高频 LBC_TYPE 字节 tag：`.0` 为 u16，值均 < 256，编译期窄化（安全），
// 替代散落的 `.0 as u8`
const T_ANY: u8 = LBC_TYPE_ANY.0 as u8;
const T_NIL: u8 = LBC_TYPE_NIL.0 as u8;
const T_BOOLEAN: u8 = LBC_TYPE_BOOLEAN.0 as u8;
const T_NUMBER: u8 = LBC_TYPE_NUMBER.0 as u8;
const T_STRING: u8 = LBC_TYPE_STRING.0 as u8;
const T_TABLE: u8 = LBC_TYPE_TABLE.0 as u8;
const T_FUNCTION: u8 = LBC_TYPE_FUNCTION.0 as u8;
const T_VECTOR: u8 = LBC_TYPE_VECTOR.0 as u8;
const T_OPTIONAL: u8 = LBC_TYPE_OPTIONAL_BIT.0 as u8;

/// 遍历字节码推导各 pc 的类型 tag。
///
/// 内部 unsafe 指针解引用的统一前置条件（与 C++ 参考实现一致）：
/// `proto` 有效（入口 assert）、`code`/`pc` 界内（startpc/finishpc 来自合法 CFG）、
/// `k[]` 常量下标来自字节码 AUX 字段、hook 指针已判非空。
pub fn analyze_bytecode_types(function: &mut IrFunction, host_hooks: &HostIrHooks) {
  let proto = function.proto;
  CODEGEN_ASSERT!(!proto.is_null());

  let bc_type_info = &mut function.bc_type_info;
  prepare_reg_type_info_lookups(bc_type_info);

  let mut reg_tags = [T_ANY; 256];

  unsafe {
    function
      .bc_types
      .resize((*proto).sizecode as usize, BytecodeTypes::default());
  }

  // BytecodeBlock 是 Copy 且循环体只写 bc_type_info（字段不相交），按引用迭代免整表 clone
  for block in &function.bc_blocks {
    CODEGEN_ASSERT!(block.startpc != -1);
    CODEGEN_ASSERT!(block.finishpc != -1);

    for (i, et) in bc_type_info.argument_types.iter().copied().enumerate() {
      reg_tags[i] = et & !(T_OPTIONAL);
    }

    let (numparams, maxstacksize, code) = unsafe {
      (
        (*proto).numparams as i32,
        (*proto).maxstacksize as i32,
        (*proto).code,
      )
    };

    for i in numparams..maxstacksize {
      reg_tags[i as usize] = T_ANY;
    }

    let mut known_next_call_result = T_ANY;
    let mut i = block.startpc;

    while i <= block.finishpc {
      let pc = unsafe { code.add(i as usize) };
      let insn = unsafe { *pc };
      let op = LuauOpcode::from(luau_insn_op(insn) as u8);

      if !LuauCodegenRegTag2.get() {
        for el in &bc_type_info.reg_types {
          if el.r#type != T_ANY && i >= el.startpc && i < el.endpc {
            reg_tags[el.reg as usize] = el.r#type;
          }
        }
      }

      let mut bc_type = BytecodeTypes::default();

      match op {
        LuauOpcode::LOP_NOP => {}
        LuauOpcode::LOP_LOADNIL => {
          let ra = luau_insn_a(insn) as usize;
          reg_tags[ra] = T_NIL;
          bc_type.result = reg_tags[ra];
        }
        LuauOpcode::LOP_LOADB => {
          let ra = luau_insn_a(insn) as usize;
          reg_tags[ra] = T_BOOLEAN;
          bc_type.result = reg_tags[ra];
          refine_reg_type(bc_type_info, ra as u8, i, bc_type.result);
        }
        LuauOpcode::LOP_LOADN => {
          let ra = luau_insn_a(insn) as usize;
          reg_tags[ra] = T_NUMBER;
          bc_type.result = reg_tags[ra];
          refine_reg_type(bc_type_info, ra as u8, i, bc_type.result);
        }
        LuauOpcode::LOP_LOADK => {
          let ra = luau_insn_a(insn) as usize;
          let kb = luau_insn_d(insn) as u32;
          bc_type.a = get_bytecode_constant_tag(proto, kb);
          reg_tags[ra] = bc_type.a;
          bc_type.result = reg_tags[ra];
          refine_reg_type(bc_type_info, ra as u8, i, bc_type.result);
        }
        LuauOpcode::LOP_LOADKX => {
          let ra = luau_insn_a(insn) as usize;
          let kb = unsafe { *pc.add(1) };
          bc_type.a = get_bytecode_constant_tag(proto, kb);
          reg_tags[ra] = bc_type.a;
          bc_type.result = reg_tags[ra];
          refine_reg_type(bc_type_info, ra as u8, i, bc_type.result);
        }
        LuauOpcode::LOP_MOVE => {
          let ra = luau_insn_a(insn) as usize;
          let rb = luau_insn_b(insn) as u8;
          bc_type.a = get_reg_tag(&mut reg_tags, bc_type_info, rb, i);
          reg_tags[ra] = bc_type.a;
          bc_type.result = reg_tags[ra];
          refine_reg_type(bc_type_info, ra as u8, i, bc_type.result);
        }
        LuauOpcode::LOP_GETTABLE => {
          let ra = luau_insn_a(insn) as usize;
          let rb = luau_insn_b(insn) as u8;
          let rc = luau_insn_c(insn) as u8;
          bc_type.a = get_reg_tag(&mut reg_tags, bc_type_info, rb, i);
          bc_type.b = get_reg_tag(&mut reg_tags, bc_type_info, rc, i);
          reg_tags[ra] = T_ANY;
          bc_type.result = reg_tags[ra];
        }
        LuauOpcode::LOP_SETTABLE => {
          let rb = luau_insn_b(insn) as u8;
          let rc = luau_insn_c(insn) as u8;
          bc_type.a = get_reg_tag(&mut reg_tags, bc_type_info, rb, i);
          bc_type.b = get_reg_tag(&mut reg_tags, bc_type_info, rc, i);
        }
        LuauOpcode::LOP_GETTABLEKS | LuauOpcode::LOP_GETUDATAKS => {
          let ra = luau_insn_a(insn) as usize;
          let rb = luau_insn_b(insn) as u8;
          let kc = if op == LuauOpcode::LOP_GETUDATAKS {
            luau_insn_aux_kv16(unsafe { *pc.add(1) })
          } else {
            unsafe { *pc.add(1) }
          };

          bc_type.a = get_reg_tag(&mut reg_tags, bc_type_info, rb, i);
          bc_type.b = get_bytecode_constant_tag(proto, kc);

          reg_tags[ra] = T_ANY;

          let (field, len) = unsafe { proto_constant_string(proto, kc) };

          if bc_type.a == T_VECTOR {
            if len == 1 {
              let ch = unsafe { *field } as u8 | b' ';
              if ch == b'x' || ch == b'y' || ch == b'z' {
                reg_tags[ra] = T_NUMBER;
              }
            }

            if reg_tags[ra] == T_ANY
              && let Some(hook) = host_hooks.vector_access_bytecode_type
            {
              reg_tags[ra] = unsafe { hook(field, len) };
            }
          } else if is_custom_userdata_bytecode_type(bc_type.a)
            && reg_tags[ra] == T_ANY
            && let Some(hook) = host_hooks.userdata_access_bytecode_type
          {
            reg_tags[ra] = unsafe { hook(bc_type.a, field, len) };
          }

          bc_type.result = reg_tags[ra];
        }
        LuauOpcode::LOP_SETTABLEKS | LuauOpcode::LOP_SETUDATAKS => {
          let rb = luau_insn_b(insn) as u8;
          bc_type.a = get_reg_tag(&mut reg_tags, bc_type_info, rb, i);
          bc_type.b = T_STRING;
        }
        LuauOpcode::LOP_GETTABLEN => {
          let ra = luau_insn_a(insn) as usize;
          let rb = luau_insn_b(insn) as u8;
          reg_tags[ra] = T_ANY;
          bc_type.a = get_reg_tag(&mut reg_tags, bc_type_info, rb, i);
          bc_type.b = T_NUMBER;
          bc_type.result = reg_tags[ra];
        }
        LuauOpcode::LOP_SETTABLEN => {
          let rb = luau_insn_b(insn) as u8;
          bc_type.a = get_reg_tag(&mut reg_tags, bc_type_info, rb, i);
          bc_type.b = T_NUMBER;
        }
        LuauOpcode::LOP_ADD | LuauOpcode::LOP_SUB => {
          let ra = luau_insn_a(insn) as usize;
          let rb = luau_insn_b(insn) as u8;
          let rc = luau_insn_c(insn) as u8;
          bc_type.a = get_reg_tag(&mut reg_tags, bc_type_info, rb, i);
          bc_type.b = get_reg_tag(&mut reg_tags, bc_type_info, rc, i);
          reg_tags[ra] = binary_add_sub_type(host_hooks, bc_type.a, bc_type.b, op);
          bc_type.result = reg_tags[ra];
        }
        LuauOpcode::LOP_MUL | LuauOpcode::LOP_DIV | LuauOpcode::LOP_IDIV => {
          let ra = luau_insn_a(insn) as usize;
          let rb = luau_insn_b(insn) as u8;
          let rc = luau_insn_c(insn) as u8;
          bc_type.a = get_reg_tag(&mut reg_tags, bc_type_info, rb, i);
          bc_type.b = get_reg_tag(&mut reg_tags, bc_type_info, rc, i);
          reg_tags[ra] = binary_mul_div_type(host_hooks, bc_type.a, bc_type.b, op);
          bc_type.result = reg_tags[ra];
        }
        LuauOpcode::LOP_MOD | LuauOpcode::LOP_POW => {
          let ra = luau_insn_a(insn) as usize;
          let rb = luau_insn_b(insn) as u8;
          let rc = luau_insn_c(insn) as u8;
          bc_type.a = get_reg_tag(&mut reg_tags, bc_type_info, rb, i);
          bc_type.b = get_reg_tag(&mut reg_tags, bc_type_info, rc, i);
          reg_tags[ra] = binary_number_or_userdata_type(host_hooks, bc_type.a, bc_type.b, op);
          bc_type.result = reg_tags[ra];
        }
        LuauOpcode::LOP_ADDK | LuauOpcode::LOP_SUBK => {
          let ra = luau_insn_a(insn) as usize;
          let rb = luau_insn_b(insn) as u8;
          let kc = luau_insn_c(insn);
          bc_type.a = get_reg_tag(&mut reg_tags, bc_type_info, rb, i);
          bc_type.b = get_bytecode_constant_tag(proto, kc);
          reg_tags[ra] = binary_add_sub_type(host_hooks, bc_type.a, bc_type.b, op);
          bc_type.result = reg_tags[ra];
        }
        LuauOpcode::LOP_MULK | LuauOpcode::LOP_DIVK | LuauOpcode::LOP_IDIVK => {
          let ra = luau_insn_a(insn) as usize;
          let rb = luau_insn_b(insn) as u8;
          let kc = luau_insn_c(insn);
          bc_type.a = get_reg_tag(&mut reg_tags, bc_type_info, rb, i);
          bc_type.b = get_bytecode_constant_tag(proto, kc);
          reg_tags[ra] = binary_mul_div_type(host_hooks, bc_type.a, bc_type.b, op);
          bc_type.result = reg_tags[ra];
        }
        LuauOpcode::LOP_MODK | LuauOpcode::LOP_POWK => {
          let ra = luau_insn_a(insn) as usize;
          let rb = luau_insn_b(insn) as u8;
          let kc = luau_insn_c(insn);
          bc_type.a = get_reg_tag(&mut reg_tags, bc_type_info, rb, i);
          bc_type.b = get_bytecode_constant_tag(proto, kc);
          reg_tags[ra] = binary_number_or_userdata_type(host_hooks, bc_type.a, bc_type.b, op);
          bc_type.result = reg_tags[ra];
        }
        LuauOpcode::LOP_SUBRK => {
          let ra = luau_insn_a(insn) as usize;
          let kb = luau_insn_b(insn);
          let rc = luau_insn_c(insn) as u8;
          bc_type.a = get_bytecode_constant_tag(proto, kb);
          bc_type.b = get_reg_tag(&mut reg_tags, bc_type_info, rc, i);
          reg_tags[ra] = binary_add_sub_type(host_hooks, bc_type.a, bc_type.b, op);
          bc_type.result = reg_tags[ra];
        }
        LuauOpcode::LOP_DIVRK => {
          let ra = luau_insn_a(insn) as usize;
          let kb = luau_insn_b(insn);
          let rc = luau_insn_c(insn) as u8;
          bc_type.a = get_bytecode_constant_tag(proto, kb);
          bc_type.b = get_reg_tag(&mut reg_tags, bc_type_info, rc, i);
          reg_tags[ra] = binary_mul_div_type(host_hooks, bc_type.a, bc_type.b, op);
          bc_type.result = reg_tags[ra];
        }
        LuauOpcode::LOP_NOT => {
          let ra = luau_insn_a(insn) as usize;
          let rb = luau_insn_b(insn) as u8;
          bc_type.a = get_reg_tag(&mut reg_tags, bc_type_info, rb, i);
          reg_tags[ra] = T_BOOLEAN;
          bc_type.result = reg_tags[ra];
        }
        LuauOpcode::LOP_MINUS => {
          let ra = luau_insn_a(insn) as usize;
          let rb = luau_insn_b(insn) as u8;
          bc_type.a = get_reg_tag(&mut reg_tags, bc_type_info, rb, i);
          reg_tags[ra] = T_ANY;
          if bc_type.a == T_NUMBER {
            reg_tags[ra] = T_NUMBER;
          } else if bc_type.a == T_VECTOR {
            reg_tags[ra] = T_VECTOR;
          } else {
            reg_tags[ra] = userdata_hook_type(host_hooks, bc_type.a, T_ANY, HostMetamethod::Minus);
          }
          bc_type.result = reg_tags[ra];
        }
        LuauOpcode::LOP_LENGTH => {
          let ra = luau_insn_a(insn) as usize;
          let rb = luau_insn_b(insn) as u8;
          bc_type.a = get_reg_tag(&mut reg_tags, bc_type_info, rb, i);
          reg_tags[ra] = T_NUMBER;
          bc_type.result = reg_tags[ra];
        }
        LuauOpcode::LOP_NEWTABLE | LuauOpcode::LOP_DUPTABLE => {
          let ra = luau_insn_a(insn) as usize;
          reg_tags[ra] = T_TABLE;
          bc_type.result = reg_tags[ra];
        }
        LuauOpcode::LOP_FASTCALL => {
          let bfid = luau_insn_a(insn) as u8;
          let skip = luau_insn_c(insn) as i32;
          let call = unsafe { *pc.add(skip as usize + 1) };
          CODEGEN_ASSERT!(LuauOpcode::from(luau_insn_op(call) as u8) == LuauOpcode::LOP_CALL);
          let ra = luau_insn_a(call) as usize;
          apply_builtin_call(builtin_function(bfid), &mut bc_type);
          reg_tags[ra + 1] = bc_type.a;
          reg_tags[ra + 2] = bc_type.b;
          reg_tags[ra + 3] = bc_type.c;
          reg_tags[ra] = bc_type.result;
          refine_reg_type(bc_type_info, ra as u8, i, bc_type.result);
          i += skip;
        }
        LuauOpcode::LOP_FASTCALL1 | LuauOpcode::LOP_FASTCALL2K => {
          let bfid = luau_insn_a(insn) as u8;
          let skip = luau_insn_c(insn) as i32;
          let call = unsafe { *pc.add(skip as usize + 1) };
          CODEGEN_ASSERT!(LuauOpcode::from(luau_insn_op(call) as u8) == LuauOpcode::LOP_CALL);
          let ra = luau_insn_a(call) as usize;
          apply_builtin_call(builtin_function(bfid), &mut bc_type);
          reg_tags[luau_insn_b(insn) as usize] = bc_type.a;
          reg_tags[ra] = bc_type.result;
          refine_reg_type(bc_type_info, ra as u8, i, bc_type.result);
          i += skip;
        }
        LuauOpcode::LOP_FASTCALL2 => {
          let bfid = luau_insn_a(insn) as u8;
          let skip = luau_insn_c(insn) as i32;
          let call = unsafe { *pc.add(skip as usize + 1) };
          CODEGEN_ASSERT!(LuauOpcode::from(luau_insn_op(call) as u8) == LuauOpcode::LOP_CALL);
          let ra = luau_insn_a(call) as usize;
          apply_builtin_call(builtin_function(bfid), &mut bc_type);
          reg_tags[luau_insn_b(insn) as usize] = bc_type.a;
          reg_tags[unsafe { *pc.add(1) } as usize] = bc_type.b;
          reg_tags[ra] = bc_type.result;
          refine_reg_type(bc_type_info, ra as u8, i, bc_type.result);
          i += skip;
        }
        LuauOpcode::LOP_FASTCALL3 => {
          let bfid = luau_insn_a(insn) as u8;
          let skip = luau_insn_c(insn) as i32;
          let aux = unsafe { *pc.add(1) };
          let call = unsafe { *pc.add(skip as usize + 1) };
          CODEGEN_ASSERT!(LuauOpcode::from(luau_insn_op(call) as u8) == LuauOpcode::LOP_CALL);
          let ra = luau_insn_a(call) as usize;
          apply_builtin_call(builtin_function(bfid), &mut bc_type);
          reg_tags[luau_insn_b(insn) as usize] = bc_type.a;
          reg_tags[luau_insn_aux_a(aux) as usize] = bc_type.b;
          reg_tags[luau_insn_aux_b(aux) as usize] = bc_type.c;
          reg_tags[ra] = bc_type.result;
          refine_reg_type(bc_type_info, ra as u8, i, bc_type.result);
          i += skip;
        }
        LuauOpcode::LOP_FORNPREP => {
          let ra = luau_insn_a(insn) as usize;
          reg_tags[ra] = T_NUMBER;
          reg_tags[ra + 1] = T_NUMBER;
          reg_tags[ra + 2] = T_NUMBER;
          refine_reg_type(bc_type_info, ra as u8, i, reg_tags[ra]);
          refine_reg_type(bc_type_info, (ra + 1) as u8, i, reg_tags[ra + 1]);
          refine_reg_type(bc_type_info, (ra + 2) as u8, i, reg_tags[ra + 2]);
        }
        LuauOpcode::LOP_FORNLOOP => {
          let ra = luau_insn_a(insn) as usize;
          reg_tags[ra] = T_NUMBER;
          reg_tags[ra + 1] = T_NUMBER;
          reg_tags[ra + 2] = T_NUMBER;
        }
        LuauOpcode::LOP_CONCAT => {
          let ra = luau_insn_a(insn) as usize;
          reg_tags[ra] = T_STRING;
          bc_type.result = reg_tags[ra];
        }
        LuauOpcode::LOP_NEWCLOSURE | LuauOpcode::LOP_DUPCLOSURE => {
          let ra = luau_insn_a(insn) as usize;
          reg_tags[ra] = T_FUNCTION;
          bc_type.result = reg_tags[ra];
        }
        LuauOpcode::LOP_NAMECALL | LuauOpcode::LOP_NAMECALLUDATA => {
          let ra = luau_insn_a(insn) as usize;
          let rb = luau_insn_b(insn) as u8;
          let kc = if op == LuauOpcode::LOP_NAMECALLUDATA {
            luau_insn_aux_kv16(unsafe { *pc.add(1) })
          } else {
            unsafe { *pc.add(1) }
          };
          bc_type.a = get_reg_tag(&mut reg_tags, bc_type_info, rb, i);
          bc_type.b = get_bytecode_constant_tag(proto, kc);
          reg_tags[ra] = T_FUNCTION;
          reg_tags[ra + 1] = bc_type.a;
          bc_type.result = T_FUNCTION;

          let (field, len) = unsafe { proto_constant_string(proto, kc) };
          if bc_type.a == T_VECTOR {
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
          let ra = luau_insn_a(insn) as usize;
          if known_next_call_result != T_ANY {
            bc_type.result = known_next_call_result;
            known_next_call_result = T_ANY;
            reg_tags[ra] = bc_type.result;
          }
          refine_reg_type(bc_type_info, ra as u8, i, bc_type.result);
        }
        LuauOpcode::LOP_GETUPVAL => {
          let ra = luau_insn_a(insn) as usize;
          let up = luau_insn_b(insn) as usize;
          bc_type.a = T_ANY;
          if up < bc_type_info.upvalue_types.len() {
            bc_type.a = bc_type_info.upvalue_types[up] & !(T_OPTIONAL);
          }
          reg_tags[ra] = bc_type.a;
          bc_type.result = reg_tags[ra];
        }
        LuauOpcode::LOP_SETUPVAL => {
          let ra = luau_insn_a(insn) as usize;
          let up = luau_insn_b(insn) as i32;
          refine_upvalue_type(bc_type_info, up, reg_tags[ra]);
        }
        LuauOpcode::LOP_GETGLOBAL | LuauOpcode::LOP_GETIMPORT => {
          let ra = luau_insn_a(insn) as usize;
          reg_tags[ra] = T_ANY;
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
          let ra = luau_insn_a(insn) as u8;
          let rb = unsafe { *pc.add(1) } as u8;
          bc_type.a = get_reg_tag(&mut reg_tags, bc_type_info, ra, i);
          bc_type.b = get_reg_tag(&mut reg_tags, bc_type_info, rb, i);
        }
        LuauOpcode::LOP_AND | LuauOpcode::LOP_OR => {
          let ra = luau_insn_a(insn) as usize;
          let rb = luau_insn_b(insn) as u8;
          let rc = luau_insn_c(insn) as u8;
          bc_type.a = get_reg_tag(&mut reg_tags, bc_type_info, rb, i);
          bc_type.b = get_reg_tag(&mut reg_tags, bc_type_info, rc, i);
          reg_tags[ra] = T_ANY;
          bc_type.result = reg_tags[ra];
        }
        LuauOpcode::LOP_ANDK | LuauOpcode::LOP_ORK => {
          let ra = luau_insn_a(insn) as usize;
          let rb = luau_insn_b(insn) as u8;
          let kc = luau_insn_c(insn);
          bc_type.a = get_reg_tag(&mut reg_tags, bc_type_info, rb, i);
          bc_type.b = get_bytecode_constant_tag(proto, kc);
          reg_tags[ra] = T_ANY;
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
        _ => CODEGEN_ASSERT!(false),
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

/// # Safety
/// `proto` 必须有效且 `k[index]` 为 GC 字符串常量，调用方须满足 C++ 参考实现的前置条件。
unsafe fn proto_constant_string(proto: *mut Proto, index: u32) -> (*const c_char, usize) {
  unsafe {
    let gc = (*(*proto).k.add(index as usize)).value.gc;
    let ts = gco2ts!(gc) as *const _ as *const tstring;
    let field = getstr(ts);
    let len = (*(ts as *const tstringHeader)).len as usize;
    (field, len)
  }
}

/// 自定义 userdata 的 metamethod 类型回调：hook 已注册且任一操作数为自定义
/// userdata 时才调用，否则落 ANY（与 C++ getBytecodeType 系列语义一致）
#[inline]
fn userdata_hook_type(host_hooks: &HostIrHooks, a: u8, b: u8, method: HostMetamethod) -> u8 {
  if let Some(hook) = host_hooks.userdata_metamethod_bytecode_type
    && (is_custom_userdata_bytecode_type(a) || is_custom_userdata_bytecode_type(b))
  {
    // SAFETY：hook 为宿主注册的 extern "C-unwind" 函数指针，已判非空
    unsafe { hook(a, b, method) }
  } else {
    T_ANY
  }
}

fn binary_add_sub_type(host_hooks: &HostIrHooks, a: u8, b: u8, op: LuauOpcode) -> u8 {
  if a == T_NUMBER && b == T_NUMBER {
    T_NUMBER
  } else if a == T_VECTOR && b == T_VECTOR {
    T_VECTOR
  } else {
    userdata_hook_type(host_hooks, a, b, opcode_to_host_metamethod(op))
  }
}

/// bfid → 枚举：校验后再转换，越界 id（损坏字节码）归入 LBF_NONE，
/// apply_builtin_call 的 LBF_NONE 臂写 result = ANY，与 C++ switch default 一致
fn builtin_function(id: u8) -> LuauBuiltinFunction {
  LuauBuiltinFunction::from_id(i32::from(id)).unwrap_or(LuauBuiltinFunction::LBF_NONE)
}

fn binary_mul_div_type(host_hooks: &HostIrHooks, a: u8, b: u8, op: LuauOpcode) -> u8 {
  if a == T_NUMBER {
    if b == T_NUMBER {
      T_NUMBER
    } else if b == T_VECTOR {
      T_VECTOR
    } else {
      T_ANY
    }
  } else if a == T_VECTOR {
    if b == T_NUMBER || b == T_VECTOR {
      T_VECTOR
    } else {
      T_ANY
    }
  } else {
    userdata_hook_type(host_hooks, a, b, opcode_to_host_metamethod(op))
  }
}

fn binary_number_or_userdata_type(host_hooks: &HostIrHooks, a: u8, b: u8, op: LuauOpcode) -> u8 {
  if a == T_NUMBER && b == T_NUMBER {
    T_NUMBER
  } else {
    userdata_hook_type(host_hooks, a, b, opcode_to_host_metamethod(op))
  }
}
