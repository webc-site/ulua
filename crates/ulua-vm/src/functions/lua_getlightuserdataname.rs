use core::{ffi::c_char, ptr::null};

use crate::{
  macros::{api_check::api_check, getstr::getstr, lua_lutag_limit::LUA_LUTAG_LIMIT},
  records::{lua_state::LuaState, t_string::tstring},
};

/// `lua_getlightuserdataname`（cpp/VM/src/lapi.cpp 同名）：读回 `tag` 登记的 light
/// userdatum C 名字串首指针；未登记得 null。r16-v4 引用形前移取 `&LuaState`（注册表
/// 读数经 `gs_ref` 一句一借，视图不出取指针当句）；`tag` 须 `< LUA_LUTAG_LIMIT`
/// （`api_check` debug 断言；release 越界由数组安全索引 panic 兜住）。
/// 返回裸指针是 C 登记名语义本体，保留——出处契约：指针指向 `tag` 登记时 intern 的
/// `tstring` 柔性数据区（NUL 结尾，可读界 `len + 1`，见 `getstr` 契约），寿命随该
/// TString：登记存续期内可读，跨重入/GC/改名点后须现读，不得并持旧指针与新登记名。
pub fn lua_getlightuserdataname(l: &LuaState, tag: i32) -> *const c_char {
  api_check!(l, (tag as u32) < LUA_LUTAG_LIMIT as u32);

  // r16-b1 收编形制保持：注册表读数经 gs_ref 只读视图，同指针同值（见其契约）；
  // r16-v4：`(*l).global` 场域裸解引用收进门面，借用止于取回登记指针当句
  let name = l.gs_ref().lightuserdataname[tag as usize];
  if name.is_null() {
    null()
  } else {
    // SAFETY: 契约保证 `name` 为登记时 intern 的存活 TString（lightuserdataname 注册
    // API 语义），getstr 仅取柔性数据区首地址不解引用内容
    unsafe { getstr(name as *const tstring) }
  }
}
