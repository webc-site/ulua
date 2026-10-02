use ulua_common::{
  enums::{luau_bytecode_type::LuauBytecodeType, luau_opcode::LuauOpcode},
  macros::luau_insn_ops::{luau_insn_a, luau_insn_aux_kv16, luau_insn_b, luau_insn_op},
};
use ulua_vm::enums::lua_type::LuaType;

use crate::{
  enums::ir_cmd::IrCmd,
  functions::{
    bytecode_types::is_userdata_bytecode_type,
    check_table_tag_guard::check_table_tag_guard,
    emit::x_64::check_tag_exit,
    proto_views::{string_constant, string_constant_hash},
  },
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{fallback_stream_scope::FallbackStreamScope, ir_builder::IrBuilder},
  type_aliases::ir::Instruction,
};

/// f32 向量分量的字节宽度（x/y/z 依次偏移 0/4/8）
const K_VECTOR_COMPONENT_SIZE: i32 = 4;

/// 向量分量直读：生成 LoadFloat + FloatToNum + StoreDouble/StoreTag
fn emit_vector_component_load(build: &mut IrBuilder, ra: u8, rb: u8, component: i32) {
  let reg_rb = build.vm_reg(rb);
  let offset = build.const_int(component * K_VECTOR_COMPONENT_SIZE);
  let value = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadFloat, reg_rb, offset);
  let value = build.inst_ir_cmd_ir_op(IrCmd::FloatToNum, value);
  let reg_ra = build.vm_reg(ra);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, reg_ra, value);
  let reg_ra = build.vm_reg(ra);
  build.store_tag(reg_ra, LuaType::Number as u8);
}

/// 尾回退：发射 FallbackGettableks/FallbackSettableks（`is_set` 选写侧）。
fn emit_fallback_table_ks(
  build: &mut IrBuilder,
  pcpos: i32,
  ra: u8,
  rb: u8,
  aux: u32,
  is_set: bool,
) {
  let pcpos_op = build.const_uint(pcpos as u32);
  let reg_ra = build.vm_reg(ra);
  let reg_rb = build.vm_reg(rb);
  let aux_op = build.vm_const(aux);
  build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(
    if is_set {
      IrCmd::FallbackSettableks
    } else {
      IrCmd::FallbackGettableks
    },
    pcpos_op,
    reg_ra,
    reg_rb,
    aux_op,
  );
}

/// 翻译 LOP_GETTABLEKS / LOP_GETUDATAKS。
pub fn translate_inst_get_table_ks(build: &mut IrBuilder, code: &[Instruction], pcpos: i32) {
  translate_table_access_ks(build, code, pcpos, false);
}

/// 翻译 LOP_SETTABLEKS / LOP_SETUDATAKS。
pub fn translate_inst_set_table_ks(build: &mut IrBuilder, code: &[Instruction], pcpos: i32) {
  translate_table_access_ks(build, code, pcpos, true);
}

