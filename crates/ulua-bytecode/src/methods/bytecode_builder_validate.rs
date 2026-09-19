use ulua_common::macros::luau_assertenabled::LUAU_ASSERTENABLED;

use crate::records::bytecode_builder::BytecodeBuilder;

impl BytecodeBuilder {
  /// cpp `BytecodeBuilder::validate()`：整段定义都在 `#ifdef LUAU_ASSERTENABLED`
  /// 之内，release 下不参与编译。这里用同一个运行时开关提前返回，`LUAU_ASSERTENABLED`
  /// 是 `const bool`，release 下整个函数体（含 `validate_instructions` 里 400 余行
  /// 校验）会被判为死分支消除，零开销。
  ///
  /// 注意：必须在 `encoder.encode` 置换操作码**之前**调用，否则 `LUAU_INSN_OP/D`
  /// 解出的全是置换后的值。
  pub fn validate(&self) {
    if !LUAU_ASSERTENABLED {
      return;
    }

    self.validate_instructions();
    self.validate_variadic();
  }
}
