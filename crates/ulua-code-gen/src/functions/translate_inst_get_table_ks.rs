use core::ffi::c_char;

use ulua_common::{
  enums::{luau_bytecode_type::LuauBytecodeType, luau_opcode::LuauOpcode},
  macros::luau_insn_ops::{luau_insn_a, luau_insn_aux_kv16, luau_insn_b, luau_insn_op},
};
use ulua_vm::enums::lua_type::LuaType;

use crate::{
  enums::ir_cmd::IrCmd,
  functions::{
    check_table_tag_guard::check_table_tag_guard, check_tag_exit::check_tag_exit,
    is_userdata_bytecode_type::is_userdata_bytecode_type, proto_views::string_constant,
  },
  records::{fallback_stream_scope::FallbackStreamScope, ir_builder::IrBuilder},
  type_aliases::instruction_ir_builder::Instruction,
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

      // Safety: `build.host_hooks` 为宿主注册的非空 `*const HostIrHooks`(fn 契约),对齐且比 `build` 长寿,取共享引用仅读表。
      let host_hooks = unsafe { &*build.host_hooks };
      if let Some(vector_access) = host_hooks.vector_access {
        // Safety: `vector_access` 是 fn 契约保证存在的宿主 hook;`build as *mut` 由 `&mut build` 转来非空/对齐;
        // `name` 视图首址即 getstr 地址（VM 在 len 处保留 NUL），ra/rb/pcpos 源自解码后的指令字,
        // 符合 hook 的 `(*const c_char, size_t)` 入参约定。
        let handled = unsafe {
          vector_access(
            build as *mut IrBuilder,
            name.as_ptr().cast::<c_char>(),
            name.len(),
            ra as i32,
            rb as i32,
            pcpos,
          )
        };
        if handled {
          return;
        }
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

      // Safety: `build.host_hooks` 为宿主注册的非空 `*const HostIrHooks`(fn 契约),对齐且比 `build` 长寿,取共享引用仅读表。
      let host_hooks = unsafe { &*build.host_hooks };
      if let Some(userdata_access) = host_hooks.userdata_access {
        // Safety: `userdata_access` 是 fn 契约保证存在的宿主 hook;`build as *mut` 由 `&mut build` 转来非空/对齐;
        // bc_types.a 为已读取的字节码类型,`name` 视图首址即 k[aux] 的 getstr 地址,其余源自指令字,
        // 符合 hook 的 `(*const c_char, size_t)` 入参约定。
        let handled = unsafe {
          userdata_access(
            build as *mut IrBuilder,
            bc_types.a,
            name.as_ptr().cast::<c_char>(),
            name.len(),
            ra as i32,
            rb as i32,
            pcpos,
          )
        };
        if handled {
          return;
        }
      }

      emit_fallback_table_ks(build, pcpos, ra, rb, aux, is_set);
      return;
    }
  }

  let fallback = build.fallback_block(pcpos as u32);

  let a_is_table = bc_types.a == LuauBytecodeType::LBC_TYPE_TABLE.0 as u8;
  check_table_tag_guard(build, tb, a_is_table, pcpos, fallback);

  let reg_rb = build.vm_reg(rb);
  let vb = build.inst_ir_cmd_ir_op(IrCmd::LoadPointer, reg_rb);

  let pcpos_op = build.const_uint(pcpos as u32);
  let aux_op = build.vm_const(aux);
  let addr_slot_el =
    build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::GetSlotNodeAddr, vb, pcpos_op, aux_op);

  let aux_op = build.vm_const(aux);
  build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckSlotMatch, addr_slot_el, aux_op, fallback);

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
  let scope = FallbackStreamScope::new(build, fallback, next);
  let build = &mut *scope.build;

  emit_fallback_table_ks(build, pcpos, ra, rb, aux, is_set);
  build.inst_ir_cmd_ir_op(IrCmd::JUMP, next);
}
