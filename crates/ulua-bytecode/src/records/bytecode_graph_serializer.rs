use core::mem::take;
use std::vec::Vec;

use ulua_common::{
  enums::{luau_capture_type::LuauCaptureType, luau_opcode::LuauOpcode},
  fflag::DebugLuauUserDefinedClasses,
  functions::{
    import_layout::{K_IMPORT_COMPONENT_MASK, K_IMPORT_COUNT_SHIFT, import_component_shift},
    is_jump_d::is_jump_d,
    is_skip_c::is_skip_c,
  },
  macros::luau_assert::{LUAU_ASSERT, LUAU_UNREACHABLE},
};

use crate::{
  enums::{
    bc_block_edge_kind::BcBlockEdgeKind, bc_block_flag::BcBlockFlag, bc_imm_kind::BcImmKind,
    bc_op_kind::BcOpKind,
  },
  macros::input_getter::define_input_getter,
  records::{
    bc_block::BcBlock,
    bc_function::BcFunction,
    bc_imm::BcImm,
    bc_jump::BcJump,
    bc_op::BcOp,
    bc_proj::BcProj,
    bytecode_builder::{
      BytecodeBuilder, K_INVALID_REG, K_MAX_CLOSURE_COUNT, K_MAX_CONSTANT_COUNT,
      K_MAX_UPVALUE_COUNT, insn,
    },
    jump_info::JumpInfo,
  },
  type_aliases::reg::Reg,
};

#[derive(Debug)]
pub(crate) struct BytecodeGraphSerializer<'a, 'b, 'f> {
  pub(crate) bcb: &'a mut BytecodeBuilder<'b>,
  pub(crate) func: &'a mut BcFunction<'f>,
  pub(crate) jumps: Vec<JumpInfo>,
  pub(crate) error: bool,
  pub(crate) consts: Option<Vec<u32>>,
}

impl<'a, 'b, 'f> BytecodeGraphSerializer<'a, 'b, 'f> {
  pub(crate) fn new(bcb: &'a mut BytecodeBuilder<'b>, func: &'a mut BcFunction<'f>) -> Self {
    Self {
      bcb,
      func,
      jumps: Vec::new(),
      error: false,
      consts: None,
    }
  }
}

// ── abs-r139：并自 `methods/bytecode_graph_serializer_emit_bytecode.rs` ──
impl<'a, 'b, 'f> BytecodeGraphSerializer<'a, 'b, 'f> {
  pub(crate) fn emit_bytecode(&mut self) -> Vec<u32> {
    let schedule = self.reschedule();
    // cpp 700：初始哨兵 ~0u，死块/未发射指令的 pc 映射保持无效值
    let mut insns_pc: Vec<u32> = vec![u32::MAX; self.func.instructions.len()];

    for (i, &block_op) in schedule.iter().enumerate() {
      let fallthrough = {
        let block: &BcBlock = &self.func.blocks[block_op.index as usize];
        block
          .successors
          .iter()
          .find(|edge| edge.kind == BcBlockEdgeKind::Fallthrough)
          .map(|edge| edge.target)
      };
      // cpp 707：fallthrough 落在死块上时不补跳转
      if let Some(fallthrough_op) = fallthrough
        && fallthrough_op != self.func.exit_block
        && (self.func.blocks[fallthrough_op.index as usize].flags & BcBlockFlag::Dead) == 0
        && schedule.get(i + 1) != Some(&fallthrough_op)
      {
        let mut jump = BcJump::create(self.func);
        jump.set_target(fallthrough_op);
        jump.append_to(block_op);
        // cpp 713：jump 追加导致指令数增长，新槽位按 resize 默认补 0
        insns_pc.resize(self.func.instructions.len(), 0);
      }
      let ops = {
        let block: &mut BcBlock = self.func.block_op(block_op);
        block.startpc = self.bcb.get_debug_pc();
        block.ops.iter().cloned().collect::<Vec<_>>()
      };
      for op in ops {
        LUAU_ASSERT!(op.kind == BcOpKind::Inst);
        insns_pc[op.index as usize] = self.bcb.get_debug_pc();
        self.emit_instruction(op);
      }
    }

    let mut jumps = take(&mut self.jumps);
    for jump in jumps.iter_mut() {
      self.patch_jump(jump);
    }
    self.jumps = jumps;

    if self.error { Vec::new() } else { insns_pc }
  }
}

