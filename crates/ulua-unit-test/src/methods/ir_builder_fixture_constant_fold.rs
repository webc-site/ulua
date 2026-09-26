use ulua_code_gen::{
  enums::ir_block_kind::IrBlockKind,
  functions::{
    apply_substitutions_ir_utils::apply_substitutions_at, fold_constants::fold_constants,
  },
  records::{ir_builder::ConstantMap, ir_function::IrFunction},
};

use crate::records::ir_builder_fixture::IrBuilderFixture;

impl IrBuilderFixture {
  /// C++ `IrBuilderFixture::constantFold`: walk every live block's instruction
  /// range, applying substitutions then folding constants. `function` 与
  /// `constant_map` 是 `IrBuilder` 上互不相交的字段，按索引定位块与指令即可
  /// 满足借用检查（无需 cpp 多 `&mut` 重叠借用的裸指针别名手法）。
  pub fn constant_fold(&mut self) {
    let function = &mut self.build.function;
    let constant_map = &mut self.build.constant_map;
    let block_count = function.blocks.len();
    for bi in 0..block_count {
      if function.blocks[bi].kind != IrBlockKind::Dead {
        const_prop_in_block(function, constant_map, bi as u32);
      }
    }
  }
}

/// cpp `constPropInBlock` 对应物：改写单块 `[start, finish]` 指令区间。
fn const_prop_in_block(function: &mut IrFunction, constant_map: &mut ConstantMap, block_idx: u32) {
  // 块区间边界快照一次（与 cpp 循环头取 block->start/finish 语义一致）。
  let (start, finish) = {
    let block = &function.blocks[block_idx as usize];
    (block.start, block.finish)
  };
  for index in start..=finish {
    apply_substitutions_at(function, index);
    fold_constants(function, constant_map, block_idx, index);
  }
}
