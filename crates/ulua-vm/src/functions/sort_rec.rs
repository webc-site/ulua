use crate::{
  functions::{
    lua_l_error_l::lua_l_error_l, sort_heap::sort_heap, sort_less::sort_less, sort_swap::sort_swap,
  },
  records::lua_table::LuaTable,
  type_aliases::{lua_state::lua_State, sort_predicate::SortPredicate},
};

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub(crate) unsafe fn sort_rec(
  l: *mut lua_State,
  t: *mut LuaTable,
  mut lo: i32,
  mut u: i32,
  mut limit: i32,
  pred: SortPredicate,
) {
  unsafe {
    // sort range [lo..u] (inclusive, 0-based)
    while lo < u {
      // if the limit has been reached, quick sort is going over the permitted nlogn complexity,
      // so we fall back to heap sort
      if limit == 0 {
        sort_heap(l, t, lo, u, pred);
        return;
      }

      // sort elements a[lo], a[(lo+u)/2] and a[u]
      // note: this simultaneously acts as a small sort and a median selector
      if sort_less(l, t, u, lo, pred) != 0 {
        sort_swap(l, t, u, lo);
      }

      if u - lo == 1 {
        break; // only 2 elements
      }

      let m = lo + ((u - lo) >> 1); // midpoint

      if sort_less(l, t, m, lo, pred) != 0 {
        sort_swap(l, t, m, lo);
      } else if sort_less(l, t, u, m, pred) != 0 {
        sort_swap(l, t, m, u);
      }

      if u - lo == 2 {
        break; // only 3 elements
      }

      // here lo, m, u are ordered; m will become the new pivot
      let p = u - 1;
      sort_swap(l, t, m, u - 1); // pivot is now (and always) at u-1

      // a[lo] <= P == a[u-1] <= a[u], only need to sort from lo+1 to u-2
      let mut i = lo;
      let mut j = u - 1;

      loop {
        // invariant: a[lo..i] <= P <= a[j..u]
        // repeat ++i until a[i] >= P
        loop {
          i += 1;
          if sort_less(l, t, i, p, pred) != 0 {
            if i >= u {
              lua_l_error_l(
                l,
                c"invalid order function for sorting".as_ptr(),
                core::format_args!("invalid order function for sorting"),
              );
            }
            continue;
          }
          break;
        }

        // repeat --j until a[j] <= P
        loop {
          j -= 1;
          if sort_less(l, t, p, j, pred) != 0 {
            if j <= lo {
              lua_l_error_l(
                l,
                c"invalid order function for sorting".as_ptr(),
                core::format_args!("invalid order function for sorting"),
              );
            }
            continue;
          }
          break;
        }

        if j < i {
          break;
        }

        sort_swap(l, t, i, j);
      }

      // swap pivot a[p] with a[i], which is the new midpoint
      sort_swap(l, t, p, i);

      // adjust limit to allow 1.5 log2N recursive steps
      limit = (limit >> 1) + (limit >> 2);

      // a[lo..i-1] <= a[i] == P <= a[i+1..u]
      // sort smaller half recursively; the larger half is sorted in the next loop iteration
      if i - lo < u - i {
        sort_rec(l, t, lo, i - 1, limit, pred);
        lo = i + 1;
      } else {
        sort_rec(l, t, i + 1, u, limit, pred);
        u = i - 1;
      }
    }
  }
}
