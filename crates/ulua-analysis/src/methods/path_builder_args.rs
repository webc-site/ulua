use crate::{
  enums::pack_field::PackField, records::path_builder::PathBuilder,
  type_aliases::component::Component,
};

pub trait PathBuilderArgs {
  fn args(&mut self) -> &mut Self;
}

impl PathBuilderArgs for PathBuilder {
  fn args(&mut self) -> &mut Self {
    self
      .components
      .push(Component::PackField(PackField::Arguments));
    self
  }
}
