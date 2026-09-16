#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum DumpFlags {
  Code = 1 << 0,
  Lines = 1 << 1,
  Source = 1 << 2,
  Locals = 1 << 3,
  Remarks = 1 << 4,
  Types = 1 << 5,
  Constants = 1 << 6,
}