// ── abs-r139：并自 `methods/bytecode_graph_serializer_emit_instruction.rs` ──
impl<'a, 'b, 'f> BytecodeGraphSerializer<'a, 'b, 'f> {
  pub(crate) fn emit_instruction(&mut self, insn_op: BcOp) {
    // op/line 为 Copy 字段先拷出；指令本体不再整体 clone：辅助方法按
    // insn_op 句柄现取操作数（emit 期间 func 只读，现取与快照语义等价），
    // 旧实现绕 `&mut self.func`/`&mut self.bcb` 借用冲突的 per-instruction
    // clone 随之消除（拆分思路同 `BytecodeBuilder::finalize`）。
    let (op, line) = {
      let insn = self.func.inst_op(insn_op);
      (insn.op, insn.line)
    };
    self.bcb.set_debug_line(line as i32);
    match op {
      LuauOpcode::LOP_NOP | LuauOpcode::LOP_BREAK | LuauOpcode::LOP_NATIVECALL => {
        self.bcb.emit_abc(op, 0, 0, 0);
      }
      LuauOpcode::LOP_FASTPCALL => {
        let imm_int_0 = self.get_imm_int(insn_op, 0);
        let imm_int_1 = self.get_imm_int(insn_op, 1);
        let imm_int_2 = self.get_imm_int(insn_op, 2);
        self.bcb.emit_abc(
          LuauOpcode::LOP_FASTPCALL,
          imm_int_0 as u8,
          imm_int_1 as u8,
          imm_int_2 as u8,
        );
      }
      LuauOpcode::LOP_NEWCLASS => {
        let out = self.get_register(insn_op);
        let has_super = self.func.inst_op(insn_op).ops[0].kind != BcOpKind::None;
        let b = if has_super {
          self.get_reg_input(insn_op, 0)
        } else {
          K_INVALID_REG as u8
        };
        let count = self.get_imm_import(insn_op, 1);
        self
          .bcb
          .emit_abc(LuauOpcode::LOP_NEWCLASS, out, b, count as u8);
        let const_idx = self.get_vm_const_input_aux(insn_op, 2);
        self.bcb.emit_aux(const_idx);
      }
      LuauOpcode::LOP_LOADNIL => {
        let out = self.get_register(insn_op);
        self.bcb.emit_abc(LuauOpcode::LOP_LOADNIL, out, 0, 0);
      }
      LuauOpcode::LOP_LOADB => {
        if self.func.inst_op(insn_op).ops.len() > 1 {
          self.record_jump(insn_op, 1);
        }
        let imm_bool = self.get_imm_bool(insn_op, 0);
        let out = self.get_register(insn_op);
        self
          .bcb
          .emit_abc(LuauOpcode::LOP_LOADB, out, imm_bool as u8, 0);
      }
      LuauOpcode::LOP_LOADN => {
        // cpp 339：getImmIntAsSignedD 超出 i16 时置 error
        let imm_int = self.get_imm_int_as_signed_d(insn_op, 0);
        let out = self.get_register(insn_op);
        self.bcb.emit_ad(LuauOpcode::LOP_LOADN, out, imm_int);
      }
      LuauOpcode::LOP_LOADK => {
        let vm_const_input_d = self.get_vm_const_input_d(insn_op, 0);
        let out = self.get_register(insn_op);
        self
          .bcb
          .emit_ad(LuauOpcode::LOP_LOADK, out, vm_const_input_d as i16);
      }
      LuauOpcode::LOP_MOVE => {
        let reg_input = self.get_reg_input(insn_op, 0);
        let out = self.get_register(insn_op);
        self.bcb.emit_abc(LuauOpcode::LOP_MOVE, out, reg_input, 0);
      }
      LuauOpcode::LOP_GETGLOBAL => {
        let imm_int = self.get_imm_int(insn_op, 0);
        let out = self.get_register(insn_op);
        self
          .bcb
          .emit_abc(LuauOpcode::LOP_GETGLOBAL, out, 0, imm_int as u8);
        let vm_const_input_aux = self.get_vm_const_input_aux(insn_op, 1);
        self.bcb.emit_aux(vm_const_input_aux);
      }
      LuauOpcode::LOP_SETGLOBAL => {
        let reg_input = self.get_reg_input(insn_op, 0);
        let imm_int = self.get_imm_int(insn_op, 1);
        self
          .bcb
          .emit_abc(LuauOpcode::LOP_SETGLOBAL, reg_input, 0, imm_int as u8);
        let vm_const_input_aux = self.get_vm_const_input_aux(insn_op, 2);
        self.bcb.emit_aux(vm_const_input_aux);
      }
      LuauOpcode::LOP_GETUPVAL => {
        let upval_input = self.get_upval_input(insn_op, 0);
        let out = self.get_register(insn_op);
        self
          .bcb
          .emit_abc(LuauOpcode::LOP_GETUPVAL, out, upval_input, 0);
      }
      LuauOpcode::LOP_SETUPVAL => {
        let reg_input = self.get_reg_input(insn_op, 0);
        let upval_input = self.get_upval_input(insn_op, 1);
        self
          .bcb
          .emit_abc(LuauOpcode::LOP_SETUPVAL, reg_input, upval_input, 0);
      }
      LuauOpcode::LOP_CLOSEUPVALS => {
        // cpp 368-369：与其它寄存器输入同走 getRegInput（含溢出检查）。
        let reg_input = self.get_reg_input(insn_op, 0);
        self
          .bcb
          .emit_abc(LuauOpcode::LOP_CLOSEUPVALS, reg_input, 0, 0);
      }
      LuauOpcode::LOP_GETIMPORT => {
        let vm_const_input_d = self.get_vm_const_input_d(insn_op, 0);
        let out = self.get_register(insn_op);
        self
          .bcb
          .emit_ad(LuauOpcode::LOP_GETIMPORT, out, vm_const_input_d as i16);
        // aux 高 2 位为组件数，其余每 10 位为一个组件的常量索引
        let components_count = self.get_imm_int(insn_op, 1) as u32;
        LUAU_ASSERT!(components_count > 0 && components_count <= 3);
        LUAU_ASSERT!(self.func.inst_op(insn_op).ops.len() as u32 - 2 == components_count);
        let mut aux = components_count << K_IMPORT_COUNT_SHIFT;
        // component 是第 component 个导入分量的编号：既算出它的操作数槽位（ops 下标
        // 2+component），又决定它在 aux 里的位域位移——两处都是编码数据而非容器游标；
        // 且每步要经 `get_vm_const_input_raw` 回调 &mut self（越界时置 error），
        // 无法持有 ops 的切片借用，故保留编号范围遍历。
        for component in 0..components_count {
          let component_id = self.get_vm_const_input_raw(insn_op, (2 + component) as u8);
          if component_id > K_IMPORT_COMPONENT_MASK {
            self.error = true;
          }
          aux |= component_id << import_component_shift(component);
        }
        self.bcb.emit_aux(aux);
      }
      LuauOpcode::LOP_SETTABLE => {
        let reg_input_0 = self.get_reg_input(insn_op, 0);
        let reg_input_1 = self.get_reg_input(insn_op, 1);
        let reg_input_2 = self.get_reg_input(insn_op, 2);
        self.bcb.emit_abc(
          LuauOpcode::LOP_SETTABLE,
          reg_input_0,
          reg_input_1,
          reg_input_2,
        );
      }
      LuauOpcode::LOP_GETUDATAKS | LuauOpcode::LOP_GETTABLEKS => {
        let reg_input_0 = self.get_reg_input(insn_op, 0);
        let imm_int = self.get_imm_int(insn_op, 1);
        let out = self.get_register(insn_op);
        self.bcb.emit_abc(op, out, reg_input_0, imm_int as u8);
        // cpp 386-390：UDATAKS 的 aux 低 16 位是 Aux16 常量、高 16 位是 imm 标志
        self.emit_ks_aux(insn_op, 2, 3);
      }
      LuauOpcode::LOP_SETUDATAKS | LuauOpcode::LOP_SETTABLEKS => {
        let reg_input_0 = self.get_reg_input(insn_op, 0);
        let reg_input_1 = self.get_reg_input(insn_op, 1);
        let imm_int = self.get_imm_int(insn_op, 2);
        self
          .bcb
          .emit_abc(op, reg_input_0, reg_input_1, imm_int as u8);
        // cpp 398-402：同 GETUDATAKS 的 Aux16 编码
        self.emit_ks_aux(insn_op, 3, 4);
      }
      LuauOpcode::LOP_GETTABLEN => {
        let reg_input_0 = self.get_reg_input(insn_op, 0);
        // cpp 419：getImmIntAsUnsignedABC(bias=-1) 超出 u8 时置 error
        let imm_int = self.get_imm_int_as_unsigned_abc(insn_op, 1, -1);
        let out = self.get_register(insn_op);
        self
          .bcb
          .emit_abc(LuauOpcode::LOP_GETTABLEN, out, reg_input_0, imm_int);
      }
      LuauOpcode::LOP_SETTABLEN => {
        let reg_input_0 = self.get_reg_input(insn_op, 0);
        let reg_input_1 = self.get_reg_input(insn_op, 1);
        // cpp 423：getImmIntAsUnsignedABC(bias=-1) 超出 u8 时置 error
        let imm_int = self.get_imm_int_as_unsigned_abc(insn_op, 2, -1);
        self
          .bcb
          .emit_abc(LuauOpcode::LOP_SETTABLEN, reg_input_0, reg_input_1, imm_int);
      }
      LuauOpcode::LOP_NEWCLOSURE => {
        let proto_input = self.get_proto_input(insn_op, 0);
        let out = self.get_register(insn_op);
        self
          .bcb
          .emit_ad(LuauOpcode::LOP_NEWCLOSURE, out, proto_input as i16);
      }
      LuauOpcode::LOP_NAMECALLUDATA | LuauOpcode::LOP_NAMECALL => {
        let reg_input_0 = self.get_reg_input(insn_op, 0);
        let imm_int = self.get_imm_int(insn_op, 1);
        let out = self.get_register(insn_op);
        self.bcb.emit_abc(op, out, reg_input_0, imm_int as u8);
        // cpp 435-439：UDATA 变体走 Aux16
        self.emit_ks_aux(insn_op, 2, 3);
      }
      LuauOpcode::LOP_CALL => {
        let reg_input_2 = self.get_reg_input(insn_op, 2);
        let imm_int_0 = self.get_imm_int(insn_op, 0);
        let imm_int_1 = self.get_imm_int(insn_op, 1);
        self.bcb.emit_abc(
          LuauOpcode::LOP_CALL,
          reg_input_2,
          (imm_int_0 + 1) as u8,
          (imm_int_1 + 1) as u8,
        );
      }
      LuauOpcode::LOP_CALLFB => {
        let reg_input_3 = self.get_reg_input(insn_op, 3);
        let imm_int_0 = self.get_imm_int(insn_op, 0);
        let imm_int_1 = self.get_imm_int(insn_op, 1);
        let imm_int_2 = self.get_imm_int(insn_op, 2);
        self.bcb.emit_abc(
          LuauOpcode::LOP_CALLFB,
          reg_input_3,
          (imm_int_0 + 1) as u8,
          (imm_int_1 + 1) as u8,
        );
        self.bcb.emit_aux(imm_int_2 as u32);
      }
      LuauOpcode::LOP_RETURN => {
        LUAU_ASSERT!(self.func.inst_op(insn_op).ops.len() > 1);
        let imm_int_0 = self.get_imm_int(insn_op, 0);
        // cpp 452：范围寄存器走 getRegInputForRange 的栈顶检查
        let reg_input = self.get_reg_input_for_range(insn_op, 1, imm_int_0);
        self
          .bcb
          .emit_abc(LuauOpcode::LOP_RETURN, reg_input, (imm_int_0 + 1) as u8, 0);
      }
      LuauOpcode::LOP_JUMP => {
        self.record_jump(insn_op, 0);
        self.bcb.emit_ad(LuauOpcode::LOP_JUMP, 0, 0);
      }
      LuauOpcode::LOP_JUMPBACK => {
        self.record_jump(insn_op, 0);
        self.bcb.emit_ad(LuauOpcode::LOP_JUMPBACK, 0, 0);
      }
      LuauOpcode::LOP_JUMPIFNOT | LuauOpcode::LOP_JUMPIF => {
        self.record_jump(insn_op, 1);
        let reg_input_0 = self.get_reg_input(insn_op, 0);
        self.bcb.emit_ad(op, reg_input_0, 0);
      }
      LuauOpcode::LOP_JUMPIFEQ
      | LuauOpcode::LOP_JUMPIFLE
      | LuauOpcode::LOP_JUMPIFLT
      | LuauOpcode::LOP_JUMPIFNOTEQ
      | LuauOpcode::LOP_JUMPIFNOTLE
      | LuauOpcode::LOP_JUMPIFNOTLT => {
        self.record_jump(insn_op, 2);
        let reg_input_0 = self.get_reg_input(insn_op, 0);
        self.bcb.emit_ad(op, reg_input_0, 0);
        let reg_input_1 = self.get_reg_input(insn_op, 1);
        self.bcb.emit_aux(reg_input_1 as u32);
      }
      LuauOpcode::LOP_ADD
      | LuauOpcode::LOP_SUB
      | LuauOpcode::LOP_MUL
      | LuauOpcode::LOP_DIV
      | LuauOpcode::LOP_MOD
      | LuauOpcode::LOP_POW
      | LuauOpcode::LOP_AND
      | LuauOpcode::LOP_OR
      // 与算术族同形：两寄存器输入 + 寄存器输出（GETTABLE/IDIV 逐字节同体）
      | LuauOpcode::LOP_GETTABLE
      | LuauOpcode::LOP_IDIV => {
        let reg_input_0 = self.get_reg_input(insn_op, 0);
        let reg_input_1 = self.get_reg_input(insn_op, 1);
        let out = self.get_register(insn_op);
        self.bcb.emit_abc(op, out, reg_input_0, reg_input_1);
      }
      LuauOpcode::LOP_ADDK
      | LuauOpcode::LOP_SUBK
      | LuauOpcode::LOP_MULK
      | LuauOpcode::LOP_DIVK
      | LuauOpcode::LOP_MODK
      | LuauOpcode::LOP_POWK
      | LuauOpcode::LOP_ANDK
      | LuauOpcode::LOP_ORK
      // 与 K 常量族同形：寄存器输入 + VM 常量输入（IDIVK 逐字节同体）
      | LuauOpcode::LOP_IDIVK => {
        let reg_input_0 = self.get_reg_input(insn_op, 0);
        let vm_const_input_abc = self.get_vm_const_input_abc(insn_op, 1);
        let out = self.get_register(insn_op);
        self.bcb.emit_abc(op, out, reg_input_0, vm_const_input_abc);
      }
      LuauOpcode::LOP_CONCAT => {
        LUAU_ASSERT!(!self.func.inst_op(insn_op).ops.is_empty());
        let reg_input_0 = self.get_reg_input(insn_op, 0);
        let last_idx = self.func.inst_op(insn_op).ops.len() - 1;
        let reg_input_last = self.get_reg_input(insn_op, last_idx as u8);
        let out = self.get_register(insn_op);
        self
          .bcb
          .emit_abc(LuauOpcode::LOP_CONCAT, out, reg_input_0, reg_input_last);
      }
      LuauOpcode::LOP_NOT | LuauOpcode::LOP_MINUS | LuauOpcode::LOP_LENGTH => {
        let reg_input_0 = self.get_reg_input(insn_op, 0);
        let out = self.get_register(insn_op);
        self.bcb.emit_abc(op, out, reg_input_0, 0);
      }
      LuauOpcode::LOP_NEWTABLE => {
        let imm_int_0 = self.get_imm_int(insn_op, 0);
        let imm_int_1 = self.get_imm_int(insn_op, 1);
        let out = self.get_register(insn_op);
        self
          .bcb
          .emit_abc(LuauOpcode::LOP_NEWTABLE, out, imm_int_0 as u8, 0);
        self.bcb.emit_aux(imm_int_1 as u32);
      }
      LuauOpcode::LOP_DUPTABLE => {
        let vm_const_input_d = self.get_vm_const_input_d(insn_op, 0);
        let out = self.get_register(insn_op);
        self
          .bcb
          .emit_ad(LuauOpcode::LOP_DUPTABLE, out, vm_const_input_d as i16);
      }
      LuauOpcode::LOP_SETLIST => {
        LUAU_ASSERT!(self.func.inst_op(insn_op).ops.len() > 2);
        let imm_int_1 = self.get_imm_int(insn_op, 1);
        LUAU_ASSERT!(imm_int_1 < 255);
        // cpp 523-530：count==0（变长尾参）时 B 复用 A 的寄存器，
        // 只有固定 count 时才有独立的 ops[3] 起始寄存器。
        let start_reg = if imm_int_1 != 0 {
          LUAU_ASSERT!(self.func.inst_op(insn_op).ops.len() > 3);
          self.get_reg_input(insn_op, 3)
        } else {
          self.get_reg_input(insn_op, 2)
        };
        let reg_input_2 = self.get_reg_input(insn_op, 2);
        let imm_int_0 = self.get_imm_int(insn_op, 0);
        self.bcb.emit_abc(
          LuauOpcode::LOP_SETLIST,
          reg_input_2,
          start_reg,
          (imm_int_1 + 1) as u8,
        );
        self.bcb.emit_aux(imm_int_0 as u32);
      }
      // 数值/泛型 for 循环 Prep/Loop 六指令同形：跳槽 3、寄存器输入 0、单字 AD
      LuauOpcode::LOP_FORNPREP
      | LuauOpcode::LOP_FORNLOOP
      | LuauOpcode::LOP_FORGPREP
      | LuauOpcode::LOP_FORGPREP_NEXT
      | LuauOpcode::LOP_FORGPREP_INEXT => {
        self.record_jump(insn_op, 3);
        let reg_input_0 = self.get_reg_input(insn_op, 0);
        self.bcb.emit_ad(op, reg_input_0, 0);
      }
      LuauOpcode::LOP_FORGLOOP => {
        self.record_jump(insn_op, 5);
        let reg_input_0 = self.get_reg_input(insn_op, 0);
        self.bcb.emit_ad(LuauOpcode::LOP_FORGLOOP, reg_input_0, 0);
        let imm_bool_3 = self.get_imm_bool(insn_op, 3);
        let imm_int_4 = self.get_imm_int(insn_op, 4);
        let invert = insn::bit_if(imm_bool_3, insn::AUX_INVERT_BIT);
        let aux = invert | (imm_int_4 as u32);
        self.bcb.emit_aux(aux);
      }
      LuauOpcode::LOP_FASTCALL => {
        let imm_int_0 = self.get_imm_int(insn_op, 0);
        let imm_int_1 = self.get_imm_int(insn_op, 1);
        self.bcb.emit_abc(
          LuauOpcode::LOP_FASTCALL,
          imm_int_0 as u8,
          0,
          imm_int_1 as u8,
        );
      }
      LuauOpcode::LOP_FASTCALL1 => {
        let imm_int_0 = self.get_imm_int(insn_op, 0);
        let reg_input_1 = self.get_reg_input(insn_op, 1);
        let imm_int_2 = self.get_imm_int(insn_op, 2);
        self.bcb.emit_abc(
          LuauOpcode::LOP_FASTCALL1,
          imm_int_0 as u8,
          reg_input_1,
          imm_int_2 as u8,
        );
      }
      LuauOpcode::LOP_FASTCALL2 => {
        let imm_int_0 = self.get_imm_int(insn_op, 0);
        let reg_input_1 = self.get_reg_input(insn_op, 1);
        let imm_int_3 = self.get_imm_int(insn_op, 3);
        let reg_input_2 = self.get_reg_input(insn_op, 2);
        self.bcb.emit_abc(
          LuauOpcode::LOP_FASTCALL2,
          imm_int_0 as u8,
          reg_input_1,
          imm_int_3 as u8,
        );
        self.bcb.emit_aux(reg_input_2 as u32);
      }
      LuauOpcode::LOP_FASTCALL2K => {
        let imm_int_0 = self.get_imm_int(insn_op, 0);
        let reg_input_1 = self.get_reg_input(insn_op, 1);
        let imm_int_3 = self.get_imm_int(insn_op, 3);
        let vm_const_input_aux = self.get_vm_const_input_aux(insn_op, 2);
        self.bcb.emit_abc(
          LuauOpcode::LOP_FASTCALL2K,
          imm_int_0 as u8,
          reg_input_1,
          imm_int_3 as u8,
        );
        self.bcb.emit_aux(vm_const_input_aux);
      }
      LuauOpcode::LOP_FASTCALL3 => {
        let imm_int_0 = self.get_imm_int(insn_op, 0);
        let reg_input_1 = self.get_reg_input(insn_op, 1);
        let imm_int_4 = self.get_imm_int(insn_op, 4);
        let reg_input_2 = self.get_reg_input(insn_op, 2);
        let reg_input_3 = self.get_reg_input(insn_op, 3);
        let aux = reg_input_2 as u32 | (reg_input_3 as u32) << insn::AUX_BYTE_SHIFT;
        self.bcb.emit_abc(
          LuauOpcode::LOP_FASTCALL3,
          imm_int_0 as u8,
          reg_input_1,
          imm_int_4 as u8,
        );
        self.bcb.emit_aux(aux);
      }
      LuauOpcode::LOP_GETVARARGS => {
        LUAU_ASSERT!(
          self.func.inst_op(insn_op).ops.len() == 2
            && self.func.inst_op(insn_op).ops[0].kind == BcOpKind::VmReg
        );
        let imm_int_1 = self.get_imm_int(insn_op, 1);
        // cpp 598：走 getRegInputForRange 的栈顶检查
        let reg_input = self.get_reg_input_for_range(insn_op, 0, imm_int_1);
        self.bcb.emit_abc(
          LuauOpcode::LOP_GETVARARGS,
          reg_input,
          (imm_int_1 + 1) as u8,
          0,
        );
      }
      LuauOpcode::LOP_DUPCLOSURE => {
        let vm_const_input_d = self.get_vm_const_input_d(insn_op, 0);
        let out = self.get_register(insn_op);
        self
          .bcb
          .emit_ad(LuauOpcode::LOP_DUPCLOSURE, out, vm_const_input_d as i16);
      }
      LuauOpcode::LOP_PREPVARARGS => {
        let imm_int_0 = self.get_imm_int(insn_op, 0);
        self
          .bcb
          .emit_ad(LuauOpcode::LOP_PREPVARARGS, imm_int_0 as u8, 0);
      }
      LuauOpcode::LOP_LOADKX => {
        let out = self.get_register(insn_op);
        self.bcb.emit_ad(LuauOpcode::LOP_LOADKX, out, 0);
        let vm_const_input_aux = self.get_vm_const_input_aux(insn_op, 0);
        self.bcb.emit_aux(vm_const_input_aux);
      }
      LuauOpcode::LOP_JUMPX => {
        self.record_jump(insn_op, 0);
        self.bcb.emit_e(LuauOpcode::LOP_JUMPX, 0);
      }
      LuauOpcode::LOP_COVERAGE => {
        let imm_int_0 = self.get_imm_int(insn_op, 0);
        self.bcb.emit_e(LuauOpcode::LOP_COVERAGE, imm_int_0);
      }
      LuauOpcode::LOP_CAPTURE => {
        let capture_type = self.get_imm_int(insn_op, 0) as u8;
        if capture_type == LuauCaptureType::LCT_VAL as u8
          || capture_type == LuauCaptureType::LCT_REF as u8
        {
          let reg_input_1 = self.get_reg_input(insn_op, 1);
          let imm_int_2 = self.get_imm_int(insn_op, 2);
          self.bcb.emit_abc(
            LuauOpcode::LOP_CAPTURE,
            capture_type,
            reg_input_1,
            imm_int_2 as u8,
          );
        } else {
          let upval_input = self.get_upval_input(insn_op, 1);
          let imm_int_2 = self.get_imm_int(insn_op, 2);
          self.bcb.emit_abc(
            LuauOpcode::LOP_CAPTURE,
            capture_type,
            upval_input,
            imm_int_2 as u8,
          );
        }
      }
      LuauOpcode::LOP_SUBRK | LuauOpcode::LOP_DIVRK => {
        let vm_const_input_abc = self.get_vm_const_input_abc(insn_op, 0);
        let reg_input_1 = self.get_reg_input(insn_op, 1);
        let out = self.get_register(insn_op);
        self.bcb.emit_abc(op, out, vm_const_input_abc, reg_input_1);
      }
      LuauOpcode::LOP_JUMPXEQKNIL => {
        self.record_jump(insn_op, 2);
        let reg_input_0 = self.get_reg_input(insn_op, 0);
        let imm_bool_1 = self.get_imm_bool(insn_op, 1);
        let invert = insn::bit_if(imm_bool_1, insn::AUX_INVERT_BIT);
        let aux = invert;
        self
          .bcb
          .emit_ad(LuauOpcode::LOP_JUMPXEQKNIL, reg_input_0, 0);
        self.bcb.emit_aux(aux);
      }
      LuauOpcode::LOP_JUMPXEQKB => {
        self.record_jump(insn_op, 2);
        let reg_input_0 = self.get_reg_input(insn_op, 0);
        let imm_bool_1 = self.get_imm_bool(insn_op, 1);
        let imm_bool_3 = self.get_imm_bool(insn_op, 3);
        let invert = insn::bit_if(imm_bool_1, insn::AUX_INVERT_BIT);
        let bool_value = insn::bit_if(imm_bool_3, insn::AUX_BOOL_VALUE_BIT);
        let aux = invert | bool_value;
        self.bcb.emit_ad(LuauOpcode::LOP_JUMPXEQKB, reg_input_0, 0);
        self.bcb.emit_aux(aux);
      }
      LuauOpcode::LOP_JUMPXEQKN | LuauOpcode::LOP_JUMPXEQKS => {
        self.record_jump(insn_op, 2);
        let reg_input_0 = self.get_reg_input(insn_op, 0);
        let imm_bool_1 = self.get_imm_bool(insn_op, 1);
        let vm_const_input_aux = self.get_vm_const_input_aux(insn_op, 3);
        let invert = insn::bit_if(imm_bool_1, insn::AUX_INVERT_BIT);
        let aux = invert | vm_const_input_aux;
        self.bcb.emit_ad(op, reg_input_0, 0);
        self.bcb.emit_aux(aux);
      }
      LuauOpcode::LOP_NEWCLASSMEMBER => {
        LUAU_ASSERT!(DebugLuauUserDefinedClasses.get());
        let reg_input_0 = self.get_reg_input(insn_op, 0);
        let reg_input_1 = self.get_reg_input(insn_op, 1);
        self
          .bcb
          .emit_abc(LuauOpcode::LOP_NEWCLASSMEMBER, reg_input_0, 0, reg_input_1);
        let vm_const_input_aux = self.get_vm_const_input_aux(insn_op, 2);
        self.bcb.emit_aux(vm_const_input_aux);
      }
      LuauOpcode::LOP_CMPPROTO => {
        self.record_jump(insn_op, 2);
        let reg_input_0 = self.get_reg_input(insn_op, 0);
        let imm_int_1 = self.get_imm_int(insn_op, 1);
        self.bcb.emit_ad(LuauOpcode::LOP_CMPPROTO, reg_input_0, 0);
        self.bcb.emit_aux(imm_int_1 as u32);
      }
      LuauOpcode::LOP__COUNT => {
        LUAU_UNREACHABLE!();
      }
    }
  }
}

