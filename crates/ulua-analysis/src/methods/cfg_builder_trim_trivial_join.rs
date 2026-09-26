use crate::{records::cfg_builder::CfgBuilder, type_aliases::instr_id::InstrId};

impl CfgBuilder {
  pub fn trim_trivial_join(&mut self, _j: InstrId) {}
}