/// GETTABLEKS / SETTABLEKS 共用主体：`is_set` 取 false（读）/ true（写）。
pub(crate) fn translate_table_access_ks(
  build: &mut IrBuilder,
  code: &[Instruction],
  pcpos: i32,
  is_set: bool,
) {
  // 界内约定：fn 契约保证 `code` 切片于 `pcpos` 处指向界内的当前指令。
  let insn = code[pcpos as usize];
  let ra = luau_insn_a(insn) as u8;
  let rb = luau_insn_b(insn) as u8;

  let op = luau_insn_op(insn);
  let aux = if LuauOpcode::from(op as u8) == LuauOpcode::LOP_GETUDATAKS
    || LuauOpcode::from(op as u8) == LuauOpcode::LOP_SETUDATAKS
  {
    // Safety: GETUDATAKS/SETUDATAKS 为双字指令,`code[pcpos+1]` 即 aux 字,落在 code 缓冲界内;AUX_KV16 仅按位提取。
    luau_insn_aux_kv16(code[pcpos as usize + 1])
  } else {
    // Safety: GETTABLEKS/SETTABLEKS 同为双字指令,`code[pcpos+1]` 为常量下标 aux 字,在 code 缓冲界内。
    code[pcpos as usize + 1]
  } as u32;

  let bc_types = build.function.get_bytecode_types_at(pcpos);

  let reg_rb = build.vm_reg(rb);
  let tb = build.inst_ir_cmd_ir_op(IrCmd::LoadTag, reg_rb);

  if is_set {
    if is_userdata_bytecode_type(bc_types.a) {
      check_tag_exit(build, tb, LuaType::UserData as u8, pcpos);

      let pcpos_op = build.const_uint(pcpos as u32);
      let ra_op = build.vm_reg(ra);
      let rb_op = build.vm_reg(rb);
      let aux_op = build.vm_const(aux);
      build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(
        IrCmd::FallbackSettableks,
        pcpos_op,
        ra_op,
        rb_op,
        aux_op,
      );
      return;
    }
  } else {
    if bc_types.a == LuauBytecodeType::LBC_TYPE_VECTOR.0 as u8 {
      check_tag_exit(build, tb, LuaType::Vector as u8, pcpos);

      // `aux` 由 fn 契约保证为 k[aux] 字符串常量的合法下标；原型经 `proto_view` 安全借用，
      // 空 proto 短路为空视图（等价「识别不了」，回退后续通用路径）。
      let name = build
        .function
        .proto_view()
        .map_or(&[][..], |proto| string_constant(proto, aux as usize));

      // cpp: 仅单字符字段名（`len == 1`）才识别为向量分量
      let component = match name {
        [b'X' | b'x'] => Some(0),
        [b'Y' | b'y'] => Some(1),
        [b'Z' | b'z'] => Some(2),
        _ => None,
      };

      if let Some(component) = component {
        emit_vector_component_load(build, ra, rb, component);
        return;
      }

      // 视图重建与安全论证单源于 `IrBuilder::host_hooks_ref`。
      let vector_access = build.host_hooks_ref().vector_access;
      if let Some(vector_access) = vector_access
        && vector_access(build, name, ra as i32, rb as i32, pcpos)
      {
        return;
      }

      emit_fallback_table_ks(build, pcpos, ra, rb, aux, is_set);
      return;
    }

    if is_userdata_bytecode_type(bc_types.a) {
      check_tag_exit(build, tb, LuaType::UserData as u8, pcpos);

      // cpp IrTranslation.cpp:1850-1860：userdata 属性访问先走宿主 hook，
      // 未处理再回退 FALLBACK_GETTABLEKS
      // 原型经 `proto_view` 安全借用，空 proto 短路为空视图（等价「hook 未命中」）。
      let name = build
        .function
        .proto_view()
        .map_or(&[][..], |proto| string_constant(proto, aux as usize));

      // 视图重建与安全论证单源于 `IrBuilder::host_hooks_ref`。
      let userdata_access = build.host_hooks_ref().userdata_access;
      if let Some(userdata_access) = userdata_access
        && userdata_access(build, bc_types.a, name, ra as i32, rb as i32, pcpos)
      {
        return;
      }

      emit_fallback_table_ks(build, pcpos, ra, rb, aux, is_set);
      return;
    }
  }

  // J4b：SETTABLEKS 专属的 IC miss 块——内联主位空直插快路（cpp `luaH_newkey`
  // 主位空分支）。GETTABLEKS（读侧无插入语义）与 SETUDATAKS（aux 为 kv16 索引、
  // 非常量下标，取不到编译期 hash）不建插入块，miss 一律直落 helper。
  let insert_block = if is_set && LuauOpcode::from(op as u8) == LuauOpcode::LOP_SETTABLEKS {
    Some(build.fallback_block(pcpos as u32))
  } else {
    None
  };

  let fallback = build.fallback_block(pcpos as u32);

  let a_is_table = bc_types.a == LuauBytecodeType::LBC_TYPE_TABLE.0 as u8;
  check_table_tag_guard(build, tb, a_is_table, pcpos, fallback);

  let reg_rb = build.vm_reg(rb);
  let vb = build.inst_ir_cmd_ir_op(IrCmd::LoadPointer, reg_rb);

  let pcpos_op = build.const_uint(pcpos as u32);
  let aux_op = build.vm_const(aux);
  let addr_slot_el =
    build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::GetSlotNodeAddr, vb, pcpos_op, aux_op);

  // slot mismatch 时：SETTABLEKS 先试内联插入，其余站点直接落 helper 块
  let aux_op = build.vm_const(aux);
  build.inst_ir_cmd_ir_op_ir_op_ir_op(
    IrCmd::CheckSlotMatch,
    addr_slot_el,
    aux_op,
    insert_block.unwrap_or(fallback),
  );

  let reg_ra = build.vm_reg(ra);

  if is_set {
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::CheckReadonly, vb, fallback);
    let tva = build.inst_ir_cmd_ir_op(IrCmd::LoadTvalue, reg_ra);
    let offset = build.const_int(0);
    build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::StoreTvalue, addr_slot_el, tva, offset);

    let undef = build.undef();
    build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::BarrierTableForward, vb, reg_ra, undef);
  } else {
    let offsetof_val = build.const_int(0);
    let tvn = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadTvalue, addr_slot_el, offsetof_val);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, reg_ra, tvn);
  }

  let next = build.block_at_inst((pcpos + 2) as u32);

  if let Some(insert_block) = insert_block {
    let scope = FallbackStreamScope::new(build, insert_block, next);
    let build = &mut *scope.build;

    // 主位节点地址：cpp `mainposition(t, key)` = node + (hash & !(-1 << lsizenode))，
    // hash 为 `tsvalue(&proto->k[aux])->hash` 编译期常量（NAMECALL 同款取法）。
    let reg_rb = build.vm_reg(rb);
    let vb = build.inst_ir_cmd_ir_op(IrCmd::LoadPointer, reg_rb);

    CODEGEN_ASSERT!(build.function.proto.is_some());
    // 原型经 `proto_view` 安全借用，空 proto 短路为 0（同 NAMECALL，生产中不可达）。
    let hash = build
      .function
      .proto_view()
      .map(|proto| string_constant_hash(proto, aux as usize))
      .unwrap_or(0);
    let hash_op = build.const_uint(hash);
    let node = build.inst_ir_cmd_ir_op_ir_op(IrCmd::GetHashNodeAddr, vb, hash_op);

    // 前置守卫，任一失败转 helper 块（helper 全路径语义不变）：
    // ① 哨兵表（t->node == dummynode，插入需 rehash 换发实向量）或主位被占
    //   （val.tt != NIL，cpp 走 freepos/碰撞链）——cpp `luaH_newkey` 主位空分支前提；
    // ② cpp `luaH_setstr` 直插前提（lvmexecute.cpp:712）：
    //   fastnotm(metatable, TM_NEWINDEX)（有元表但无 __newindex 仍可直插，快判走
    //   metatable->tmcache 缺席位）且非 readonly。
    build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckNodeInsertable, node, vb, fallback);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::CheckNoNewindexMeta, vb, fallback);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::CheckReadonly, vb, fallback);

    // cpp `setnodekey`：key.value 拷自 k[aux].value、extra 清零、tt=LUA_TSTRING
    // （next 位随整字归零）——禁止 16B 整拷 k TValue（extra 语义不同）。
    // 附带 cpp `luaH_set` 的 `invalidateTMcache(t)`（ltable.cpp:1325——新键插入
    // 改变键集合，本表作为 metatable 被查询过的元方法缺席位必须作废；
    // luaH_setslot（ltable.h:37）连既有槽 set 也清，直插对齐 newkey 路径）。
    let kv = build.vm_const(aux);
    build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::StoreNodeKey, node, kv, vb);

    // cpp `setobj2t(gval(mp), ra)`：完整 TValue 16B 落位。
    let tva = build.inst_ir_cmd_ir_op(IrCmd::LoadTvalue, reg_ra);
    let offset = build.const_int(0);
    build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::StoreTvalue, node, tva, offset);

    // cpp `luaC_barriert(L, t, key)`：key 屏障（luaH_newkey 主位空直插分支，
    // ltable.cpp:1160——key 虽在 proto 常量表仍需屏障）。
    let undef = build.undef();
    build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::BarrierTableForward, vb, kv, undef);
    // cpp `luaC_barriert(L, h, ra)`：value 屏障（lvmexecute.cpp:714，与 IC 命中路径对称）。
    let reg_ra = build.vm_reg(ra);
    build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::BarrierTableForward, vb, reg_ra, undef);

    // 内联插入完成，跳过 helper。
    build.inst_ir_cmd_ir_op(IrCmd::JUMP, next);

    // helper 块：原 fallback 内容原样（SetSavedpc + helper 调用 + JUMP next）。
    build.begin_block(fallback);
    emit_fallback_table_ks(build, pcpos, ra, rb, aux, is_set);
    build.inst_ir_cmd_ir_op(IrCmd::JUMP, next);
  } else {
    let scope = FallbackStreamScope::new(build, fallback, next);
    let build = &mut *scope.build;

    emit_fallback_table_ks(build, pcpos, ra, rb, aux, is_set);
    build.inst_ir_cmd_ir_op(IrCmd::JUMP, next);
  }
}
