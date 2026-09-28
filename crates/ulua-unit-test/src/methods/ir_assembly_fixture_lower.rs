use alloc::string::String;
use core::ptr::null_mut;

use ulua_code_gen::functions::{
  get_assembly::assembly_text, get_assembly_from_ir::get_assembly_from_ir,
};

use crate::{
  functions::{
    normalize_state_offsets::normalize_state_offsets,
    strip_lines_containing::strip_lines_containing,
  },
  records::ir_assembly_fixture::IrAssemblyFixture,
};
impl IrAssemblyFixture {
  pub fn lower(&mut self) -> String {
    let mut text = assembly_text(unsafe {
      // Safety: get_assembly_from_ir 的 # Safety 契约由本调用满足：&mut self.build 是借用期内字段独占可变借用（addr 由安全 &mut 转交），options 传值拷贝，null_mut() 对应 cpp 可空 context 形参（callee 约定不解引用）；返回值立即转 String 拷贝。
      get_assembly_from_ir(&mut self.build, self.options.clone(), null_mut())
    });
    strip_lines_containing(&mut text, "; skipping ");
    normalize_state_offsets(&mut text);
    text
  }
}
