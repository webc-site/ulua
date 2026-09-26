use alloc::vec::Vec;
use core::{
  mem::size_of,
  ptr::{from_ref, null_mut},
};

use ulua_common::records::{
  dense_hash_map::DenseHashMap, dense_hash_table::DenseDefault, small_vector::SmallVector,
};
use ulua_vm::records::proto::Proto;

use crate::{
  enums::{ir_op_kind::IrOpKind, ir_value_kind::IrValueKind},
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{
    bytecode_block::BytecodeBlock,
    bytecode_mapping::BytecodeMapping,
    bytecode_type_info::BytecodeTypeInfo,
    bytecode_types::BytecodeTypes,
    cfg_info::CfgInfo,
    ir_block::IrBlock,
    ir_builder::{ConstantKey, ConstantMap},
    ir_const::IrConst,
    ir_inst::IrInst,
    ir_op::IrOp,
    lowering_stats::LoweringStats,
    store_location_hint::StoreLocationHint,
    value_restore_location::ValueRestoreLocation,
    vm_exit_sync_info::VmExitSyncInfo,
  },
};

extern crate alloc;

#[derive(Debug, Clone)]
#[repr(C)]
pub struct IrFunction {
  pub blocks: Vec<IrBlock>,
  pub instructions: Vec<IrInst>,
  pub constants: Vec<IrConst>,

  pub bc_blocks: Vec<BytecodeBlock>,
  pub bc_types: Vec<BytecodeTypes>,

  pub bc_mapping: Vec<BytecodeMapping>,
  pub entry_block: u32,
  pub entry_location: u32,
  pub end_location: u32,

  pub extra_native_data: Vec<u32>,

  pub value_restore_ops: Vec<ValueRestoreLocation>,
  pub valid_restore_op_blocks: Vec<u32>,
  pub store_location_hints: DenseHashMap<u32, StoreLocationHint>,

  pub vm_exit_info: DenseHashMap<u32, VmExitSyncInfo>,
  pub block_to_vm_exit_map: DenseHashMap<u32, u32>,

  pub bc_original_type_info: BytecodeTypeInfo,
  pub bc_type_info: BytecodeTypeInfo,

  pub proto: *mut Proto,
  pub variadic: bool,

  pub cfg: CfgInfo,

  pub stats: *mut LoweringStats,

  pub record_counters: bool,

  pub jit_rng_state: u64,

  pub block_exit_tags: Vec<Vec<u8>>,

  pub fallback_entry_tags: Vec<Vec<u8>>,
}

impl DenseDefault for StoreLocationHint {
  fn dense_default() -> Self {
    Self {
      op: IrOp { kind_and_index: 0 },
      inst_idx: !0u32,
      kind: IrValueKind::None,
    }
  }
}

impl DenseDefault for VmExitSyncInfo {
  fn dense_default() -> Self {
    Self {
      reg_stores: Vec::new(),
      block: IrOp { kind_and_index: 0 },
      vm_exit: IrOp { kind_and_index: 0 },
      arg_ops: SmallVector::new(),
    }
  }
}

impl Default for IrFunction {
  fn default() -> Self {
    Self {
      blocks: Vec::new(),
      instructions: Vec::new(),
      constants: Vec::new(),
      bc_blocks: Vec::new(),
      bc_types: Vec::new(),
      bc_mapping: Vec::new(),
      entry_block: 0,
      entry_location: 0,
      end_location: 0,
      extra_native_data: Vec::new(),
      value_restore_ops: Vec::new(),
      valid_restore_op_blocks: Vec::new(),
      // kInvalidInstIdx 是 ~0u32
      store_location_hints: DenseHashMap::new(!0u32),
      vm_exit_info: DenseHashMap::new(!0u32),
      block_to_vm_exit_map: DenseHashMap::new(!0u32),
      bc_original_type_info: BytecodeTypeInfo::default(),
      bc_type_info: BytecodeTypeInfo::default(),
      proto: null_mut(),
      variadic: false,
      cfg: CfgInfo::default(),
      stats: null_mut(),
      record_counters: false,
      jit_rng_state: 0,
      block_exit_tags: Vec::new(),
      fallback_entry_tags: Vec::new(),
    }
  }
}

