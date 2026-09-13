use crate::records::{normalized_type::NormalizedType, normalizer::Normalizer};

impl Normalizer {
  pub fn clear_normal(&mut self, norm: &mut NormalizedType) {
    let builtin_types = norm.builtin_types;
    let never_type = unsafe { (*builtin_types).never_type };

    norm.tops = never_type;
    norm.booleans = never_type;
    norm.extern_types.reset_to_never();
    norm.errors = never_type;
    norm.nils = never_type;
    norm.numbers = never_type;
    norm.integers = never_type;
    norm.strings.reset_to_never();
    norm.threads = never_type;
    norm.buffers = never_type;
    norm.tables.clear();
    norm.functions.reset_to_never();
    norm.tyvars.clear();
  }
}
