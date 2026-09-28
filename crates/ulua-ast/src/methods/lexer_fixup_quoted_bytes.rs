use alloc::vec::Vec;

use memchr::memchr;
use ulua_common::{LUAU_ASSERT, functions::is_c_space::is_c_space};

use crate::{
  functions::{char_classifier::is_digit, to_utf_8::to_utf_8, unescape::unescape},
  records::lexer::Lexer,
};

/// `\u{...}` 花括号内允许的最大十六进制位数（对应 C++ 的同名上限）。
const MAX_BRACED_UNICODE_DIGITS: usize = 16;
/// 十进制转义上限：C++ `UCHAR_MAX`。
const UCHAR_MAX: u32 = u8::MAX as u32;

/// 十六进制字符 → 数值；非法字符返回 `None`。合并 C++ 的 `isHexDigit` 判定
/// 与 `(ch | ' ') - 'a' + 10` 展开为单次查找。
#[inline]
const fn hex_value(ch: u8) -> Option<u32> {
  match ch {
    b'0'..=b'9' => Some((ch - b'0') as u32),
    b'a'..=b'f' | b'A'..=b'F' => Some(((ch | 32) - b'a') as u32 + 10),
    _ => None,
  }
}

impl Lexer {
  pub fn fixup_quoted_bytes(data: &mut Vec<u8>) -> bool {
    let Some(first_escape) = memchr(b'\\', data) else {
      return true;
    };

    let size = data.len();
    let mut write = first_escape;
    let mut i = first_escape;

    while i < size {
      // 快路径：memchr 定位下一个反斜杠，其前的普通字节整段搬移（逐字节复制的 memcpy 化）
      match memchr(b'\\', &data[i..]) {
        None => {
          data.copy_within(i.., write);
          write += size - i;
          break;
        }
        Some(offset) => {
          data.copy_within(i..i + offset, write);
          write += offset;
          i += offset;
        }
      }

      // data[i] == b'\\'：转义序列至少还需一个字符
      if i + 1 == size {
        return false;
      }

      let escape = data[i + 1] as char;
      i += 2; // skip \e

      match escape {
        '\n' => {
          data[write] = b'\n';
          write += 1;
        }

        '\r' => {
          data[write] = b'\n';
          write += 1;
          if i < size && data[i] == b'\n' {
            i += 1;
          }
        }

        '\0' => {
          return false;
        }

        'x' => {
          // hex escape codes are exactly 2 hex digits long
          if i + 2 > size {
            return false;
          }

          let code = data[i..i + 2].iter().try_fold(0u32, |code, &ch| {
            hex_value(ch).map(|digit| 16 * code + digit)
          });
          let Some(code) = code else {
            return false;
          };

          data[write] = code as u8;
          write += 1;
          i += 2;
        }

        'z' => {
          // 跳过全部空白：position 定位首个非空白，全空白时消费至末尾
          i += data[i..]
            .iter()
            .position(|&b| !is_c_space(b))
            .unwrap_or(size - i);
        }

        'u' => {
          // unicode escape codes are at least 3 characters including braces
          if i + 3 > size {
            return false;
          }

          if data[i] != b'{' {
            return false;
          }
          i += 1;

          if data[i] == b'}' {
            return false;
          }

          let mut code: u32 = 0;
          let mut digits = 0;
          loop {
            let Some(&ch) = data.get(i) else {
              return false;
            };
            if ch == b'}' {
              break;
            }
            if digits == MAX_BRACED_UNICODE_DIGITS {
              return false;
            }
            let Some(digit) = hex_value(ch) else {
              return false;
            };
            // C++ `code` 是 `unsigned int`，溢出回绕
            code = code.wrapping_mul(16).wrapping_add(digit);
            digits += 1;
            i += 1;
          }

          // 与 C++ 一致：循环耗尽后仍需 data[i] == '}'，恰好 16 位数字 + '}' 合法
          if data.get(i) != Some(&b'}') {
            return false;
          }
          i += 1;

          let utf8_len = to_utf_8(&mut data[write..], code);
          if utf8_len == 0 {
            return false;
          }

          write += utf8_len;
        }

        _ => {
          if is_digit(escape) {
            let mut code = (escape as u32) - ('0' as u32);

            for _ in 0..2 {
              let Some(&ch) = data.get(i) else {
                break;
              };
              if !is_digit(ch as char) {
                break;
              }
              code = 10 * code + ((ch as u32) - ('0' as u32));
              i += 1;
            }

            if code > UCHAR_MAX {
              return false;
            }

            data[write] = code as u8;
            write += 1;
          } else {
            data[write] = unescape(escape) as u8;
            write += 1;
          }
        }
      }
    }

    LUAU_ASSERT!(write <= size);
    data.truncate(write);

    true
  }
}
