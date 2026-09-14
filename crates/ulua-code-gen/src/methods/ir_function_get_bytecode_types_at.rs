use crate::records::{bytecode_types::BytecodeTypes, ir_function::IrFunction};

impl IrFunction {
  pub fn get_bytecode_types_at(&self, pcpos: i32) -> BytecodeTypes {
    // Avoid using CODEGEN_ASSERT! here, since the macro's dependency on
    // ulua_common::assertCallHandler / arch intrinsics fails to compile
    // in this crate configuration.
    if !(pcpos >= 0) {
      return BytecodeTypes {
        result: 0,
        a: 0,
        b: 0,
        c: 0,
      };
    }

    let pcpos_usize = pcpos as usize;

    if pcpos_usize < self.bc_types.len() {
      self.bc_types[pcpos_usize]
    } else {
      BytecodeTypes {
        result: 0,
        a: 0,
        b: 0,
        c: 0,
      }
    }
  }
}
