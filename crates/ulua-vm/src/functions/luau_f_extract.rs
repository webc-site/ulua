use crate::{
  enums::value_view::ValueView,
  macros::{luai_num_2_unsigned::luai_num2unsigned, luau_f_arm::luau_f_arm, setnvalue::setnvalue},
};

luau_f_arm! {
  /// C++ `luauF_extract`（`lbuiltins.cpp:605`）：`bit32.extract` 的快速调用实现。
  ///
  /// §11 pass B（位运算内建读簇）：快速路径的 `ttisnumber! + nvalue!` tag/payload 读链
  /// 收敛为 [`ValueView::Number`] 变体 match——变体即 tag，payload 由视图带出；结果槽写入
  /// 仍走 `setnvalue!`（写宏不在本轮迁移范围）。视图借用止于各 `if let` 内，不外溢到写入。
  ///
  /// # Safety
  /// `l` 为当前快速调用的存活 `lua_State`；`arg0` 与 `args` 指向的实参、`res` 结果槽均有效可读/可写，实参数与 `nparams`、结果数与 `nresults` 相符；`args` 为 `None` 表示 FASTCALL1 的单实参派发（无第二参数槽）。
  pub fn luau_f_extract [] (res, arg0, nresults, args, nparams) => args {
    // Safety: 契约保证 `arg0`/`args` 指向存活实参 TValue（串数据界内可读），`res` 为可写结果槽
    unsafe {
      if nparams >= 2
        && nresults <= 1
        && let (ValueView::Number(a1), ValueView::Number(a2)) = (
          ValueView::from_tvalue(&*arg0),
          ValueView::from_tvalue(&*args),
        )
      {
        let n = luai_num2unsigned(a1);
        let f = a2 as i32;

        if nparams == 2 {
          if (f as u32) < 32 {
            let m: u32 = 1;
            let r: u32 = (n >> (f as u32)) & m;

            setnvalue!(res, r as f64);
            return 1;
          }
        } else if let ValueView::Number(a3) = ValueView::from_tvalue(&*args.offset(1)) {
          let w = a3 as i32;

          if f >= 0 && w > 0 && f as i64 + w as i64 <= 32 {
            let m: u32 = !(0xFFFF_FFFE_u32 << (w - 1));
            let r: u32 = (n >> (f as u32)) & m;

            setnvalue!(res, r as f64);
            return 1;
          }
        }
      }

      -1
    }
  }
}