impl IrFunction {
  /// 常量取浮点：纯查询，只需 `&self`
  pub fn as_double_op(&self, op: IrOp) -> Option<f64> {
    if op.kind() != IrOpKind::Constant {
      return None;
    }

    match self.const_op(op) {
      IrConst::Double(value) => Some(value),
      _ => None,
    }
  }

  pub fn const_op(&self, op: IrOp) -> IrConst {
    self.constants[op.index() as usize]
  }

  /// 对应 cpp `IrFunction::asInstOp` 的可变版：非 Inst kind 或越界返回 `None`。
  /// 写入路径经此消除裸指针判空样板。
  pub fn as_inst_op_mut(&mut self, op: IrOp) -> Option<&mut IrInst> {
    if op.kind() == IrOpKind::Inst {
      self.instructions.get_mut(op.index() as usize)
    } else {
      None
    }
  }

  /// 只读版：非 Inst kind 或越界返回 `None`。用于纯读路径，避免强制调用方持有
  /// `&mut IrFunction`（try_get_operand_tag 等）。
  pub fn as_inst_op_ref(&self, op: IrOp) -> Option<&IrInst> {
    if op.kind() == IrOpKind::Inst {
      self.instructions.get(op.index() as usize)
    } else {
      None
    }
  }

  /// 常量取 i64：纯查询，只需 `&self`
  pub fn as_int_64_op(&self, op: IrOp) -> Option<i64> {
    if op.kind() != IrOpKind::Constant {
      return None;
    }

    match self.const_op(op) {
      IrConst::Int64(value) => Some(value),
      _ => None,
    }
  }

  /// 常量取整：纯查询，只需 `&self`
  pub fn as_int_op(&self, op: IrOp) -> Option<i32> {
    if op.kind() != IrOpKind::Constant {
      return None;
    }

    match self.const_op(op) {
      IrConst::Int(value) => Some(value),
      _ => None,
    }
  }

  pub fn block_op(&mut self, op: IrOp) -> &mut IrBlock {
    assert!(op.kind() == IrOpKind::Block);
    &mut self.blocks[op.index() as usize]
  }

  /// 常量 intern 收口在 `IrFunction`（constants 宿主）上，去重表 `ConstantMap` 以独立参数传入。
  /// `IrBuilder::const_*` 仅为转发；const-prop 等只借用 `function`/`constant_map` 两个互不相交
  /// 字段的调用方因此无需 `&mut IrBuilder` 与 `&mut build.function` 的裸指针自别名。
  pub fn const_any(
    &mut self,
    constant_map: &mut ConstantMap,
    constant: IrConst,
    as_common_key: u64,
  ) -> IrOp {
    let key = ConstantKey {
      kind: constant.kind(),
      value: as_common_key,
    };

    if let Some(&index) = constant_map.find(&key) {
      return IrOp::ir_op_ir_op_kind_u32(IrOpKind::Constant, index);
    }

    let index = self.constants.len() as u32;
    self.constants.push(constant);
    *constant_map.get_or_insert(key) = index;

    IrOp::ir_op_ir_op_kind_u32(IrOpKind::Constant, index)
  }

  pub fn const_int(&mut self, constant_map: &mut ConstantMap, value: i32) -> IrOp {
    self.const_any(constant_map, IrConst::Int(value), value as u64)
  }

  pub fn const_int_64(&mut self, constant_map: &mut ConstantMap, value: i64) -> IrOp {
    self.const_any(constant_map, IrConst::Int64(value), value as u64)
  }

  pub fn const_uint(&mut self, constant_map: &mut ConstantMap, value: u32) -> IrOp {
    self.const_any(constant_map, IrConst::Uint(value), value as u64)
  }

