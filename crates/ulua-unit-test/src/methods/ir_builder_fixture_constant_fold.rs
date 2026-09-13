use ulua_code_gen::{
  enums::ir_block_kind::IrBlockKind,
  functions::{
    apply_substitutions_ir_utils_alt_b::apply_substitutions_ir_function_ir_inst,
    fold_constants::fold_constants,
  },
  records::{ir_block::IrBlock, ir_function::IrFunction},
};

use crate::records::ir_builder_fixture::IrBuilderFixture;

impl IrBuilderFixture {
  /// C++ `IrBuilderFixture::constantFold`: walk every live block's instruction
  /// range, applying substitutions then folding constants. `build`, `function`
  /// and `block` alias mutably (as in the translated `const_prop_in_block`),
  /// expressed here with raw pointers (the IR types are crate-private to
  /// luau-code-gen, so they are reached only through `addr_of_mut!`).
  pub fn constant_fold(&mut self) {
    use core::ptr::addr_of_mut;
    let build = addr_of_mut!(self.build);
    unsafe {
      let function: *mut IrFunction = addr_of_mut!((*build).function);
      let block_count = (*function).blocks.len();
      for bi in 0..block_count {
        // Explicit `&mut (...)` before indexing (as in the translated
        // `const_prop_in_block`) so the index doesn't create an *implicit*
        // autoref through the raw-pointer deref.
        let block: *mut IrBlock = &mut (&mut (*function).blocks)[bi];
        if (*block).kind == IrBlockKind::Dead {
          continue;
        }
        let start = (*block).start;
        let finish = (*block).finish;
        for i in start..=finish {
          let inst = &mut (&mut (*function).instructions)[i as usize] as *mut _;
          apply_substitutions_ir_function_ir_inst(&mut *function, &mut *inst);
          fold_constants(&mut *build, &mut *function, &mut *block, i);
        }
      }
    }
  }
}
