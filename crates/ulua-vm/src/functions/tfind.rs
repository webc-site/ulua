use crate::{
  enums::lua_type::LuaType,
  functions::{lua_h_getnum::lua_h_getnum, lua_l_optinteger::lua_l_optinteger},
  macros::{equalobj::equalobj, lua_lib_fn::lua_lib_fn},
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 的存活/独占前提已由 `&mut` 接收者类型承载（r16-v41 收形）；屏障仍保留是因为 1 号槽表句柄
/// `t`、`lua_h_getnum` 所得表内元素裸指针 `e` 与 2 号槽操作数裸指针 `v` 三窗须跨
/// `equalobj!` → `lua_v_equalval` 存活，而被比较值若带 `__eq` 元方法，该调用会执行任意 Lua——
/// 重哈希即搬移表存储、扩栈即搬移栈基址，三窗随之悬空（`lua_v_equalval` 自契亦声明「操作数槽在
/// 调用期间不得迁移」，cpp `ltablib.cpp:598` 同源隐患，属调用方无从外证的内部不变量）。
/// 窗侧仅存的守卫：`v` 每轮现读 `base`（本轮调用若已搬栈，下轮读到的即新基址，与 cpp 逐位一致）。
/// 余下调用序前提：`l` 须处于可抛错受保护帧——栈 1 号位为 table、2 号位任意值（`check_type`/
/// `check_any` 校验否则发散），3 号位可选整数下界，`arg_error` 界失即发散。
/// cpp/VM/src/ltablib.cpp:598 tfind。
pub unsafe fn tfind(l: &mut LuaState) -> i32 {
  l.check_type(1, LuaType::Table);
  l.check_any(2);
  let init = lua_l_optinteger(l, 3, 1);
  if init < 1 {
    l.arg_error(3, "index out of range");
  }

  // SAFETY: `lp` 自 `&mut` 入口就地转手、全程只用 `lp`（tunpack r16-v41 锚定形判例）；`t` 借自
  // check_type 刚钉住的 1 号槽存活表，`e` 为表内可读槽或静态 nil（lua_h_getnum 入约），`v` 为
  // check_any 保有的 2 号槽；三窗跨 `equalobj` 的存续由上方契约承载。
  let found = unsafe {
    let lp = l.as_mut_ptr();
    let t = (*(*lp).base).as_table_ptr();

    let mut found = None;
    // cpp `for (int i = init;; ++i)` 的无条件 `i++` 在 INT_MAX 处为有符号溢出 UB（上游
    // ltablib.cpp:608）；INT_MAX 之后本无合法下标可试，区间收口即同形干净终止。
    for i in init..=i32::MAX {
      let e = lua_h_getnum(&*t, i);
      if (*e).is_nil() {
        break;
      }

      let v = (*lp).base.offset(1);

      if equalobj!(lp, v, e) {
        found = Some(i);
        break;
      }
    }

    found
  };

  match found {
    Some(i) => l.push_integer(i),
    None => l.push_nil(),
  }

  1
}

lua_lib_fn!(pub fn tfind @ref, tfind_arm);
