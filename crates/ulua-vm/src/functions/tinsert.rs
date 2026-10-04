use crate::{
  enums::lua_type::LuaType,
  functions::{lua_rawseti::lua_rawseti, moveelements::moveelements},
  macros::{lua_l_error::luaL_error, lua_lib_fn::lua_lib_fn},
  records::lua_state::LuaState,
};

/// 调用序契约（正确性，非内存安全——`l` 的存活/独占前提已由 `&mut` 接收者类型承载；收形后取参/
/// 判型/落值全经安全门面，体内两处 `unsafe` 皆为自由函数取参面的裸指针转手，无越窗别名）：`l` 须
/// 处于可抛错的受保护帧，栈 1 号位为 table（`check_type` 校验、非表即抛错发散），故交与
/// `moveelements` 的 srct/dstt=1 槽必为表且满足其 `f <= e + 1` 入约；`check_integer`/
/// `arg_check`/`luaL_error` 失败即抛错不返回，`moveelements`/`lua_rawseti` 可触发再哈希与 GC。
/// cpp/VM/src/ltablib.cpp:219 tinsert。
pub fn tinsert(l: &mut LuaState) -> i32 {
  l.check_type(1, LuaType::Table);
  let n = l.obj_len(1) as i32;

  let pos = match l.get_top() {
    2 => n + 1, // called with only 2 arguments: insert new element at the end
    3 => {
      // 2nd argument is the position
      let pos = l.check_integer(2);

      // move up elements if necessary
      if 1 <= pos && pos <= n {
        // SAFETY: `as_mut_ptr` 自 `&mut` 独占借用就地派生，借用窗止于本次调用；槽 1 已由上方
        // `check_type` 保证为表，区间界内前提见本函数契约。
        unsafe { moveelements(l.as_mut_ptr(), 1, 1, pos, n, pos + 1, false) };
      }
      pos
    }
    _ => unsafe { luaL_error!(l.as_mut_ptr(), "wrong number of arguments to 'insert'") },
  };

  lua_rawseti(l, 1, pos); // t[pos] = v
  0
}

lua_lib_fn!(pub fn tinsert @ref, tinsert_arm);