  pub fn const_tag(&mut self, constant_map: &mut ConstantMap, value: u8) -> IrOp {
    self.const_any(constant_map, IrConst::Tag(value), value as u64)
  }

  pub fn const_double(&mut self, constant_map: &mut ConstantMap, value: f64) -> IrOp {
    // 去重键取位模式，规避 NaN 内容不等导致的重复常量
    self.const_any(constant_map, IrConst::Double(value), value.to_bits())
  }

  pub fn double_op(&self, op: IrOp) -> f64 {
    match self.const_op(op) {
      IrConst::Double(value) => value,
      // 与原 assert 语义一致：release 下仍校验，类型不符即编译器内部不变量被破坏
      _ => panic!("double_op: 非 Double 常量"),
    }
  }

  pub fn find_restore_location_u32_bool(
    &self,
    inst_idx: u32,
    limit_to_current_block: bool,
  ) -> ValueRestoreLocation {
    if inst_idx >= self.value_restore_ops.len() as u32 {
      // IrOp{0}、Unknown、NOP 均为判别值 0，与 zeroed 位等价
      return ValueRestoreLocation::default();
    }

    if limit_to_current_block {
      for &block_idx in &self.valid_restore_op_blocks {
        let block: &IrBlock = &self.blocks[block_idx as usize];

        if inst_idx >= block.start && inst_idx <= block.finish {
          return self.value_restore_ops[inst_idx as usize];
        }
      }

      return ValueRestoreLocation::default();
    }

    self.value_restore_ops[inst_idx as usize]
  }

  pub fn find_restore_location_ir_inst_bool(
    &self,
    inst: &IrInst,
    limit_to_current_block: bool,
  ) -> ValueRestoreLocation {
    self.find_restore_location_u32_bool(self.get_inst_index(inst), limit_to_current_block)
  }

  pub fn find_store_location_hint(&self, inst_idx: u32) -> Option<&StoreLocationHint> {
    self.store_location_hints.find(&inst_idx)
  }

  pub fn get_block_index(&self, block: &IrBlock) -> u32 {
    // 只能传入本 vector 中的 block（cpp getBlockIndex 以指针差反推索引，跨函数传入即 UB；
    // 此处显式校验区间并 panic，属编译器内部不变量）。
    let block_ptr = from_ref(block).cast::<u8>() as usize;
    let base_ptr = self.blocks.as_ptr().cast::<u8>() as usize;
    let end_ptr = base_ptr + self.blocks.len() * size_of::<IrBlock>();

    if !(block_ptr >= base_ptr && block_ptr <= end_ptr) {
      panic!("IrFunction::get_block_index: 传入块不属于本函数");
    }

    let offset = block_ptr - base_ptr;
    (offset / size_of::<IrBlock>()) as u32
  }

  pub fn get_bytecode_types_at(&self, pcpos: i32) -> BytecodeTypes {
    // 这里避免使用 CODEGEN_ASSERT!，因为它对
    // ulua_common::assertCallHandler / 架构 intrinsic 的依赖
    // 在本 crate 配置下编译失败。
    if !(pcpos >= 0) {
      return BytecodeTypes {
        result: 0,
        a: 0,
        b: 0,
        c: 0,
      };
    }

    let pcpos_usize = pcpos as usize;

    if pcpos_usize < self.bc_types.len() {
      self.bc_types[pcpos_usize]
    } else {
      BytecodeTypes {
        result: 0,
        a: 0,
        b: 0,
        c: 0,
      }
    }
  }

  pub fn has_restore_location_ir_inst_bool(
    &self,
    inst: &IrInst,
    limit_to_current_block: bool,
  ) -> bool {
    let restore_location = self.find_restore_location_ir_inst_bool(inst, limit_to_current_block);
    restore_location.op.kind() != IrOpKind::None
  }

