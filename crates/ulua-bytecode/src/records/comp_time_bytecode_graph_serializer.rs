use std::vec::Vec;

use crate::records::{
  bc_function::BcFunction, bytecode_builder::BytecodeBuilder,
  bytecode_graph_serializer::BytecodeGraphSerializer,
};

/// cpp `CompTimeBytecodeGraphSerializer`（BytecodeGraph.cpp:302）：编译期把
/// `BcVmConst` 下标映射为 bcb 常量表 id。cpp 靠虚函数 `getVmConstInputRaw` 覆写，
/// Rust 无虚派发：基类 `consts: Option<Vec<u32>>` 命中时即做同一映射，
/// 故此处只负责注入 consts，不再重复实现覆写点。
#[derive(Debug)]
pub(crate) struct CompTimeBytecodeGraphSerializer<'a, 'b, 'f> {
  pub(crate) base: BytecodeGraphSerializer<'a, 'b, 'f>,
}

impl<'a, 'b, 'f> CompTimeBytecodeGraphSerializer<'a, 'b, 'f> {
  pub fn new(
    bcb: &'a mut BytecodeBuilder<'b>,
    fn_: &'a mut BcFunction<'f>,
    consts: Vec<u32>,
  ) -> Self {
    let mut base = BytecodeGraphSerializer::new(bcb, fn_);
    base.consts = Some(consts);

    Self { base }
  }

  pub(crate) fn emit_bytecode(&mut self) -> Vec<u32> {
    self.base.emit_bytecode()
  }

  pub fn error(&self) -> bool {
    self.base.error
  }
}
