use core::ffi::c_char;

use ulua_common::{
  enums::{
    luau_builtin_function::LuauBuiltinFunction, luau_bytecode_type::LuauBytecodeType,
    luau_opcode::LuauOpcode,
  },
  functions::get_op_length::get_op_length,
  macros::{
    luau_insn_a::luau_insn_a, luau_insn_aux_a::luau_insn_aux_a, luau_insn_aux_b::luau_insn_aux_b,
    luau_insn_aux_kv_16::luau_insn_aux_kv16, luau_insn_b::luau_insn_b, luau_insn_c::luau_insn_c,
    luau_insn_d::luau_insn_d, luau_insn_op::luau_insn_op,
  },
};

use crate::{
  enums::host_metamethod::HostMetamethod,
  functions::{
    apply_builtin_call::apply_builtin_call,
    get_bytecode_constant_tag::get_bytecode_constant_tag,
    get_reg_tag::get_reg_tag,
    is_custom_userdata_bytecode_type::is_custom_userdata_bytecode_type,
    opcode_to_host_metamethod::opcode_to_host_metamethod,
    prepare_reg_type_info_lookups::prepare_reg_type_info_lookups,
    proto_views::{code, string_constant},
    refine_reg_type::refine_reg_type,
    refine_upvalue_type::refine_upvalue_type,
  },
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{bytecode_types::BytecodeTypes, host_ir_hooks::HostIrHooks, ir_function::IrFunction},
};

// 高频 LBC_TYPE 字节 tag：`.0` 为 u16，值均 < 256，编译期窄化（安全），
// 替代散落的 `.0 as u8`
const T_ANY: u8 = LuauBytecodeType::LBC_TYPE_ANY.0 as u8;
const T_NIL: u8 = LuauBytecodeType::LBC_TYPE_NIL.0 as u8;
const T_BOOLEAN: u8 = LuauBytecodeType::LBC_TYPE_BOOLEAN.0 as u8;
const T_NUMBER: u8 = LuauBytecodeType::LBC_TYPE_NUMBER.0 as u8;
const T_STRING: u8 = LuauBytecodeType::LBC_TYPE_STRING.0 as u8;
const T_TABLE: u8 = LuauBytecodeType::LBC_TYPE_TABLE.0 as u8;
const T_FUNCTION: u8 = LuauBytecodeType::LBC_TYPE_FUNCTION.0 as u8;
const T_VECTOR: u8 = LuauBytecodeType::LBC_TYPE_VECTOR.0 as u8;
const T_OPTIONAL: u8 = LuauBytecodeType::LBC_TYPE_OPTIONAL_BIT.0 as u8;

