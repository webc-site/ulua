use ulua_analysis::records::magic_function::MagicFunction;

#[derive(Debug)]
#[repr(C)]
pub struct MagicInstanceIsA {
  pub base: MagicFunction,
}
