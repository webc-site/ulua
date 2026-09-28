use ulua_common::{
  enums::{luau_bytecode_type::LuauBytecodeType, luau_opcode::LuauOpcode},
  functions::get_op_length::get_op_length,
  macros::luau_insn_ops::{
    luau_insn_a, luau_insn_aux_kv16, luau_insn_b, luau_insn_c, luau_insn_op,
  },
};
use ulua_vm::enums::{lua_type::LuaType, tms::TMS};

use crate::{
  enums::{ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
  functions::{
    bytecode_types::is_userdata_bytecode_type,
    proto_views::{string_constant, string_constant_hash},
  },
  macros::codegen_assert::CODEGEN_ASSERT,
  records::ir_builder::IrBuilder,
  type_aliases::ir::Instruction,
};

/// 翻译 LOP_NAMECALL / LOP_NAMECALLUDATA。
///
/// 前置约定（字节码流以切片访问，越界即 panic，不构成 UB）：
/// 内部 unsafe 读取的统一前置条件（与 C++ 参考实现一致）：
/// `code` 切片于 `pcpos` 处指向一条 `LOP_NAMECALL`/`LOP_NAMECALLUDATA` 指令且界内，
/// 其后依次为 aux 字与 `LOP_CALL`/`LOP_CALLFB` 指令（字节码结构保证）；
/// `k[..=aux]` 为 GC 字符串常量；`build.host_hooks` 为宿主注册的非空函数指针表。
pub fn translate_inst_namecall(build: &mut IrBuilder, code: &[Instruction], pcpos: i32) -> bool {
  // 快照读：namecall 为双字指令，当前指令读一次，aux 字紧随其后
  //（cpp IrTranslation.cpp:2015：pc[1] 无条件读取；NAMECALLUDATA 取 AUX_KV16 域，否则取原字）
  // 界内约定:  fn 契约保证 `code` 切片于 `pcpos` 处指向界内的当前 NAMECALL 指令,读取字对齐且在界内。
  let insn = code[pcpos as usize];
  // 界内约定:  契约保证 namecall 为双字指令,`code[pcpos+1]` 即其 aux 字,仍落在 code 缓冲界内。
  let aux_insn = code[pcpos as usize + 1];
  let ra = luau_insn_a(insn) as u8;
  let rb = luau_insn_b(insn) as u8;
  let aux = if LuauOpcode::from(luau_insn_op(insn) as u8) == LuauOpcode::LOP_NAMECALLUDATA {
    luau_insn_aux_kv16(aux_insn)
  } else {
    aux_insn
  } as u32;

  let bc_types = build.function.get_bytecode_types_at(pcpos);

  if bc_types.a == LuauBytecodeType::LBC_TYPE_VECTOR.0 as u8 {
    let reg_rb = build.vm_reg(rb);
    let exit = build.vm_exit(pcpos as u32);
    build.load_and_check_tag(reg_rb, LuaType::Vector as u8, exit);

    // 宿主 hook 结构体快照：一次解引用后安全取字段（与 get_table_ks 手法一致）
    // 视图重建与安全论证单源于 `IrBuilder::host_hooks_ref`。
    let host_hooks = build.host_hooks_ref();
    if let Some(vector_namecall) = host_hooks.vector_namecall {
      // cpp pc[2]：namecall 之后按字节码结构必为 CALL/CALLFB 指令
      // Safety: 按字节码结构 namecall 之后必为 CALL/CALLFB,`code[pcpos+2]` 界内;opcode 由下一行 CODEGEN_ASSERT 复核。
      let call = code[pcpos as usize + 2];
      let call_op = LuauOpcode::from(luau_insn_op(call) as u8);
      CODEGEN_ASSERT!(matches!(
        call_op,
        LuauOpcode::LOP_CALL | LuauOpcode::LOP_CALLFB
      ));

      let callra = luau_insn_a(call) as i32;
      let nparams = luau_insn_b(call) as i32 - 1;
      let nresults = luau_insn_c(call) as i32 - 1;

      // k[aux] 字符串常量经 proto_views 的只读视图取回（cpp: getstr(gco2ts(k[aux].value.gc)), str->len）；
      // 原型经 `proto_view` 安全借用，空 proto 短路为「未命中」回退通用路径（生产中由 fn 契约保证非空）。
      let name = build
        .function
        .proto_view()
        .map_or(&[][..], |proto| string_constant(proto, aux as usize));

      if vector_namecall(build, name, callra, rb as i32, nparams, nresults, pcpos) {
        return true;
      }
    }

    let pcpos_op = build.const_uint(pcpos as u32);
    let reg_ra = build.vm_reg(ra);
    let reg_rb = build.vm_reg(rb);
    let aux_op = build.vm_const(aux);
    build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(
      IrCmd::FallbackNamecall,
      pcpos_op,
      reg_ra,
      reg_rb,
      aux_op,
    );
    return false;
  }

  if is_userdata_bytecode_type(bc_types.a) {
    let reg_rb = build.vm_reg(rb);
    let exit = build.vm_exit(pcpos as u32);
    build.load_and_check_tag(reg_rb, LuaType::UserData as u8, exit);

    // cpp IrTranslation.cpp:2045-2063：userdata 命名调用先走宿主 hook，
    // 未处理再回退 FALLBACK_NAMECALL
    // 视图重建与安全论证单源于 `IrBuilder::host_hooks_ref`。
    let host_hooks = build.host_hooks_ref();
    if let Some(userdata_namecall) = host_hooks.userdata_namecall {
      // cpp pc[2]：namecall 之后按字节码结构必为 CALL/CALLFB 指令
      // Safety: 同 vector 分支——namecall 之后必为 CALL/CALLFB,`code[pcpos+2]` 界内,opcode 由下一行 CODEGEN_ASSERT 复核。
      let call = code[pcpos as usize + 2];
      let call_op = LuauOpcode::from(luau_insn_op(call) as u8);
      CODEGEN_ASSERT!(matches!(
        call_op,
        LuauOpcode::LOP_CALL | LuauOpcode::LOP_CALLFB
      ));

      let callra = luau_insn_a(call) as i32;
      let nparams = luau_insn_b(call) as i32 - 1;
      let nresults = luau_insn_c(call) as i32 - 1;

      // k[aux] 字符串常量经 proto_views 的只读视图取回（cpp: getstr(gco2ts(k[aux].value.gc)), str->len）；
      // 原型经 `proto_view` 安全借用，空 proto 短路为「未命中」回退通用路径（生产中由 fn 契约保证非空）。
      let name = build
        .function
        .proto_view()
        .map_or(&[][..], |proto| string_constant(proto, aux as usize));

      if userdata_namecall(
        build, bc_types.a, name, callra, rb as i32, nparams, nresults, pcpos,
      ) {
        return true;
      }
    }

    let pcpos_op = build.const_uint(pcpos as u32);
    let reg_ra = build.vm_reg(ra);
    let reg_rb = build.vm_reg(rb);
    let aux_op = build.vm_const(aux);
    build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(
      IrCmd::FallbackNamecall,
      pcpos_op,
      reg_ra,
      reg_rb,
      aux_op,
    );
    return false;
  }

  let next = build.block_at_inst((pcpos + get_op_length(LuauOpcode::LOP_NAMECALL)) as u32);
  let fallback = build.fallback_block(pcpos as u32);
  let first_fast_path_success = build.block(IrBlockKind::Internal);
  let second_fast_path = build.block(IrBlockKind::Internal);

  let exit_or_fallback = if bc_types.a == LuauBytecodeType::LBC_TYPE_TABLE.0 as u8 {
    build.vm_exit(pcpos as u32)
  } else {
    fallback
  };
  let reg_rb = build.vm_reg(rb);
  build.load_and_check_tag(reg_rb, LuaType::Table as u8, exit_or_fallback);
  let reg_rb = build.vm_reg(rb);
  let table = build.inst_ir_cmd_ir_op(IrCmd::LoadPointer, reg_rb);

  CODEGEN_ASSERT!(!build.function.proto.is_null());
  // cpp: tsvalue(&build.function.proto->k[aux])->hash（哈希读取收口在 proto_views；
  // 原型经 `proto_view` 安全借用，空 proto 短路为 0——上一行断言保证生产中不可达）。
  let hash = build
    .function
    .proto_view()
    .map(|proto| string_constant_hash(proto, aux as usize))
    .unwrap_or(0);
  let hash_op = build.const_uint(hash);
  let addr_node_el = build.inst_ir_cmd_ir_op_ir_op(IrCmd::GetHashNodeAddr, table, hash_op);

  let aux_op = build.vm_const(aux);
  build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(
    IrCmd::JumpSlotMatch,
    addr_node_el,
    aux_op,
    first_fast_path_success,
    second_fast_path,
  );

  build.begin_block(first_fast_path_success);
  let reg_self = build.vm_reg(ra.wrapping_add(1));
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StorePointer, reg_self, table);
  let reg_self = build.vm_reg(ra.wrapping_add(1));
  build.store_tag(reg_self, LuaType::Table as u8);

  let offset_val = build.const_int(0);
  let node_el = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadTvalue, addr_node_el, offset_val);
  let reg_ra = build.vm_reg(ra);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, reg_ra, node_el);
  build.inst_ir_cmd_ir_op(IrCmd::JUMP, next);

  build.begin_block(second_fast_path);

  build.inst_ir_cmd_ir_op_ir_op(IrCmd::CheckNodeNoNext, addr_node_el, fallback);

  let tm_index = build.const_int(TMS::TmIndex as i32);
  let index_ptr =
    build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::TryCallFastgettm, table, tm_index, fallback);

  build.load_and_check_tag(index_ptr, LuaType::Table as u8, fallback);
  let index = build.inst_ir_cmd_ir_op(IrCmd::LoadPointer, index_ptr);

  let pcpos_op = build.const_uint(pcpos as u32);
  let aux_op = build.vm_const(aux);
  let addr_index_node_el =
    build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::GetSlotNodeAddr, index, pcpos_op, aux_op);
  let aux_op = build.vm_const(aux);
  build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckSlotMatch, addr_index_node_el, aux_op, fallback);

  let reg_rb = build.vm_reg(rb);
  let table2 = build.inst_ir_cmd_ir_op(IrCmd::LoadPointer, reg_rb);
  let reg_self = build.vm_reg(ra.wrapping_add(1));
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StorePointer, reg_self, table2);
  let reg_self = build.vm_reg(ra.wrapping_add(1));
  build.store_tag(reg_self, LuaType::Table as u8);

  let zero = build.const_int(0);
  let index_node_el = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadTvalue, addr_index_node_el, zero);
  let reg_ra = build.vm_reg(ra);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, reg_ra, index_node_el);
  build.inst_ir_cmd_ir_op(IrCmd::JUMP, next);

  build.begin_block(fallback);
  let pcpos_op = build.const_uint(pcpos as u32);
  let reg_ra = build.vm_reg(ra);
  let reg_rb = build.vm_reg(rb);
  let aux_op = build.vm_const(aux);
  build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(
    IrCmd::FallbackNamecall,
    pcpos_op,
    reg_ra,
    reg_rb,
    aux_op,
  );
  build.inst_ir_cmd_ir_op(IrCmd::JUMP, next);

  build.begin_block(next);

  false
}
