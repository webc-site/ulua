#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct unw_dynamic_unwind_sections_t {
  pub dso_base: usize,
  pub dwarf_section: usize,
  pub dwarf_section_length: usize,
  pub compact_unwind_section: usize,
  pub compact_unwind_section_length: usize,
}

pub type UnwDynamicUnwindSectionsT = unw_dynamic_unwind_sections_t;
