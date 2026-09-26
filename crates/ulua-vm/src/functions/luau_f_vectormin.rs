use crate::macros::{
  lua_vector_size::LUA_VECTOR_SIZE, luau_f_arm::luau_f_arm, setvvalue::setvvalue,
};

luau_f_arm! {
  /// C++ `luauF_vectormin`（`lbuiltins.cpp:1637`）：`vector.min` 的快速调用实现。
  ///
  /// # Safety
  /// `l` 为当前快速调用的存活 `lua_State`；`arg0` 与 `args` 指向的实参、`res` 结果槽均有效可读/可写，实参数与 `nparams`、结果数与 `nresults` 相符；`args` 为 `None` 表示 FASTCALL1 的单实参派发（无第二参数槽）。
  pub fn luau_f_vectormin [] (res, arg0, nresults, args, nparams) => args {
    // Safety: 契约保证 `arg0`/`args` 指向存活实参 TValue（各 4 个 f32 可读），结果向量 16 字节 payload 写回 res 界内
    unsafe {
      if nparams >= 2 && nresults <= 1 && (*arg0).is_vector() && (*args).is_vector() {
        let a = (*arg0).as_vector_ref().as_ptr();
        let b = (*args).as_vector_ref().as_ptr();

        let mut result = [0.0f32; 4];

        result[0] = if (*b.offset(0)) < (*a.offset(0)) {
          *b.offset(0)
        } else {
          *a.offset(0)
        };
        result[1] = if (*b.offset(1)) < (*a.offset(1)) {
          *b.offset(1)
        } else {
          *a.offset(1)
        };
        result[2] = if (*b.offset(2)) < (*a.offset(2)) {
          *b.offset(2)
        } else {
          *a.offset(2)
        };

        result[3] = if LUA_VECTOR_SIZE == 4 {
          if (*b.offset(3)) < (*a.offset(3)) {
            *b.offset(3)
          } else {
            *a.offset(3)
          }
        } else {
          0.0f32
        };

        for i in 3..=nparams {
          if !(*args.offset(i as isize - 2)).is_vector() {
            return -1;
          }

          let c = (*args.offset(i as isize - 2)).as_vector_ref().as_ptr();

          result[0] = if (*c.offset(0)) < result[0] {
            *c.offset(0)
          } else {
            result[0]
          };
          result[1] = if (*c.offset(1)) < result[1] {
            *c.offset(1)
          } else {
            result[1]
          };
          result[2] = if (*c.offset(2)) < result[2] {
            *c.offset(2)
          } else {
            result[2]
          };
          if LUA_VECTOR_SIZE == 4 {
            result[3] = if (*c.offset(3)) < result[3] {
              *c.offset(3)
            } else {
              result[3]
            };
          }
        }

        setvvalue!(res, result[0], result[1], result[2], result[3]);
        return 1;
      }

      -1
    }
  }
}
