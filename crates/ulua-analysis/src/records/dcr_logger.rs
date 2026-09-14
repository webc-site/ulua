use alloc::vec::Vec;

use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  records::{
    constraint::Constraint, constraint_generation_log::ConstraintGenerationLog,
    to_string_options::ToStringOptions, type_check_log::TypeCheckLog, type_solve_log::TypeSolveLog,
  },
  type_aliases::constraint_block_target::ConstraintBlockTarget,
};
#[derive(Debug, Clone)]
pub struct DcrLogger {
  pub(crate) generation_log: ConstraintGenerationLog,
  pub(crate) constraint_blocks: DenseHashMap<*const Constraint, Vec<ConstraintBlockTarget>>,
  pub(crate) solve_log: TypeSolveLog,
  pub(crate) check_log: TypeCheckLog,
  pub(crate) opts: ToStringOptions,
}