  pub fn import_op(&self, op: IrOp) -> u32 {
    let value = self.const_op(op);
    debug_assert!(matches!(value, IrConst::Import(_)));
    match value {
      IrConst::Import(v) => v,
      // 原 release 下为 union 误读 UB；此处确定回退零值，debug 下由断言拦截
      _ => 0,
    }
  }

  pub fn inst_op(&mut self, op: IrOp) -> &mut IrInst {
    assert!(op.kind() == IrOpKind::Inst);
    &mut self.instructions[op.index() as usize]
  }

  /// 只读取常量池中的 Int64 常量，与 `int_op`/`uint_op`/`double_op` 同为共享借用
  /// （cpp `IrFunction::int64Op` 不修改函数体）。
  pub fn int64_op(&self, op: IrOp) -> i64 {
    let value = self.const_op(op);
    debug_assert!(matches!(value, IrConst::Int64(_)));
    match value {
      IrConst::Int64(v) => v,
      // 原 release 下为 union 误读 UB；此处确定回退零值，debug 下由断言拦截
      _ => 0,
    }
  }

  pub fn int_op(&self, op: IrOp) -> i32 {
    match self.const_op(op) {
      IrConst::Int(value) => value,
      // 与原 assert 语义一致：release 下仍校验，类型不符即编译器内部不变量被破坏
      _ => panic!("int_op: 非 Int 常量"),
    }
  }

  pub fn materialize_restore_location(&mut self, inst_idx: u32) {
    assert!((inst_idx as usize) < self.value_restore_ops.len());
    assert!(self.value_restore_ops[inst_idx as usize].lazy);

    self.value_restore_ops[inst_idx as usize].lazy = false;
  }

  /// `proto` 字段的只读视图：`None` 表示该 IR 函数没有原型（合成 IR/无调试信息路径）。
  ///
  /// 返回寿命刻意不受 `&self` 约束：cpp 侧 `Proto*` 的存活契约是「整段编译会话内由 VM
  /// 持有」（见 `proto_views` 模块文档），`IrFunction` 只是消费者，故视图可以长于本结构的
  /// 借用——这让调用方能同时持有 `&mut IrFunction` 与原型视图，而无需各写 unsafe 解引用。
  #[inline]
  pub(crate) fn proto_view<'p>(&self) -> Option<&'p Proto> {
    // Safety: 空指针走 `Option::as_ref` 的 `None` 分支、不解引用；非空时依上述接线点契约
    // （`build_function_ir` 自活闭包的 `inner.l.p` 写入，VM 在编译期间持有该 Proto）。
    // 视图只读，与 lowering 侧对 `execdata` 的写入不相交。
    unsafe { self.proto.as_ref() }
  }

  pub fn record_restore_location(&mut self, inst_idx: u32, location: ValueRestoreLocation) {
    CODEGEN_ASSERT!(matches!(
      location.op.kind(),
      IrOpKind::None | IrOpKind::VmReg | IrOpKind::VmConst
    ));

    if inst_idx >= self.value_restore_ops.len() as u32 {
      self
        .value_restore_ops
        .resize(inst_idx as usize + 1, ValueRestoreLocation::default());
    }

    self.value_restore_ops[inst_idx as usize] = location;
  }

  pub fn record_store_location_hint(&mut self, inst_idx: u32, hint: StoreLocationHint) {
    *self.store_location_hints.get_or_insert(inst_idx) = hint;
  }

  pub fn tag_op(&self, op: IrOp) -> u8 {
    match self.const_op(op) {
      IrConst::Tag(value) => value,
      // 与原 CODEGEN_ASSERT 语义一致：release 下仍校验
      _ => panic!("tag_op: 非 Tag 常量"),
    }
  }

  pub fn uint_op(&self, op: IrOp) -> u32 {
    let value = self.const_op(op);
    debug_assert!(matches!(value, IrConst::Uint(_)));
    match value {
      IrConst::Uint(v) => v,
      // 原 release 下为 union 误读 UB；此处确定回退零值，debug 下由断言拦截
      _ => 0,
    }
  }
}
