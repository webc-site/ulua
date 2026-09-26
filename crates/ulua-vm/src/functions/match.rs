use crate::{
  functions::{
    classend::classend, end_capture::end_capture, match_capture::match_capture,
    matchbalance::matchbalance, matchbracketclass::matchbracketclass, max_expand::max_expand,
    min_expand::min_expand, singlematch::singlematch, start_capture::start_capture,
  },
  macros::{
    cap_position::CAP_POSITION, cap_unfinished::CAP_UNFINISHED, l_esc::L_ESC,
    lua_l_error::luaL_error,
  },
  records::match_state::MatchState,
};

/// cpp `lstrlib.cpp match`（含 `init` 尾递归 goto 与 `dflt` 标签）的索引化实现：
/// `s`/`p` 为相对 `ms.src`/`ms.pat` 的偏移游标，返回 `Some(新 s 偏移)`；
/// `None` 即原 `NULL` 失败哨兵。
///
/// # Safety
/// `ms` 必须是 `prepstate` 建立、仍处本次匹配调用中的 `MatchState`；
/// `s <= ms.src.len()`、`p <= ms.pat.len()`，捕获槽按 CAPTURES_SIZE 界内提供。
pub(crate) unsafe fn match_item(ms: &mut MatchState, s: usize, mut p: usize) -> Option<usize> {
  // Safety: 契约保证偏移界内，单步回溯与递归仅在源串/pattern 界内进行
  unsafe {
    if ms.matchdepth == 0 {
      luaL_error!(ms.l, "pattern too complex");
    }
    ms.matchdepth -= 1;

    let l = ms.l;
    if let Some(interrupt) = (*(*l).global).cb.interrupt {
      (*l).n_ccalls = (*l).n_ccalls.wrapping_add(1);
      interrupt(l, -1);
      (*l).n_ccalls = (*l).n_ccalls.wrapping_sub(1);
    }

    // cpp 中 s 为 NULL 后不再进入 init 分派（各失败分支直接返回），
    // 故循环不变式：进入迭代顶时 s 恒为 Some
    let mut s: Option<usize> = Some(s);
    'init: loop {
      let Some(cur) = s else { break 'init };
      // cpp: `if (p != ms->p_end)` —— 偏移 == pat.len() 即原 p == p_end 哨兵，模式结束即匹配成功
      if p == ms.pat.len() {
        break 'init;
      }
      match ms.pat_byte(p) {
        b'(' => {
          // cpp `case '('`: 位置捕获 `()` 记 CAP_POSITION，否则 CAP_UNFINISHED
          s = if ms.pat_byte(p + 1) == b')' {
            start_capture(ms, cur, p + 2, CAP_POSITION)
          } else {
            start_capture(ms, cur, p + 1, CAP_UNFINISHED)
          };
          break 'init;
        }
        b')' => {
          s = end_capture(ms, cur, p + 1);
          break 'init;
        }
        b'$' => {
          // cpp `case '$'`: 非模式末字符则 goto dflt（落下方 dflt 块），否则要求 s 达源串尾
          if p + 1 == ms.pat.len() {
            if cur != ms.src.len() {
              s = None;
            }
            break 'init;
          }
        }
        x if x == L_ESC as u8 => match ms.pat_byte(p + 1) {
          b'b' => {
            s = matchbalance(ms, cur, p + 2);
            if s.is_none() {
              break 'init; // cpp: else fail (s == NULL)
            }
            p += 4;
            continue 'init; // cpp: return match(ms, s, p + 4)
          }
          b'f' => {
            // cpp `case 'f'` frontier（lstrlib.cpp match）：p += 2 后要求 '['
            p += 2;
            if ms.pat_byte(p) != b'[' {
              luaL_error!(ms.l, "missing '[' after '%f' in pattern");
            }
            let ep = classend(ms, p);
            let class = ms.pat_slice(p, ep - p); // 覆盖 p..=']'，转义分支会读 ec 字节
            // cpp: previous = (s == ms->src_init) ? '\0' : *(s - 1)
            // —— src_init 即偏移 0；越界角点消解为显式边界判定，禁读偏移 -1。
            // cur == src.len() 时 *s 读到终止 NUL（cpp 在 src_end 处读终止符同点位）
            let previous = if cur == 0 { 0u8 } else { ms.src_byte(cur - 1) };
            if matchbracketclass(previous as i32, class) == 0
              && matchbracketclass(ms.src_byte(cur) as i32, class) != 0
            {
              p = ep;
              continue 'init; // cpp: return match(ms, s, ep)
            }
            s = None; // match failed
            break 'init;
          }
          b'0'..=b'9' => {
            // cpp `case '0'..'9'`: uchar(*(p+1)) 即模式数字字节本身
            s = match_capture(ms, cur, ms.pat_byte(p + 1) as i32);
            if s.is_none() {
              break 'init;
            }
            p += 2;
            continue 'init; // cpp: return match(ms, s, p + 2)
          }
          // cpp L_ESC `default: goto dflt`
          _ => {}
        },
        _ => {}
      }

      // cpp `dflt:` 模式类 + 可选量词后缀（此处 s 恒为 Some(cur)，见循环不变式）
      let ep = classend(ms, p); // points to optional suffix
      if !singlematch(ms, cur, p, ep) {
        if matches!(ms.pat_byte(ep), b'*' | b'?' | b'-') {
          p = ep + 1;
          continue 'init; // cpp: return match(ms, s, ep + 1)
        }
        s = None; // '+' or no suffix: fail
      } else {
        match ms.pat_byte(ep) {
          b'?' => match match_item(ms, cur + 1, ep + 1) {
            Some(res) => s = Some(res),
            None => {
              p = ep + 1;
              continue 'init; // else return match(ms, s, ep + 1)
            }
          },
          // cpp `case '+'`: s++ 后 fallthrough 到 '*'（1 次重复已消费）
          b'+' => s = max_expand(ms, cur + 1, p, ep),
          b'*' => s = max_expand(ms, cur, p, ep),
          b'-' => s = min_expand(ms, cur, p, ep),
          _ => {
            s = Some(cur + 1);
            p = ep;
            continue 'init;
          }
        }
      }
      break 'init;
    }

    ms.matchdepth += 1;
    s
  }
}
