use core::{ffi::c_char, ptr::copy_nonoverlapping, slice::from_raw_parts};

use memchr::memchr3;

pub(crate) unsafe fn lua_o_chunkid(
  buf: *mut c_char,
  buflen: usize,
  source: *const c_char,
  srclen: usize,
) -> *mut c_char {
  unsafe {
    debug_assert!(!buf.is_null());
    debug_assert!(!source.is_null());

    if *source == b'=' as c_char {
      if srclen <= buflen {
        return source.add(1) as *mut c_char;
      }
      // truncate the part after '='
      if buflen == 0 {
        return buf;
      }

      let n = buflen.saturating_sub(1);
      copy_nonoverlapping(source.add(1) as *const u8, buf as *mut u8, n);
      *buf.add(buflen - 1) = b'\0' as c_char;
    } else if *source == b'@' as c_char {
      if srclen <= buflen {
        return source.add(1) as *mut c_char;
      }
      // truncate the part after '@'
      if buflen == 0 {
        return buf;
      }

      // memcpy(buf, "...", 3);
      copy_nonoverlapping(b"..." as *const u8, buf as *mut u8, 3);

      // memcpy(buf + 3, source + srclen - (buflen - 4), buflen - 4);
      let tail_len = buflen.saturating_sub(4);
      let src_start = source.add(srclen.saturating_sub(tail_len));
      copy_nonoverlapping(src_start as *const u8, buf.add(3) as *mut u8, tail_len);

      *buf.add(buflen - 1) = b'\0' as c_char;
    } else {
      // buf = [string "string"]

      // C++ `strcspn(source, "\n\r")`：在 srclen 范围内找首个 '\n' / '\r' / NUL
      // （TString 布局保证 srclen 处有终止 NUL，故与 strcspn 语义一致）。
      // memchr3 走 SIMD，扫描上限受 srclen 约束不越界。
      let bytes = from_raw_parts(source as *const u8, srclen);
      let len = memchr3(b'\n', b'\r', 0, bytes).unwrap_or(srclen);

      // buflen -= sizeof("[string \"...\"]");
      // cpp 的 sizeof 含 NUL 终止符，即减去 16
      let inner_buflen = buflen.saturating_sub(16);
      let len = len.min(inner_buflen);

      // strcpy(buf, "[string \"");
      let prefix_bytes = b"[string \"";
      copy_nonoverlapping(prefix_bytes.as_ptr(), buf as *mut u8, prefix_bytes.len());

      let mut end = prefix_bytes.len();
      // strncat(buf, source, len);
      if len > 0 {
        copy_nonoverlapping(source as *const u8, buf.add(end) as *mut u8, len);
        end += len;
      }
      if *source.add(len) != 0 {
        // must truncate? strcat(buf, "...");
        copy_nonoverlapping(b"..." as *const u8, buf.add(end) as *mut u8, 3);
        end += 3;
      }
      // strcat(buf, "\"]")
      let suffix = b"\"]";
      copy_nonoverlapping(suffix.as_ptr(), buf.add(end) as *mut u8, suffix.len());
      end += suffix.len();
      *buf.add(end) = b'\0' as c_char;
    }

    buf
  }
}
