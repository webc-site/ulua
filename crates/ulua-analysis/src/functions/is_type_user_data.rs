use ulua_vm::{
  functions::{lua_isuserdata::lua_isuserdata, lua_touserdatatagged::lua_touserdatatagged},
  records::lua_state::LuaState,
};
pub fn is_type_user_data(l: &mut LuaState, idx: i32) -> bool {
  // kTypeUserdataTag is a constant used for Luau Type Function userdata.
  const K_TYPE_USERDATA_TAG: i32 = 42;

  // Safety: `l.as_mut_ptr()` 只是从 `&mut l` 这一独占借用重新出借 C 句柄（不改变地址）；
  // `lua_touserdatatagged` 按 VM 约定以 `l`+`idx` 索引栈，tag 不匹配时返回 null，此处仅做
  // `is_null()` 判定、不解引用返回指针。单线程串行，无别名冲突。
  if lua_isuserdata(l, idx) == 0 {
    return false;
  }

  let result = unsafe { lua_touserdatatagged(l.as_mut_ptr(), idx, K_TYPE_USERDATA_TAG) };

  !result.is_null()
}