// ── abs-r139：并自 `methods/bytecode_graph_serializer_emit_ks_aux.rs` ──
impl<'a, 'b, 'f> BytecodeGraphSerializer<'a, 'b, 'f> {
  /// cpp 402-405 / 412-415 / 434-437 三处共用模式：UDATA 变体（GETUDATAKS/
  /// SETUDATAKS/NAMECALLUDATA）aux 低 16 位为 Aux16 常量、上 16 位为 imm 标志；
  /// 普通变体直接走 Aux 常量。`aux_index` 同时是 Aux16 与 Aux 槽位。
  pub(crate) fn emit_ks_aux(&mut self, insn_op: BcOp, aux_index: u8, flags_index: u8) {
    let is_udata = matches!(
      self.func.inst_op(insn_op).op,
      LuauOpcode::LOP_GETUDATAKS | LuauOpcode::LOP_SETUDATAKS | LuauOpcode::LOP_NAMECALLUDATA
    );
    if is_udata {
      let aux16 = self.get_vm_const_input_aux16(insn_op, aux_index);
      let flags = self.get_imm_int(insn_op, flags_index);
      self
        .bcb
        .emit_aux(aux16 | (flags as u32) << insn::AUX_FLAGS_SHIFT);
    } else {
      let vm_const_input_aux = self.get_vm_const_input_aux(insn_op, aux_index);
      self.bcb.emit_aux(vm_const_input_aux);
    }
  }
}

