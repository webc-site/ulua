use core::{
  ffi::{c_char, c_int},
  ptr::null,
};

use crate::{
  functions::{
    classend::classend, end_capture::end_capture, match_capture::match_capture,
    matchbalance::matchbalance, matchbracketclass::matchbracketclass, max_expand::max_expand,
    min_expand::min_expand, singlematch::singlematch, start_capture::start_capture,
  },
  macros::{
    cap_position::CAP_POSITION, cap_unfinished::CAP_UNFINISHED, l_esc::L_ESC,
    lua_l_error::luaL_error, uchar::uchar,
  },
  records::match_state::MatchState,
};

pub(crate) unsafe fn match_item(
  ms: *mut MatchState,
  mut s: *const c_char,
  mut p: *const c_char,
) -> *const c_char {
  unsafe {
    if (*ms).matchdepth == 0 {
      luaL_error!((*ms).l, "pattern too complex");
    }
    (*ms).matchdepth -= 1;

    let l = (*ms).l;
    if let Some(interrupt) = (*(*l).global).cb.interrupt {
      (*l).n_ccalls = (*l).n_ccalls.wrapping_add(1);
      interrupt(l, -1);
      (*l).n_ccalls = (*l).n_ccalls.wrapping_sub(1);
    }

    'init: loop {
      if p != (*ms).p_end {
        match *p as u8 {
          b'(' => {
            if *p.add(1) == b')' as c_char {
              s = start_capture(ms, s, p.add(2), CAP_POSITION);
            } else {
              s = start_capture(ms, s, p.add(1), CAP_UNFINISHED);
            }
            break 'init; // C++ outer-switch `break`: do not fall into dflt
          }
          b')' => {
            s = end_capture(ms, s, p.add(1));
            break 'init; // C++ outer-switch `break`: do not fall into dflt
          }
          b'$' => {
            if p.add(1) != (*ms).p_end {
              // default case below
            } else {
              s = if s == (*ms).src_end { s } else { null() };
              break 'init;
            }
          }
          x if x == L_ESC as u8 => match *p.add(1) as u8 {
            b'b' => {
              s = matchbalance(ms, s, p.add(2));
              if !s.is_null() {
                p = p.add(4);
                continue 'init;
              }
              break 'init;
            }
            b'f' => {
              p = p.add(2);
              if *p != b'[' as c_char {
                luaL_error!((*ms).l, "missing '[' after '%%f' in pattern");
              }
              let ep = classend(ms, p);
              let previous = if s == (*ms).src_init {
                0
              } else {
                *s.offset(-1)
              };
              if matchbracketclass(uchar(previous as c_int) as c_int, p, ep.offset(-1)) == 0
                && matchbracketclass(uchar(*s as c_int) as c_int, p, ep.offset(-1)) != 0
              {
                p = ep;
                continue 'init;
              }
              s = null();
              break 'init;
            }
            b'0'..=b'9' => {
              s = match_capture(ms, s, uchar(*p.add(1) as c_int) as c_int);
              if !s.is_null() {
                p = p.add(2);
                continue 'init;
              }
              break 'init;
            }
            _ => {}
          },
          _ => {}
        }

        if !s.is_null() {
          let ep = classend(ms, p);
          if singlematch(ms, s, p, ep) == 0 {
            if *ep == b'*' as c_char || *ep == b'?' as c_char || *ep == b'-' as c_char {
              p = ep.add(1);
              continue 'init;
            }
            s = null();
          } else {
            match *ep as u8 {
              b'?' => {
                let res = match_item(ms, s.add(1), ep.add(1));
                if !res.is_null() {
                  s = res;
                } else {
                  p = ep.add(1);
                  continue 'init;
                }
              }
              b'+' => {
                s = s.add(1);
                s = max_expand(ms, s, p, ep);
              }
              b'*' => {
                s = max_expand(ms, s, p, ep);
              }
              b'-' => {
                s = min_expand(ms, s, p, ep);
              }
              _ => {
                s = s.add(1);
                p = ep;
                continue 'init;
              }
            }
          }
        }
      }

      break 'init;
    }

    (*ms).matchdepth += 1;
    s
  }
}
