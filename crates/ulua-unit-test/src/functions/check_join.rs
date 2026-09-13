use ulua_analysis::records::join::Join;

/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn check_join(j: *mut Join, def: &str, operands: &[&str]) {
  assert!(!j.is_null());

  unsafe {
    let join = &*j;

    // DefId = NotNull<Definition> = *mut SymDef; deref to reach SymDef methods.
    assert!((*join.definition).versioned_name() == def);
    assert!(join.operands.len() == operands.len());

    for (i, operand_def) in operands.iter().enumerate() {
      assert!((*join.operands[i]).versioned_name() == *operand_def);
    }
  }
}