// ── abs-r139：并自 `methods/bytecode_graph_serializer_get_imm.rs` ──
impl<'a, 'b, 'f> BytecodeGraphSerializer<'a, 'b, 'f> {
  /// 各 `get_*_input` 共用的第 `index` 个输入操作数现取样板（单源）：操作数为
  /// Copy，块作用域借用即取即放，调用方持句柄继续走 `&mut self` 不再与 `func`
  /// 的借用冲突（`get_proto_input` 旧形态 `index < ops.len() as u8` 一并归一为
  /// usize 比较，越界判据不变）。
  pub(crate) fn insn_input(&mut self, insn_op: BcOp, index: u8) -> BcOp {
    let insn = self.func.inst_op(insn_op);
    LUAU_ASSERT!((index as usize) < insn.ops.len());
    insn.ops[index as usize]
  }

  /// 按指令句柄现取操作数再解析 Imm：`BcImm` 为 Copy 直接按值返回，
  /// 调用方不再需要脱离 `self.func` 的指令副本（旧实现的 per-instruction clone 根源）。
  pub(crate) fn get_imm(&mut self, insn_op: BcOp, index: u8) -> BcImm {
    let inp: BcOp = self.insn_input(insn_op, index);
    LUAU_ASSERT!(inp.kind == BcOpKind::Imm);
    *self.func.imm_op(inp)
  }

