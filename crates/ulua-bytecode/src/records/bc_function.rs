use core::ptr;
use std::{string::String, vec::Vec};

use ulua_common::{
  collections::HashMap, enums::luau_bytecode_type::LuauBytecodeType,
  macros::luau_assert::LUAU_ASSERT,
};

use crate::{
  enums::{bc_imm_kind::BcImmKind, bc_op_kind::BcOpKind},
  records::{
    bc_block::BcBlock, bc_imm::BcImm, bc_inst::BcInst, bc_op::BcOp, bc_op_hash::BcOpHash,
    bc_phi::BcPhi, bc_proj::BcProj, bc_ref::BcRef, bc_vm_const::BcVmConst, class_shape::ClassShape,
    debug_local_bytecode_graph::DebugLocal, table_shape::TableShape, typed_local::TypedLocal,
  },
  type_aliases::reg::Reg,
};

/// `VmConst` 与 cpp 一样**借用**外部字符串表：`'a` 是 `from_function_bytecode` 传入的
/// `strings`（或图优化阶段自行构造的字节切片）的生命周期，图本身不再拥有常量字符串。
pub type VmConst<'a> = BcVmConst<'a>;

/// SSA 图里的单个函数。生命周期参数 `'a` 对应 `constants` 里字符串常量的借用来源，
/// 语义等价于 cpp `BcFunction`（`std::string_view` 指向 `BytecodeBuilder`/调用方持有的字符串表）。
#[derive(Debug, Clone, Default)]
pub struct BcFunction<'a> {
  pub maxstacksize: u8,
  pub numparams: u8,
  pub nups: u8,
  pub is_vararg: bool,
  pub flags: u8,

  pub blocks: Vec<BcBlock>,
  pub instructions: Vec<BcInst>,
  pub constants: Vec<VmConst<'a>>,
  pub immediates: Vec<BcImm>,
  pub phis: Vec<BcPhi>,
  pub projections: Vec<BcProj>,
  pub table_shapes: Vec<TableShape>,
  /// cpp: `BcFunction::classShapes`（`cpp/Bytecode/include/Luau/BytecodeGraph.h:415`）
  pub class_shapes: Vec<ClassShape>,

  pub entry_block: BcOp,
  pub exit_block: BcOp,

  /// cpp `std::string typeInfo`：类型编码字节流（LBC_TYPE_* 原始字节）。
  pub type_info: Vec<u8>,
  pub upvalue_types: Vec<LuauBytecodeType>,
  pub(crate) local_types: Vec<TypedLocal>,
  pub protos: Vec<u32>,

  pub debugname: String,
  pub linedefined: u32,
  pub upvalue_names: Vec<String>,
  pub(crate) locals: Vec<DebugLocal>,

  /// cpp `BcFunction::regs` 即 `std::unordered_map<BcOp, Reg, BcOpHash>`：
  /// 保持 std HashMap 而非 DenseHashMap——键域内不存在可证明不可达的哨兵值
  /// （kind 判别最小的合法 op 也可能全零），硬选哨兵会重演 B3 的相撞风险。
  pub regs: HashMap<BcOp, Reg, BcOpHash>,
}

// ─── 构造器与访问器（abs-r139：原 `methods/bc_function_*.rs` 12 枚碎片并回本文件）───
// `BcFunction` 的 12 个同形 `BcOp` 访问器坍缩：只读 `BcRef` 视图 6 个 + 可变下标访问 6 个，
// 形状完全一致（断言 `op.kind` → 按字段下标构造），以两个宏家族集中生成，
// 调用方仍以固有方法 `f.block(op)` / `f.block_op(op)` 等原路径使用。
//
// `vm_const` / `const_op` 沿袭原注：（b）镜像定形——cpp 同形访问器全仓零消费，
// 为免降级触发 dead_code 升级而保持 `pub`；非降级对象，勿删（批次账 b28-bc-rt-tail）。

/// 只读视图访问器：断言 kind 后构造 `BcRef`（cpp `BcFunction::block/inst/phi/imm/vmConst`）。
macro_rules! ref_getter {
  ($vis:vis fn $name:ident($field:ident, $ty:ty, $kind:expr)) => {
    impl<'f> BcFunction<'f> {
      $vis fn $name<'a>(&'a self, op: BcOp) -> BcRef<'a, $ty> {
        LUAU_ASSERT!(op.kind == $kind);
        BcRef {
          vec: &self.$field,
          op,
        }
      }
    }
  };
}

/// 可变访问器：断言 kind 后按字段下标取 `&mut T`
/// （cpp `BcFunction::blockOp/instOp/phiOp/projOp/immOp/constOp`）。
macro_rules! mut_accessor {
  ($vis:vis fn $name:ident($field:ident, $ty:ty, $kind:expr)) => {
    impl<'f> BcFunction<'f> {
      $vis fn $name(&mut self, op: BcOp) -> &mut $ty {
        LUAU_ASSERT!(op.kind == $kind);
        &mut self.$field[op.index as usize]
      }
    }
  };
}

