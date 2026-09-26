use crate::{
  macros::path_builder_step, records::pack_slice::PackSlice, type_aliases::component::Component,
};

path_builder_step!(
  pack_slice(start_index: usize) => Component::PackSlice(PackSlice { start_index }),
);
