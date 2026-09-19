use ulua_analysis::records::join::Join;

/// 断言 join 节点的定义名与操作数定义名逐一匹配。
pub fn check_join(j: &Join, def: &str, operands: &[&str]) {
  // DefId = NotNull<Definition> = *mut SymDef; deref to reach SymDef methods.
  assert_eq!(unsafe { (*j.definition).versioned_name() }, def);
  assert_eq!(j.operands.len(), operands.len());

  for (i, operand_def) in operands.iter().enumerate() {
    assert_eq!(unsafe { (*j.operands[i]).versioned_name() }, *operand_def);
  }
}
