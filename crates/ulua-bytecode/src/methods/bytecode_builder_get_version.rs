use ulua_common::{FFlag, enums::luau_bytecode_tag::LBC_VERSION_TARGET};

use crate::records::bytecode_builder::BytecodeBuilder;

impl BytecodeBuilder {
  pub fn get_version(&self) -> u8 {
    if FFlag::LuauEmitCallFeedback.get() {
      return 11;
    }

    if FFlag::DebugLuauUserDefinedClasses.get() {
      return 10;
    }

    if FFlag::LuauCompileUdataDirect.get() {
      return 9;
    }

    // LBC_CONSTANT_INTEGER requires version 8
    if FFlag::LuauIntegerType2.get() {
      return 8;
    }

    // LBC_CONSTANT_TABLE_WITH_CONSTANTS requires version 7
    if FFlag::LuauCompileDuptableConstantPack2.get() {
      return 7;
    }

    LBC_VERSION_TARGET.0 as u8
  }
}