/// 遍历字节码推导各 pc 的类型 tag。
///
/// 只读前置条件（与 C++ 参考实现一致）：`proto` 有效（入口 assert）、`code`/`pc` 界内
/// （startpc/finishpc 来自合法 CFG）、`k[]` 常量下标来自字节码 AUX 字段、hook 指针已判非空。
/// 字节码与常量表经 `proto_views` 的安全切片视图读取，本函数内不再有裸指针游走。
pub(crate) fn analyze_bytecode_types(function: &mut IrFunction, host_hooks: &HostIrHooks) {
  let Some(proto) = function.proto_view() else {
    // cpp 在此直接解引用 `function.proto`（入口 CODEGEN_ASSERT 断言非空）；视图缺省即
    // 契约被破坏，按「无字节码可分析」早退，不写任何类型信息。
    CODEGEN_ASSERT!(false, "analyzeBytecodeTypes requires a proto");
    return;
  };

  let bc_type_info = &mut function.bc_type_info;
  prepare_reg_type_info_lookups(bc_type_info);

  let mut reg_tags = [T_ANY; 256];

  let numparams = proto.numparams;
  let maxstacksize = proto.maxstacksize;
  let code = code(proto);
  function
    .bc_types
    .resize(code.len(), BytecodeTypes::default());

  // BytecodeBlock 是 Copy 且循环体只写 bc_type_info（字段不相交），按引用迭代免整表 clone
  for block in &function.bc_blocks {
    CODEGEN_ASSERT!(block.startpc != -1);
    CODEGEN_ASSERT!(block.finishpc != -1);

    for (i, et) in bc_type_info.argument_types.iter().copied().enumerate() {
      reg_tags[i] = et & !(T_OPTIONAL);
    }

    for i in numparams..maxstacksize {
      reg_tags[i as usize] = T_ANY;
    }

    let mut known_next_call_result = T_ANY;
    let mut i = block.startpc;

    while i <= block.finishpc {
      let idx = i as usize;
      // 字节码字经切片索引读取，越界即 panic（C++ 为 UB），合法序列内恒界内
      let insn = code[idx];
      let op = LuauOpcode::from(luau_insn_op(insn) as u8);

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
        LuauOpcode::LOP_LOADK | LuauOpcode::LOP_LOADKX => {
          let ra = luau_insn_a(insn) as usize;
          let kb = if op == LuauOpcode::LOP_LOADKX {
            code[idx + 1]
          } else {
            luau_insn_d(insn) as u32
          };
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
        LuauOpcode::LOP_GETTABLE | LuauOpcode::LOP_AND | LuauOpcode::LOP_OR => {
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
          // 双字指令：aux/raw 字快照读一次，按 opcode 解码
          let aux_word = code[idx + 1];
          let kc = if op == LuauOpcode::LOP_GETUDATAKS {
            luau_insn_aux_kv16(aux_word)
          } else {
            aux_word
          };

          bc_type.a = get_reg_tag(&mut reg_tags, bc_type_info, rb, i);
          bc_type.b = get_bytecode_constant_tag(proto, kc);

          reg_tags[ra] = T_ANY;

          // GETTABLEKS/GETUDATAKS 的 `kc` 由字节码结构保证为字段名字符串常量在 `k[]` 的合法下标。
          let name = string_constant(proto, kc as usize);
          // 宿主 hook 是 `(*const c_char, size_t)` 契约：视图首址即 `getstr` 地址，
          // VM 在 `len` 处保留 NUL 终止符，故指针语义与原 cpp 传参一致。
          let field = name.as_ptr().cast::<c_char>();

          if bc_type.a == T_VECTOR {
            // cpp: `len == 1` 的单字段名（x/y/z 分量）
            if name.len() == 1 && matches!(name[0] | b' ', b'x' | b'y' | b'z') {
              reg_tags[ra] = T_NUMBER;
            }

            if reg_tags[ra] == T_ANY
              && let Some(hook) = host_hooks.vector_access_bytecode_type
            {
              // Safety: `hook` 取自 `&HostIrHooks` 的 Option 函数指针(Some 已判定);`field/len` 为 k[kc] 字符串
              // 有效字节区间,符合宿主 hook 的入参约定(纯读、返回类型标签)。
              reg_tags[ra] = unsafe { hook(field, name.len()) };
            }
          } else if is_custom_userdata_bytecode_type(bc_type.a)
            && reg_tags[ra] == T_ANY
            && let Some(hook) = host_hooks.userdata_access_bytecode_type
          {
            // Safety: 同 vector 分支——`hook` 来自 `&HostIrHooks` 且为 Some,`bc_type.a/field/len` 均为有效入参。
            reg_tags[ra] = unsafe { hook(bc_type.a, field, name.len()) };
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
        LuauOpcode::LOP_ADD
        | LuauOpcode::LOP_SUB
        | LuauOpcode::LOP_MUL
        | LuauOpcode::LOP_DIV
        | LuauOpcode::LOP_IDIV
        | LuauOpcode::LOP_MOD
        | LuauOpcode::LOP_POW => {
          let ra = luau_insn_a(insn) as usize;
          let rb = luau_insn_b(insn) as u8;
          let rc = luau_insn_c(insn) as u8;
          bc_type.a = get_reg_tag(&mut reg_tags, bc_type_info, rb, i);
          bc_type.b = get_reg_tag(&mut reg_tags, bc_type_info, rc, i);
          reg_tags[ra] = binary_type_calc(host_hooks, bc_type.a, bc_type.b, op);
          bc_type.result = reg_tags[ra];
        }
        LuauOpcode::LOP_ADDK
        | LuauOpcode::LOP_SUBK
        | LuauOpcode::LOP_MULK
        | LuauOpcode::LOP_DIVK
        | LuauOpcode::LOP_IDIVK
        | LuauOpcode::LOP_MODK
        | LuauOpcode::LOP_POWK => {
          let ra = luau_insn_a(insn) as usize;
          let rb = luau_insn_b(insn) as u8;
          let kc = luau_insn_c(insn);
          bc_type.a = get_reg_tag(&mut reg_tags, bc_type_info, rb, i);
          bc_type.b = get_bytecode_constant_tag(proto, kc);
          reg_tags[ra] = binary_type_calc(host_hooks, bc_type.a, bc_type.b, op);
          bc_type.result = reg_tags[ra];
        }
        LuauOpcode::LOP_SUBRK | LuauOpcode::LOP_DIVRK => {
          let ra = luau_insn_a(insn) as usize;
          let kb = luau_insn_b(insn);
          let rc = luau_insn_c(insn) as u8;
          bc_type.a = get_bytecode_constant_tag(proto, kb);
          bc_type.b = get_reg_tag(&mut reg_tags, bc_type_info, rc, i);
          reg_tags[ra] = binary_type_calc(host_hooks, bc_type.a, bc_type.b, op);
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
        LuauOpcode::LOP_NEWTABLE
        | LuauOpcode::LOP_DUPTABLE
        | LuauOpcode::LOP_CONCAT
        | LuauOpcode::LOP_NEWCLOSURE
        | LuauOpcode::LOP_DUPCLOSURE => {
          let ra = luau_insn_a(insn) as usize;
          reg_tags[ra] = match op {
            LuauOpcode::LOP_CONCAT => T_STRING,
            LuauOpcode::LOP_NEWCLOSURE | LuauOpcode::LOP_DUPCLOSURE => T_FUNCTION,
            _ => T_TABLE,
          };
          bc_type.result = reg_tags[ra];
        }
        LuauOpcode::LOP_FASTCALL
        | LuauOpcode::LOP_FASTCALL1
        | LuauOpcode::LOP_FASTCALL2
        | LuauOpcode::LOP_FASTCALL2K
        | LuauOpcode::LOP_FASTCALL3 => {
          let bfid = luau_insn_a(insn) as u8;
          let skip = luau_insn_c(insn) as i32;
          let call = code[idx + skip as usize + 1];
          CODEGEN_ASSERT!(LuauOpcode::from(luau_insn_op(call) as u8) == LuauOpcode::LOP_CALL);
          let ra = luau_insn_a(call) as usize;
          apply_builtin_call(builtin_function(bfid), &mut bc_type);
          match op {
            LuauOpcode::LOP_FASTCALL => {
              reg_tags[ra + 1] = bc_type.a;
              reg_tags[ra + 2] = bc_type.b;
              reg_tags[ra + 3] = bc_type.c;
            }
            LuauOpcode::LOP_FASTCALL1 | LuauOpcode::LOP_FASTCALL2K => {
              reg_tags[luau_insn_b(insn) as usize] = bc_type.a;
            }
            LuauOpcode::LOP_FASTCALL2 => {
              reg_tags[luau_insn_b(insn) as usize] = bc_type.a;
              reg_tags[code[idx + 1] as usize] = bc_type.b;
            }
            LuauOpcode::LOP_FASTCALL3 => {
              let aux = code[idx + 1];
              reg_tags[luau_insn_b(insn) as usize] = bc_type.a;
              reg_tags[luau_insn_aux_a(aux) as usize] = bc_type.b;
              reg_tags[luau_insn_aux_b(aux) as usize] = bc_type.c;
            }
            _ => {}
          }
          reg_tags[ra] = bc_type.result;
          refine_reg_type(bc_type_info, ra as u8, i, bc_type.result);
          i += skip;
        }
        LuauOpcode::LOP_FORNPREP | LuauOpcode::LOP_FORNLOOP => {
          let ra = luau_insn_a(insn) as usize;
          reg_tags[ra] = T_NUMBER;
          reg_tags[ra + 1] = T_NUMBER;
          reg_tags[ra + 2] = T_NUMBER;
          if op == LuauOpcode::LOP_FORNPREP {
            refine_reg_type(bc_type_info, ra as u8, i, T_NUMBER);
            refine_reg_type(bc_type_info, (ra + 1) as u8, i, T_NUMBER);
            refine_reg_type(bc_type_info, (ra + 2) as u8, i, T_NUMBER);
          }
        }
        LuauOpcode::LOP_NAMECALL | LuauOpcode::LOP_NAMECALLUDATA => {
          let ra = luau_insn_a(insn) as usize;
          let rb = luau_insn_b(insn) as u8;
          // 双字指令：aux/raw 字快照读一次，按 opcode 解码
          let aux_word = code[idx + 1];
          let kc = if op == LuauOpcode::LOP_NAMECALLUDATA {
            luau_insn_aux_kv16(aux_word)
          } else {
            aux_word
          };
          bc_type.a = get_reg_tag(&mut reg_tags, bc_type_info, rb, i);
          bc_type.b = get_bytecode_constant_tag(proto, kc);
          reg_tags[ra] = T_FUNCTION;
          reg_tags[ra + 1] = bc_type.a;
          bc_type.result = T_FUNCTION;

          // NAMECALL 的 `kc` 为字段名字符串常量在 `k[]` 的合法下标（字节码结构保证）。
          let name = string_constant(proto, kc as usize);
          // 宿主 hook 的 `(*const c_char, size_t)` 契约：视图首址即 `getstr` 地址，NUL 终止符
          // 仍在 `len` 处可读，与原 cpp 传参语义一致。
          let field = name.as_ptr().cast::<c_char>();
          if bc_type.a == T_VECTOR {
            if let Some(hook) = host_hooks.vector_namecall_bytecode_type {
              // Safety: `hook` 为 `&HostIrHooks` 中 Some 的函数指针;`field/len` 是 k[kc] 字符串有效字节区间。
              known_next_call_result = unsafe { hook(field, name.len()) };
            }
          } else if is_custom_userdata_bytecode_type(bc_type.a)
            && let Some(hook) = host_hooks.userdata_namecall_bytecode_type
          {
            // Safety: 同上——`hook` 来自 `&HostIrHooks` 且为 Some,`bc_type.a/field/len` 均为有效入参。
            known_next_call_result = unsafe { hook(bc_type.a, field, name.len()) };
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
          let rb = code[idx + 1] as u8;
          bc_type.a = get_reg_tag(&mut reg_tags, bc_type_info, ra, i);
          bc_type.b = get_reg_tag(&mut reg_tags, bc_type_info, rb, i);
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
        // cpp BytecodeAnalysis.cpp:1502-1508：这三条免类型分析
        | LuauOpcode::LOP_NEWCLASS
        | LuauOpcode::LOP_NEWCLASSMEMBER
        | LuauOpcode::LOP_FASTPCALL => {}
        _ => CODEGEN_ASSERT!(false),
      }

      function.bc_types[i as usize] = bc_type;
      i += get_op_length(op);
    }
  }
}

/// 自定义 userdata 的 metamethod 类型回调：hook 已注册且任一操作数为自定义
/// userdata 时才调用，否则落 ANY（与 C++ getBytecodeType 系列语义一致）
#[inline]
fn userdata_hook_type(host_hooks: &HostIrHooks, a: u8, b: u8, method: HostMetamethod) -> u8 {
  if let Some(hook) = host_hooks.userdata_metamethod_bytecode_type
    && (is_custom_userdata_bytecode_type(a) || is_custom_userdata_bytecode_type(b))
  {
    // Safety: `hook` 为宿主在 `&HostIrHooks` 注册的 `extern "C-unwind"` 函数指针,已判定为 Some(非空);
    // a/b 为字节码类型标签、method 为受支持的 metamethod 枚举,参数均为按值传递,满足该回调入参约定。
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
    if matches!(b, T_NUMBER | T_VECTOR) {
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

fn binary_type_calc(host_hooks: &HostIrHooks, a: u8, b: u8, op: LuauOpcode) -> u8 {
  match op {
    LuauOpcode::LOP_ADD
    | LuauOpcode::LOP_SUB
    | LuauOpcode::LOP_ADDK
    | LuauOpcode::LOP_SUBK
    | LuauOpcode::LOP_SUBRK => binary_add_sub_type(host_hooks, a, b, op),
    LuauOpcode::LOP_MUL
    | LuauOpcode::LOP_DIV
    | LuauOpcode::LOP_IDIV
    | LuauOpcode::LOP_MULK
    | LuauOpcode::LOP_DIVK
    | LuauOpcode::LOP_IDIVK
    | LuauOpcode::LOP_DIVRK => binary_mul_div_type(host_hooks, a, b, op),
    LuauOpcode::LOP_MOD | LuauOpcode::LOP_POW | LuauOpcode::LOP_MODK | LuauOpcode::LOP_POWK => {
      binary_number_or_userdata_type(host_hooks, a, b, op)
    }
    _ => T_ANY,
  }
}