  /// 取第 `index` 个立即数并断言其类别：五个 `getImm*` 访问器的共同前缀
  /// （cpp `BytecodeGraphSerializer.h` 各访问器同型的 kind 断言），收口单源。
  fn get_imm_of_kind(&mut self, insn_op: BcOp, index: u8, expected: BcImmKind) -> BcImm {
    let imm = self.get_imm(insn_op, index);
    LUAU_ASSERT!(imm.kind() == expected);
    imm
  }

  pub(crate) fn get_imm_bool(&mut self, insn_op: BcOp, index: u8) -> bool {
    self
      .get_imm_of_kind(insn_op, index, BcImmKind::Boolean)
      .as_boolean()
  }

  pub(crate) fn get_imm_import(&mut self, insn_op: BcOp, index: u8) -> u32 {
    self
      .get_imm_of_kind(insn_op, index, BcImmKind::Import)
      .as_import()
  }

  pub(crate) fn get_imm_int(&mut self, insn_op: BcOp, index: u8) -> i32 {
    self
      .get_imm_of_kind(insn_op, index, BcImmKind::Int)
      .as_int()
  }

  /// cpp `getImmIntAsSignedD`（BytecodeGraphSerializer.h:184-193）：D 槽
  /// 有符号立即数，超出 `i16` 范围时置 error，仍返回截断值。
  pub(crate) fn get_imm_int_as_signed_d(&mut self, insn_op: BcOp, index: u8) -> i16 {
    let value: i32 = self
      .get_imm_of_kind(insn_op, index, BcImmKind::Int)
      .as_int();

    // cpp: if (int32_t(int16_t(imm.valueInt)) != imm.valueInt) error = true;
    if i32::from(value as i16) != value {
      self.error = true;
    }

    value as i16
  }

