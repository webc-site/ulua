//! Source: `Bytecode/src/BytecodeBuilder.cpp:1487-1503` (hand-ported).
//!
//! ```cpp
//! uint8_t BytecodeBuilder::getVersion()
//! {
//!     if (FFlag::DebugLuauUserDefinedClasses)
//!         return LBC_VERSION_CLASSES;
//!     if (FFlag::LuauCompileFastpcall)
//!         return 14;
//!     if (FFlag::LuauCompileEmitVectorDouble)
//!         return 13;
//!     if (FFlag::LuauBytecodeCostModel)
//!         return 12;
//!     if (FFlag::LuauEmitCallFeedback)
//!         return 11;
//!     return LBC_VERSION_TARGET;
//! }
//! ```
//!
//! `LuauCompileFastpcall` / `LuauCompileEmitVectorDouble` / `LuauBytecodeCostModel`
//! 三个分支保留（flag 定义见 `ulua-common/src/fflag.rs`，默认 false）——旧版
//! 移植的 `UdataDirect→9 / IntegerType2→8 / DuptableConstantPack2→7` 逐 flag
//! 链已被上游吸收进 `LBC_VERSION_TARGET = 9`，逐 flag bump 属过期语义。
use ulua_common::{
  enums::luau_bytecode_tag::{LBC_VERSION_CLASSES, LBC_VERSION_TARGET},
  fflag,
};

use crate::records::bytecode_builder::BytecodeBuilder;

impl BytecodeBuilder {
  pub fn get_version(&self) -> u8 {
    if fflag::DebugLuauUserDefinedClasses.get() {
      return LBC_VERSION_CLASSES.0 as u8;
    }

    if fflag::LuauCompileFastpcall.get() {
      return 14;
    }
    if fflag::LuauCompileEmitVectorDouble.get() {
      return 13;
    }
    if fflag::LuauBytecodeCostModel.get() {
      return 12;
    }

    if fflag::LuauEmitCallFeedback.get() {
      return 11;
    }

    LBC_VERSION_TARGET.0 as u8
  }
}
