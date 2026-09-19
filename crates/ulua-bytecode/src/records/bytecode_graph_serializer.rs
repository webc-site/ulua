use std::vec::Vec;

use crate::records::{
  bc_function::BcFunction, bytecode_builder::BytecodeBuilder, jump_info::JumpInfo,
};

#[derive(Debug)]
pub struct BytecodeGraphSerializer<'a> {
  pub(crate) bcb: &'a mut BytecodeBuilder,
  pub(crate) func: &'a mut BcFunction,
  pub(crate) jumps: Vec<JumpInfo>,
  pub(crate) error: bool,
  pub(crate) consts: Option<Vec<u32>>,
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
