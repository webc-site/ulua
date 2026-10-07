use std::array::from_fn;

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
  records::{ir_builder::IrBuilder, ir_op::IrOp},
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
  // 命中收口单前驱 join 块：`index` 值经 CheckSlotMatch（check 边）流入链走块，
  // 而第二快路自身的 JUMP 终结点若直连多前驱块会触发线性化器
  // `get_live_out_value_count == 0` 前置断言（collect_direct_block_jump_path）；
  // 单前驱 join 使 try_create_linear_block 在断言前按 use_count==1 早退。
  let second_fast_path_hit_join = build.block(IrBlockKind::Internal);
  // 链走深度参数化（J4r-step2：cpp 内联前两级 → 扩深两级）：inherit3 形态的
  // describe/breathe 住在第 4/5 跳表，固定两跳全落 FallbackNamecall helper
  // （samply 实测 probe helper 占 inherit3 jit 22.4%）。EXTRA=2 即覆盖到第 5 跳
  // 表探测，更深链（现实继承不出现）仍落 helper 兜底。
  const CHAIN_EXTRA_HOPS: usize = 2;
  let index_chain_absent = build.block(IrBlockKind::Internal);
  let index_chain_probe = build.block(IrBlockKind::Internal);
  let extra_absent: [IrOp; CHAIN_EXTRA_HOPS] = from_fn(|_| build.block(IrBlockKind::Internal));
  let extra_probe: [IrOp; CHAIN_EXTRA_HOPS] = from_fn(|_| build.block(IrBlockKind::Internal));
  // 取值收口单前驱 join（second_fast_path_hit_join 同形）：probe 块以
  // `JUMP join`（目标 use_count==1）收尾而非直连多前驱的 next，避免
  // collect_direct_block_jump_path 对携带跨块活值（cur）的块触发
  // get_live_out_value_count == 0 前置断言。
  let probe0_join = build.block(IrBlockKind::Internal);
  let extra_probe_join: [IrOp; CHAIN_EXTRA_HOPS] = from_fn(|_| build.block(IrBlockKind::Internal));

  let exit_or_fallback = if bc_types.a == LuauBytecodeType::LBC_TYPE_TABLE.0 as u8 {
    build.vm_exit(pcpos as u32)
  } else {
    fallback
  };
  let reg_rb = build.vm_reg(rb);
  build.load_and_check_tag(reg_rb, LuaType::Table as u8, exit_or_fallback);
  let reg_rb = build.vm_reg(rb);
  let table = build.inst_ir_cmd_ir_op(IrCmd::LoadPointer, reg_rb);

  CODEGEN_ASSERT!(build.function.proto.is_some());
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

  // self（ra+1 = 接收者）先于一切探测落位：cpp `executeNAMECALL` 的全部分支
  // （含慢路径 helper）都先写 `ra+1 = rb`，接收者在 fasttm/helper 调用前驻留
  // 该槽是既有语义（值幂等，fallback 重写同值）。提前到分支前同时使各后继块
  // 的 vararg 序列起点一致：ra+1 在分叉前已被本块定义，
  // require_variadic_sequence 的起点剥离只发生在本块入口，不会在后继合并处
  // 产生起点分叉（表现为 compute_cfg_live_in_out_reg_sets 的起点一致断言）。
  let reg_self_early = build.vm_reg(ra.wrapping_add(1));
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StorePointer, reg_self_early, table);
  let reg_self_early = build.vm_reg(ra.wrapping_add(1));
  build.store_tag(reg_self_early, LuaType::Table as u8);

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
  // C 提示槽不匹配不直接落 helper：先沿 `__index` 链走第二跳（oop 形态
  // `a → Vec → Base`：方法在 Vec.__index = Vec 自身缺席，住在 Base）。
  build.inst_ir_cmd_ir_op_ir_op_ir_op(
    IrCmd::CheckSlotMatch,
    addr_index_node_el,
    aux_op,
    index_chain_absent,
  );

  let zero = build.const_int(0);
  let index_node_el = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadTvalue, addr_index_node_el, zero);
  let reg_ra = build.vm_reg(ra);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, reg_ra, index_node_el);
  build.inst_ir_cmd_ir_op(IrCmd::JUMP, second_fast_path_hit_join);

  build.begin_block(second_fast_path_hit_join);
  build.inst_ir_cmd_ir_op(IrCmd::JUMP, next);

  // __index 链第二跳（cpp `luaV_gettable` MAXTAGLOOP 链式行走的内联前两级）：
  // 此时 `index`（第一跳 `__index` 表）的 C 提示槽探测未命中。helper 全路径在
  // `index` 内缺席时才继续沿 `index` 自己的元表 `__index` 行走——内联必须先
  // 逐位证明这一缺席，否则同键双表（如 `new` 同在 Vec 与 Base）会错取深槽值。
  build.begin_block(index_chain_absent);

  // 缺席证明（luaH_getstr 只扫主位 + next 链）：
  // ① 主位节点无碰撞链（有链即放弃内联，helper 裁决）；
  // ② 主位键 ≠ k[aux]。CheckSlotMatch 的「命中」= 键等且值非 nil：主位持键且值非 nil
  //   时不得取更深的槽（helper 会返回 index 的值）；主位持键但值 nil（`t[k]=nil`
  //   留键）时继续，与 helper 的 `ttisnil(res)` 继续行走语义一致。
  let hash_op = build.const_uint(hash);
  let main_node_el = build.inst_ir_cmd_ir_op_ir_op(IrCmd::GetHashNodeAddr, index, hash_op);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::CheckNodeNoNext, main_node_el, fallback);
  let aux_op = build.vm_const(aux);
  build.inst_ir_cmd_ir_op_ir_op_ir_op(
    IrCmd::CheckSlotMatch,
    main_node_el,
    aux_op,
    index_chain_probe,
  );
  // 主位键等且值非 nil = index 持有该键：结果必须是 index 的值 → helper 裁决。
  build.inst_ir_cmd_ir_op(IrCmd::JUMP, fallback);

  // 缺席成立：沿 `index` 自己元表的 `__index`（fasttm 链与 helper 一致，含
  // 无元表/tmcache 缺席/非表三分支全部回退）探测 C 提示槽。
  build.begin_block(index_chain_probe);
  let tm_index2 = build.const_int(TMS::TmIndex as i32);
  let next_index_ptr =
    build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::TryCallFastgettm, index, tm_index2, fallback);
  build.load_and_check_tag(next_index_ptr, LuaType::Table as u8, fallback);
  let next_index = build.inst_ir_cmd_ir_op(IrCmd::LoadPointer, next_index_ptr);

  let pcpos_op = build.const_uint(pcpos as u32);
  let aux_op = build.vm_const(aux);
  let addr_next_node_el =
    build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::GetSlotNodeAddr, next_index, pcpos_op, aux_op);
  let aux_op = build.vm_const(aux);
  build.inst_ir_cmd_ir_op_ir_op_ir_op(
    IrCmd::CheckSlotMatch,
    addr_next_node_el,
    aux_op,
    extra_absent[0],
  );

  let zero = build.const_int(0);
  let next_node_el = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadTvalue, addr_next_node_el, zero);
  let reg_ra = build.vm_reg(ra);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, reg_ra, next_node_el);
  build.inst_ir_cmd_ir_op(IrCmd::JUMP, probe0_join);

  // 取值收口：单前驱（probe[0] 的顺序边），无跨块活值
  build.begin_block(probe0_join);
  build.inst_ir_cmd_ir_op(IrCmd::JUMP, next);

  // 第 4..(3+EXTRA) 跳：与 absent[0]→probe[0] 逐位同构的缺席证明 + 探测循环，
  // 仅「探测不匹配的去向」不同——末级落 fallback，中间级落下一组 absent 块。
  // cur 恒为上一层探测块的 `__index` 表（下一跳的走查对象）。
  let mut cur = next_index;
  for i in 0..CHAIN_EXTRA_HOPS {
    build.begin_block(extra_absent[i]);

    // 缺席证明（同 absent[0]，逐字节同构）：主位无碰撞链 + 主位键 ≠ k[aux]
    let hash_op = build.const_uint(hash);
    let main_node_el = build.inst_ir_cmd_ir_op_ir_op(IrCmd::GetHashNodeAddr, cur, hash_op);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::CheckNodeNoNext, main_node_el, fallback);
    let aux_op = build.vm_const(aux);
    build.inst_ir_cmd_ir_op_ir_op_ir_op(
      IrCmd::CheckSlotMatch,
      main_node_el,
      aux_op,
      extra_probe[i],
    );
    // 主位持键且值非 nil = 本层持有该键 → helper 裁决（不得取更深的槽）。
    build.inst_ir_cmd_ir_op(IrCmd::JUMP, fallback);

    // 缺席成立：沿本层元表 `__index` 探测 C 提示槽（第 i+5 跳表）。
    build.begin_block(extra_probe[i]);
    let tm_index = build.const_int(TMS::TmIndex as i32);
    let next_index_ptr =
      build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::TryCallFastgettm, cur, tm_index, fallback);
    build.load_and_check_tag(next_index_ptr, LuaType::Table as u8, fallback);
    cur = build.inst_ir_cmd_ir_op(IrCmd::LoadPointer, next_index_ptr);

    let pcpos_op = build.const_uint(pcpos as u32);
    let aux_op = build.vm_const(aux);
    let addr_next_node_el =
      build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::GetSlotNodeAddr, cur, pcpos_op, aux_op);
    let aux_op = build.vm_const(aux);
    let miss_target = if i + 1 < CHAIN_EXTRA_HOPS {
      extra_absent[i + 1]
    } else {
      fallback
    };
    build.inst_ir_cmd_ir_op_ir_op_ir_op(
      IrCmd::CheckSlotMatch,
      addr_next_node_el,
      aux_op,
      miss_target,
    );

    let zero = build.const_int(0);
    let next_node_el = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadTvalue, addr_next_node_el, zero);
    let reg_ra = build.vm_reg(ra);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, reg_ra, next_node_el);
    build.inst_ir_cmd_ir_op(IrCmd::JUMP, extra_probe_join[i]);

    // 取值收口：单前驱（probe[i] 的顺序边），携带值已在栈槽，无跨块活值
    build.begin_block(extra_probe_join[i]);
    build.inst_ir_cmd_ir_op(IrCmd::JUMP, next);
  }

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