  /// cpp `getImmIntAsUnsignedABC`（BytecodeGraphSerializer.h:171-182）：ABC 槽
  /// 无符号立即数（bias 用于 `imm - 1` 偏移），超出 `u8` 范围时置 error，
  /// 仍返回截断值。
  pub(crate) fn get_imm_int_as_unsigned_abc(&mut self, insn_op: BcOp, index: u8, bias: i32) -> u8 {
    // cpp: int64_t result = int64_t(imm.valueInt) + bias;
    let result: i64 = i64::from(
      self
        .get_imm_of_kind(insn_op, index, BcImmKind::Int)
        .as_int(),
    ) + i64::from(bias);

    if u8::try_from(result).map(i64::from) != Ok(result) {
      self.error = true;
    }

    result as u8
  }
}

// ── abs-r139：并自 `methods/bytecode_graph_serializer_get_proto_input.rs` ──
impl<'a, 'b, 'f> BytecodeGraphSerializer<'a, 'b, 'f> {
  /// cpp `getProtoInput`（BytecodeGraphSerializer.h:277-287）：proto 索引超出
  /// `kMaxClosureCount` 时置 error。
  pub(crate) fn get_proto_input(&mut self, insn_op: BcOp, index: u8) -> u16 {
    let inp = self.insn_input(insn_op, index);
    LUAU_ASSERT!(inp.kind == BcOpKind::VmProto);

    if inp.index >= K_MAX_CLOSURE_COUNT {
      self.error = true;
    }

    inp.index as u16
  }
}

