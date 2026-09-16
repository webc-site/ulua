use crate::{
  enums::pack_field::PackField, records::path_builder::PathBuilder,
  type_aliases::component::Component,
};

pub trait PathBuilderRets {
  fn rets(&mut self) -> &mut Self;
}

impl PathBuilderRets for PathBuilder {
  fn rets(&mut self) -> &mut Self {
    self
      .components
      .push(Component::PackField(PackField::Returns));
    self
  }
}
