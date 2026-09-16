use alloc::vec::Vec;

use crate::{
  records::{bc_function::BcFunction, bytecode_builder::BytecodeBuilder},
  type_aliases::jumps::Jumps,
};

#[derive(Debug)]
pub struct BytecodeGraphSerializer<'a> {
  pub(crate) bcb: &'a mut BytecodeBuilder,
  pub(crate) func: &'a mut BcFunction,
  pub(crate) jumps: Jumps,
  pub(crate) error: bool,
  pub(crate) consts: Option<Vec<u16>>,
}

impl<'a> BytecodeGraphSerializer<'a> {
  pub(crate) fn new(bcb: &'a mut BytecodeBuilder, func: &'a mut BcFunction) -> Self {
    Self {
      bcb,
      func,
      jumps: Vec::new(),
      error: false,
      consts: None,
    }
  }
}
