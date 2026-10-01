use core::slice::from_raw_parts;

use crate::macros::{
  lua_vector_size::LUA_VECTOR_SIZE, luau_f_arm::luau_f_arm, setvvalue::setvvalue,
};

luau_f_arm! {
  /// C++ `luauF_vectormin`（`lbuiltins.cpp:1637`）：`vector.min` 的快速调用实现。
  ///
  /// R-D V1：lane 读统一走 `TValue::as_vector_ref()` 的安全分量切片视图（`&[f32; 3]`），
  /// args 尾随实参槽经入口 `from_raw_parts` 收成只读窗，22 处裸指针 lane 算术已消灭；
  /// 第 4 lane 的两处常量门与 C++ `#if LUA_VECTOR_SIZE == 4` 逐字同形，3 分量构建下
  /// 连同 `x[3]` 索引在编译期消除（探针验证无 `unconditional_panic` 告警），且该分支
  /// 若未来被激活，数组索引将越界读从 UB 降级为 panic，不触碰 provenance 外内存。
  ///
  /// # Safety
  /// `l` 为当前快速调用的存活 `lua_State`；`arg0` 与 `args` 指向的实参、`res` 结果槽均有效可读/可写，实参数与 `nparams`、结果数与 `nresults` 相符；`args` 为 `None` 表示 FASTCALL1 的单实参派发（无第二参数槽）。
  pub fn luau_f_vectormin [] (res, arg0, nresults, args, nparams) => args {
    // SAFETY: 契约保证 `arg0`/`args` 指向存活实参 TValue（各 3 个 f32 lane 可读）、`res` 结果槽
    // 16 字节 payload 写回界内；`from_raw_parts(args, nparams - 1)` 的读窗由下方 `nparams >= 2`
    // 门保证长度至少为 1，且契约保证帧已推入全部 nparams 个实参——`args` 起连续 `nparams - 1`
    // 槽（第 2..n 个实参）均为存活已初始化 TValue，与 C++ `args + (i - 2)` 的连读范围同形。
    // 本块是全函数唯一裸指针边界：槽窗与各 lane 读数收敛后，体内不再产生指针算术。
    unsafe {
      if nparams >= 2 && nresults <= 1 && (*arg0).is_vector() && (*args).is_vector() {
        let rest = from_raw_parts(args, (nparams - 1) as usize);
        let a = (*arg0).as_vector_ref();
        let b = (*args).as_vector_ref();

        let mut result = [0.0f32; 4];

        result[0] = if b[0] < a[0] { b[0] } else { a[0] };
        result[1] = if b[1] < a[1] { b[1] } else { a[1] };
        result[2] = if b[2] < a[2] { b[2] } else { a[2] };

        result[3] = if LUA_VECTOR_SIZE == 4 {
          if b[3] < a[3] { b[3] } else { a[3] }
        } else {
          0.0f32
        };

        for i in 3..=nparams {
          let cslot = &rest[(i - 2) as usize];
          if !cslot.is_vector() {
            return -1;
          }

          let c = cslot.as_vector_ref();

          result[0] = if c[0] < result[0] { c[0] } else { result[0] };
          result[1] = if c[1] < result[1] { c[1] } else { result[1] };
          result[2] = if c[2] < result[2] { c[2] } else { result[2] };
          if LUA_VECTOR_SIZE == 4 {
            result[3] = if c[3] < result[3] { c[3] } else { result[3] };
          }
        }

        setvvalue!(res, result[0], result[1], result[2], result[3]);
        return 1;
      }

      -1
    }
  }
}
