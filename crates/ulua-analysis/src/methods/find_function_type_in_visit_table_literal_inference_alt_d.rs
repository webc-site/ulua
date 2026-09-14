use core::ptr::{from_ref, null, null_mut};

use crate::{
  functions::{get_type_alt_j::get_type_id, size_type_pack::size},
  records::{find_function_type_in::FindFunctionTypeIn, function_type::FunctionType},
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};
impl FindFunctionTypeIn {
  pub fn visit_type_id_function_type(&mut self, ty: TypeId, ftv: &FunctionType) -> bool {
    // This logic is a little clowny.
    //
    // For bidirectional inference we're trying to _guess_ what the user
    // is intending so that we can give decent results. For functions, we
    // will error if the user doesn't provide exactly the correct number of
    // arguments.
    //
    // The original C++ implementation attempts to prefer candidates with
    // an arg count closest to the lambda parameter count.
    let candidate = self.candidate;

    if candidate.is_null()
      || (unsafe { candidate_arg_count(candidate) } as i32 - self.number_of_lambda_parameters).abs()
        > (ftv_arg_count(ftv) as i32 - self.number_of_lambda_parameters).abs()
    {
      // SAFETY: candidate_arg_count 解引用句柄指向的存活 FunctionType。
      self.candidate = get_type_id::<FunctionType>(ty).map_or(null(), from_ref);
      return false;
    }

    false
  }
}

/// SAFETY: candidate 必须指向存活的 FunctionType。
unsafe fn candidate_arg_count(candidate: *const FunctionType) -> usize {
  // SAFETY: 由调用方契约保证。
  unsafe {
    let c = &*candidate;
    type_pack_len(c.arg_types)
  }
}

fn ftv_arg_count(ftv: &FunctionType) -> usize {
  type_pack_len(ftv.arg_types)
}

fn type_pack_len(arg_types: TypePackId) -> usize {
  // C++ uses `size(argTypes)` (TypePack.cpp:308) — count the bound head types
  // and follow the tail. The default-log overload passes `log = nullptr`.
  unsafe { size(arg_types, null_mut()) }
}