// ── abs-r139：并自 `methods/bytecode_graph_serializer_get_reg_input.rs` ──
impl<'a, 'b, 'f> BytecodeGraphSerializer<'a, 'b, 'f> {
  pub(crate) fn get_reg_input(&mut self, insn_op: BcOp, index: u8) -> u8 {
    let op: BcOp = self.insn_input(insn_op, index);
    self.get_register(op)
  }
}

// ── abs-r139：并自 `methods/bytecode_graph_serializer_get_register.rs` ──
impl<'a, 'b, 'f> BytecodeGraphSerializer<'a, 'b, 'f> {
  /// cpp `getRegisterRaw`（BytecodeGraphSerializer.h:62-101）：把 Phi/Inst/Proj
  /// 解析到具体寄存器号，不做溢出检查。
  pub(crate) fn get_register_raw(&mut self, op: BcOp) -> u32 {
    // 循环携带 phi（内外累加 phi 互为操作数成环）经 phi.ops 递归不终止，
    // 故先查 makePhi 时记录的 regs；只有无记录的返回值合并 phi（inliner 插入、
    // 无环）才沿首操作数解析。
    match op.kind {
      BcOpKind::Phi => {
        // 上游只在无记录分支索引 ops[0]，但断言在取 regs 之前，故先读出首操作数
        let first_op = self.func.phi_op(op).ops.first().copied();
        LUAU_ASSERT!(first_op.is_some());

        if let Some(&reg) = self.func.regs.get(&op) {
          u32::from(reg)
        } else {
          let Some(first_op) = first_op else {
            // cpp 在此处解引用空 ops（release 下 UB）；空 phi 只可能来自损坏的图
            LUAU_ASSERT!(false, "phi without inputs");
            return 0;
          };
          LUAU_ASSERT!(first_op != op);
          u32::from(self.get_register(first_op))
        }
      }
      BcOpKind::Inst => {
        let reg = self.func.regs.get(&op).copied();
        LUAU_ASSERT!(reg.is_some());
        u32::from(reg.unwrap_or(0))
      }
      BcOpKind::Proj => {
        // Avoid holding `&mut` to `self.func` while recursively calling `self.get_register`.
        let proj = {
          let proj: &mut BcProj = self.func.proj_op(op);
          *proj
        };
        u32::from(self.get_register(proj.op)) + proj.index
      }
      BcOpKind::VmReg => op.index,
      _ => {
        LUAU_UNREACHABLE!()
      }
    }
  }

  /// cpp `getRegister`（BytecodeGraphSerializer.h:102-121）：溢出置 error。
  pub(crate) fn get_register(&mut self, op: BcOp) -> Reg {
    let raw = self.get_register_raw(op);
    self.check_reg_span(raw, 1);
    raw as Reg
  }

  /// 两个取值臂共用的寄存器溢出检查（cpp `getRegister` 的 `reg >= maxstacksize`
  /// 与 `getRegInputForRange` 的 `reg + range > maxstacksize` 在整数语义下同为
  /// 「起点 + 占用数不越过栈顶」，单点占用即 `span = 1`；饱和加把 debug 下的
  /// 潜在回绕 panic 收敛为置 error）。
  fn check_reg_span(&mut self, reg: u32, span: u32) {
    if reg >= K_INVALID_REG {
      LUAU_ASSERT!(false, "register reference overflow");
      self.error = true;
    }

    if reg.saturating_add(span) > self.func.maxstacksize as u32 {
      LUAU_ASSERT!(false, "register overflows the function stack");
      self.error = true;
    }
  }

  /// cpp `getRegInputForRange`（BytecodeGraphSerializer.h:127-153）：取范围
  /// 起始寄存器，按 count 检查 `reg + range` 不越过函数栈顶。
  pub(crate) fn get_reg_input_for_range(&mut self, insn_op: BcOp, index: u8, count: i32) -> Reg {
    let input_op: BcOp = self.insn_input(insn_op, index);

    let reg = self.get_register_raw(input_op);

    if !(-1..K_INVALID_REG as i32).contains(&count) {
      LUAU_ASSERT!(false, "register count overflow");
      self.error = true;
    }

    let range = if count < 0 { 0 } else { count };
    self.check_reg_span(reg, range as u32);

    reg as Reg
  }
}

