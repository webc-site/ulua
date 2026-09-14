use ulua_analysis::{records::block::Block, type_aliases::instruction::InstructionMember};
/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn require_inst<T: InstructionMember>(block: *mut Block, idx: usize) -> *mut T {
  let instructions = unsafe { (*block).get_instructions() };
  assert!(idx < instructions.len());

  let inst = instructions[idx];
  let typed = T::get_if(unsafe { &*inst });
  assert!(typed.is_some());

  typed.unwrap() as *const T as *mut T
}
