//! Generated skeleton item.
//! Node: `cxx:Enum:Luau.Common:Common/include/Luau/Bytecode.h:747:luau_proto_flag`
//! Source: `Common/include/Luau/Bytecode.h`
//! Graph edges:
//! - declared_by: source_file Common/include/Luau/Bytecode.h
//! - incoming:
//!   - declares <- source_file Common/include/Luau/Bytecode.h

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum LuauProtoFlag {
  /// used to tag main Proto for modules with --!native
  LpfNativeModule = 1 << 0,
  /// used to tag individual protos as not profitable to compile natively
  LpfNativeCold = 1 << 1,
  /// used to tag main Proto for modules that have at least one function with native attribute
  LpfNativeFunction = 1 << 2,
  /// function can be inlined
  LpfInlinable = 1 << 3,
}

impl LuauProtoFlag {
  pub const LPF_NATIVE_MODULE: Self = Self::LpfNativeModule;
  pub const LPF_NATIVE_COLD: Self = Self::LpfNativeCold;
  pub const LPF_NATIVE_FUNCTION: Self = Self::LpfNativeFunction;
  pub const LPF_INLINABLE: Self = Self::LpfInlinable;
}
