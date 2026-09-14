use core::ffi::c_int;

use crate::{
  macros::{
    lua_vector_size::LUA_VECTOR_SIZE, setvvalue::setvvalue, ttisvector::ttisvector, vvalue::vvalue,
  },
  type_aliases::{lua_state::LuaState, stk_id::StkId, t_value::TValue},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn luau_f_vectormin(
  _l: *mut LuaState,
  res: StkId,
  arg0: *mut TValue,
  nresults: c_int,
  args: StkId,
  nparams: c_int,
) -> c_int {
  unsafe {
    if nparams >= 2 && nresults <= 1 && ttisvector!(arg0) && ttisvector!(args) {
      let a = vvalue!(arg0).as_ptr();
      let b = vvalue!(args).as_ptr();

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
        if !ttisvector!(args.offset(i as isize - 2)) {
          return -1;
        }

        let c = vvalue!(args.offset(i as isize - 2)).as_ptr();

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

pub use luau_f_vectormin as luauF_vectormin;
