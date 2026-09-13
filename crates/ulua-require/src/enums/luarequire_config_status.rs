#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub enum LuarequireConfigStatus {
  ConfigAbsent,
  ConfigAmbiguous,
  ConfigPresentJson,
  ConfigPresentLuau,
}

pub use LuarequireConfigStatus as luarequire_ConfigStatus;
