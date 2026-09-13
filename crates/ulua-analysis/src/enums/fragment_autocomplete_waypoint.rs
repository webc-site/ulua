#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum FragmentAutocompleteWaypoint {
  ParseFragmentEnd,
  CloneModuleStart,
  CloneModuleEnd,
  DfgBuildEnd,
  CloneAndSquashScopeStart,
  CloneAndSquashScopeEnd,
  ConstraintSolverStart,
  ConstraintSolverEnd,
  TypecheckFragmentEnd,
  AutocompleteEnd,
  COUNT,
}