// ── abs-r139：并自 `methods/bytecode_graph_serializer_get_upval_input.rs` ──
impl<'a, 'b, 'f> BytecodeGraphSerializer<'a, 'b, 'f> {
  /// cpp `getUpvalInput`（BytecodeGraphSerializer.h:258-275）：upvalue 索引
  /// 超出函数 nups 或 `kMaxUpvalueCount` 时置 error。
  pub(crate) fn get_upval_input(&mut self, insn_op: BcOp, index: u8) -> u8 {
    let inp: BcOp = self.insn_input(insn_op, index);
    LUAU_ASSERT!(inp.kind == BcOpKind::VmUpvalue);

    if inp.index >= self.func.nups as u32 {
      LUAU_ASSERT!(
        false,
        "upvalue reference overflows the function upvalue count"
      );
      self.error = true;
    }

    if inp.index >= K_MAX_UPVALUE_COUNT {
      self.error = true;
    }

    LUAU_ASSERT!(inp.index < self.func.nups as u32);
    inp.index as u8
  }
}

// ── abs-r139：并自 `methods/bytecode_graph_serializer_get_vm_const_input_abc.rs` ──
define_input_getter! {
  get_vm_const_input_abc -> u8 {
    cid if cid > u32::from(u8::MAX);
    cid as u8
  }
}

// ── abs-r139：并自 `methods/bytecode_graph_serializer_get_vm_const_input_aux.rs` ──
define_input_getter! {
  /// cpp `getVmConstInputAux`（BytecodeGraphSerializer.h:248-256）：aux 槽
  /// 常量索引，超出 `kMaxConstantCount` 时置 error。
  get_vm_const_input_aux -> u32 {
    cid if cid >= K_MAX_CONSTANT_COUNT;
    cid
  }
}

// ── abs-r139：并自 `methods/bytecode_graph_serializer_get_vm_const_input_aux16.rs` ──
define_input_getter! {
  /// cpp `getVmConstInputAux16`（BytecodeGraphSerializer.h:238-245）：
  /// 16 位常量索引，超出时置 error。
  get_vm_const_input_aux16 -> u32 {
    cid if cid > u32::from(u16::MAX);
    cid
  }
}

// ── abs-r139：并自 `methods/bytecode_graph_serializer_get_vm_const_input_d.rs` ──
define_input_getter! {
  /// cpp `getVmConstInputD`（BytecodeGraphSerializer.h:228-236）：D 槽 16 位
  /// 有符号常量索引，超出 `0x7fff` 时置 error。
  get_vm_const_input_d -> u16 {
    cid if cid > i32::from(i16::MAX) as u32;
    cid as u16
  }
}

// ── abs-r139：并自 `methods/bytecode_graph_serializer_get_vm_const_input_raw.rs` ──
impl<'a, 'b, 'f> BytecodeGraphSerializer<'a, 'b, 'f> {
  pub(crate) fn get_vm_const_input_raw(&mut self, insn_op: BcOp, index: u8) -> u32 {
    // 操作数为 Copy：现取即释放 func 借用，供后续 consts/constants 读取
    let inp: BcOp = self.insn_input(insn_op, index);
    LUAU_ASSERT!(inp.kind == BcOpKind::VmConst);
    LUAU_ASSERT!((inp.index as usize) < self.func.constants.len());
    if let Some(consts) = &self.consts {
      LUAU_ASSERT!((inp.index as usize) < consts.len());
      consts[inp.index as usize]
    } else {
      inp.index
    }
  }
}

// ── abs-r139：并自 `methods/bytecode_graph_serializer_patch_jump.rs` ──
impl<'a, 'b, 'f> BytecodeGraphSerializer<'a, 'b, 'f> {
  pub fn patch_jump(&mut self, jump: &JumpInfo) {
    let target = self.func.block_op(jump.target_block);
    LUAU_ASSERT!(target.startpc != BcBlock::K_BLOCK_NO_START_PC);

    // cpp（BytecodeGraphSerializer.h:304/310）：修补失败置 error 由
    // emitBytecode 返回空结果，而不是中断/忽略。
    let patched = if is_jump_d(jump.op) {
      self
        .bcb
        .patch_jump_d(jump.instruction_pc as usize, target.startpc as usize)
    } else if is_skip_c(jump.op) {
      self
        .bcb
        .patch_skip_c(jump.instruction_pc as usize, target.startpc as usize)
    } else {
      true
    };
    if !patched {
      self.error = true;
    }
  }
}

// ── abs-r139：并自 `methods/bytecode_graph_serializer_record_jump.rs` ──
impl<'a, 'b, 'f> BytecodeGraphSerializer<'a, 'b, 'f> {
  pub(crate) fn record_jump(&mut self, insn_op: BcOp, index: u8) {
    // op/操作数均为 Copy：op 现读后即释放借用，操作数走共用取输入样板
    let op = self.func.inst_op(insn_op).op;
    let inp: BcOp = self.insn_input(insn_op, index);
    LUAU_ASSERT!(inp.kind == BcOpKind::Block);
    self.jumps.push(JumpInfo {
      op,
      instruction_pc: self.bcb.get_instruction_count() as u32,
      target_block: inp,
    });
  }
}

// ── abs-r139：并自 `methods/bytecode_graph_serializer_reschedule.rs` ──
impl<'a, 'b, 'f> BytecodeGraphSerializer<'a, 'b, 'f> {
  pub(crate) fn reschedule(&mut self) -> Vec<BcOp> {
    // 装饰-排序-剥离：一次遍历取 (sortkey, chainkey) 快照，避免比较器内每次
    // 再经 `block_op` 查表（O(n log n) 次冗余 arena 访问）；sort_by_key 稳定，
    // 同键块保持原序，与旧比较器语义一致。
    let mut keyed: Vec<(u32, u32, BcOp)> = self
      .func
      .blocks
      .iter()
      .enumerate()
      .filter(|(_, block)| (block.flags & BcBlockFlag::Dead) == 0)
      .map(|(i, block)| {
        (
          block.sortkey,
          block.chainkey,
          BcOp::with(BcOpKind::Block, i as u32),
        )
      })
      .collect();

    keyed.sort_by_key(|&(sortkey, chainkey, _)| (sortkey, chainkey));

    let mut sorted_blocks: Vec<BcOp> = keyed.into_iter().map(|(_, _, block)| block).collect();

    LUAU_ASSERT!(sorted_blocks.last() == Some(&self.func.exit_block));
    sorted_blocks.pop();

    sorted_blocks
  }
}
