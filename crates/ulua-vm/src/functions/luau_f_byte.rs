use crate::{
  enums::value_view::ValueView,
  functions::{c_slice, c_slice_mut},
  macros::{getstr::getstr, luau_f_arm::luau_f_arm, setnvalue::setnvalue},
};

luau_f_arm! {
  /// C++ `luauF_byte`（`lbuiltins.cpp:803`）：`string.byte` 的快速调用实现。
  ///
  /// §11 pass B（字符串内建读簇）：快速路径的 `ttisstring! + tsvalue!`、`ttisnumber! +
  /// nvalue!` tag/payload 读链收敛为 [`ValueView::String`]/[`ValueView::Number`] 变体
  /// match；结果槽写入仍走 `setnvalue!`（写宏不在本轮迁移范围）。
  ///
  /// # Safety
  /// `l` 为当前快速调用的存活 `lua_State`；`arg0` 与 `args` 指向的实参、`res` 结果槽均有效可读/可写，实参数与 `nparams`、结果数与 `nresults` 相符；`args` 为 `None` 表示 FASTCALL1 的单实参派发（无第二参数槽）。
  pub fn luau_f_byte [] (res, arg0, nresults, args, nparams) => args {
    // Safety: 契约保证 `arg0`/`args` 指向存活实参 TValue（串数据界内可读），`res` 为可写结果槽
    unsafe {
      if nparams >= 2
        && let (ValueView::String(ts), ValueView::Number(a2)) = (
          ValueView::from_tvalue(&*arg0),
          ValueView::from_tvalue(&*args),
        )
      {
        let i = a2 as i32;
        // cpp: `int j = (nparams >= 3) ? (ttisnumber(args + 1) ? int(nvalue(args + 1)) : 0) : i;`
        // 显式给了第二个实参但它不是数字时 j 取 0，`j >= i` 必不成立从而回退慢路径，
        // 交给 `string.byte` 的 Lua 实现按 `luaL_optinteger` 抛出「无法转换」错误；
        // 若兜底为 i，会跳过第二参的 tostring 强转，产生错误区间。
        let j = if nparams < 3 {
          i
        } else if let ValueView::Number(a3) = ValueView::from_tvalue(&*args.add(1)) {
          a3 as i32
        } else {
          0
        };

        if i >= 1 && j >= i && j <= ts.len as i32 {
          let c = j - i + 1;
          let s = getstr(ts as *const _);

          // for vararg returns, we only support a single result
          // this is because this frees us from concerns about stack space
          if c == if nresults < 0 { 1 } else { nresults } {
            // 字节窗口与结果槽各一次切片定界后 zip 迭代，替代逐轮 res.add(k)/s.add(i+k-1) 双侧指针算术；
            // 区间分别由上方 j<=len 与「结果数==nresults 且 res 可写」的契约保证界内
            let bytes = c_slice(s.add((i - 1) as usize), c as usize);
            let outs = c_slice_mut(res, c as usize);
            for (slot, &byte) in outs.iter_mut().zip(bytes) {
              setnvalue!(slot, byte as u8 as f64);
            }

            return c;
          }
        }
      }

      -1
    }
  }
}
