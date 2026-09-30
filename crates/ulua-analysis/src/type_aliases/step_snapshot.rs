use ulua_common::records::variant::Variant2;

use crate::records::{
  constraint_step_snapshot::ConstraintStepSnapshot,
  generalize_step_snapshot::GeneralizeStepSnapshot,
};

pub type StepSnapshot = Variant2<ConstraintStepSnapshot, GeneralizeStepSnapshot>;