ref_getter!(pub fn block(blocks, BcBlock, BcOpKind::Block));
ref_getter!(pub fn inst(instructions, BcInst, BcOpKind::Inst));
ref_getter!(pub fn phi(phis, BcPhi, BcOpKind::Phi));
ref_getter!(pub(crate) fn imm(immediates, BcImm, BcOpKind::Imm));
ref_getter!(pub(crate) fn proj(projections, BcProj, BcOpKind::Proj));
ref_getter!(pub fn vm_const(constants, VmConst<'f>, BcOpKind::VmConst));

mut_accessor!(pub fn block_op(blocks, BcBlock, BcOpKind::Block));
mut_accessor!(pub fn imm_op(immediates, BcImm, BcOpKind::Imm));
mut_accessor!(pub fn inst_op(instructions, BcInst, BcOpKind::Inst));
mut_accessor!(pub fn phi_op(phis, BcPhi, BcOpKind::Phi));
mut_accessor!(pub fn proj_op(projections, BcProj, BcOpKind::Proj));
mut_accessor!(pub fn const_op(constants, BcVmConst<'f>, BcOpKind::VmConst));

/// 追加构造器同形坍缩（cpp `BcFunction::addBlock/addInst/addPhi/addConst/addImm(const
/// BcImm&)/addProj`）：`BcOp::pushed(&mut self.$field, $kind, $init)` 单源。
macro_rules! push_op {
  ($vis:vis fn $name:ident($($arg:ident : $ty:ty),*) { $field:ident, $kind:path, $init:expr }) => {
    impl<'f> BcFunction<'f> {
      $vis fn $name(&mut self $(, $arg: $ty)*) -> BcOp {
        BcOp::pushed(&mut self.$field, $kind, $init)
      }
    }
  };
}

push_op!(pub fn add_block() { blocks, BcOpKind::Block, BcBlock::default() });
// NOP 空槽（`BcInst::default()`）；操作码由 `BcInstHelper::create` 等调用方回填。
push_op!(pub fn add_inst() { instructions, BcOpKind::Inst, BcInst::default() });
push_op!(pub fn add_phi() { phis, BcOpKind::Phi, BcPhi { ops: Default::default() } });
// cpp `BcFunction::addConst`：追加 VM 常量并返回其引用。
push_op!(pub fn add_const(value: VmConst<'f>) { constants, BcOpKind::VmConst, value });
// cpp `BcFunction::addImm(const BcImm&)`：按现值追加立即数。
push_op!(pub(crate) fn add_imm_value(imm: BcImm) { immediates, BcOpKind::Imm, imm });
push_op!(pub(crate) fn add_proj(op: BcOp, index: u32) { projections, BcOpKind::Proj, BcProj { op, index } });

impl<'f> BcFunction<'f> {
  pub fn add_imm(&mut self, kind: BcImmKind) -> BcOp {
    // 占位零值按 kind 选活跃变体显式构造，取代 cpp 的 `memset(0)`：
    // 位型合法性不再依赖对各字段全零有效的隐式论证。追加与索引包装
    // 委托 `add_imm_value` 单源。
    self.add_imm_value(match kind {
      BcImmKind::Boolean => BcImm::Boolean(false),
      BcImmKind::Int => BcImm::Int(0),
      BcImmKind::Import => BcImm::Import(0),
    })
  }

  /// 把 locals / local_types 的 PC 从指令索引映射为最终指令偏移；
  /// 越界时回退到 fallback（通常为指令总数）。
  /// （原 `methods/bc_function_remap_local_pcs.rs` 的自由函数，abs-r139 收为方法。）
  pub(crate) fn remap_local_pcs(&mut self, insns_pc: &[u32], fallback: u32) {
    let remap = |pc: u32| {
      if pc < insns_pc.len() as u32 {
        insns_pc[pc as usize]
      } else {
        fallback
      }
    };

    for l in &mut self.local_types {
      l.startpc = remap(l.startpc);
      l.endpc = remap(l.endpc);
    }

    for l in &mut self.locals {
      l.startpc = remap(l.startpc);
      l.endpc = remap(l.endpc);
    }
  }
}

/// 指针反查下标同形坍缩（cpp `BcFunction::getBlockIndex/getInstIndex`）。
///
/// 只能用于本函数对应容器里的元素。cpp 靠 `LUAU_ASSERT` + 指针相减实现，release
/// 下断言被编译掉后对非同源指针做 `offset_from` 属 UB；这里改为逐元素 `ptr::eq`
/// 定位，找不到时 panic（可诊断，且不再有未定义行为）。签名保持 `-> u32` 不变。
/// 容器规模受可分配内存约束（元素各 ≥ 数十字节，u32::MAX 个需数 GB 连续堆），
/// 溢出分支仅为防御，expect 带上下文不静默截断。
///
/// （b）镜像定形——cpp 同形访问器全仓零消费，为免降级触发 dead_code 升级而保持
/// `pub`；非降级对象，勿删（批次账 b28-bc-rt-tail）。
macro_rules! reverse_index {
  ($vis:vis fn $name:ident($field:ident, $ty:ty)) => {
    impl BcFunction<'_> {
      $vis fn $name(&self, item: &$ty) -> u32 {
        let index = self
          .$field
          .iter()
          .position(|candidate| ptr::eq(candidate, item))
          .expect(concat!(
            "BcFunction::",
            stringify!($name),
            ": item is not from this function"
          ));
        u32::try_from(index).expect(concat!(
          "BcFunction::",
          stringify!($name),
          ": index overflows u32"
        ))
      }
    }
  };
}

reverse_index!(pub fn get_block_index(blocks, BcBlock));
reverse_index!(pub fn get_inst_index(instructions, BcInst));
