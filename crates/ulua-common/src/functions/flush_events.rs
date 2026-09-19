//! 来源：`Common/src/TimeTrace.cpp:142-251`
//! 将单个线程缓冲的事件追加到 Chrome-trace 的 `trace.json` 文件中。
use alloc::string::String;
use core::str::from_utf8;
use std::{fs::File, io::Write};

use crate::{
  enums::event_type::EventType,
  functions::format_append::format_append,
  macros::luau_assert::LUAU_ASSERT,
  records::{event::Event, global_context::GlobalContext},
};

/// 缓冲预分配量（C++ `temp.reserve(64 * 1024)`）。
const TEMP_RESERVE: usize = 64 * 1024;
/// 触发提前落盘的水位（预留 1 KiB 余量避免撞容量重新分配）。
const FLUSH_WATERMARK: usize = TEMP_RESERVE - 1024;

/// 读取数据缓冲区中位于字节偏移量 `pos` 处以 NUL 结尾的 UTF-8 字符串
/// (`rawData + ev.data.dataPos`)。
fn data_str(data: &[u8], pos: u32) -> &str {
  let pos = pos as usize;
  if pos >= data.len() {
    return "";
  }
  let slice = &data[pos..];
  let len = memchr::memchr(0, slice).unwrap_or(slice.len());
  from_utf8(&slice[..len]).unwrap_or("")
}

pub fn flush_events(context: &GlobalContext, thread_id: u32, events: &[Event], data: &[u8]) {
  let mut state = context.lock_state();

  if state.trace_file.is_none() {
    match File::create("trace.json") {
      Ok(mut file) => {
        let _ = file.write_all(b"[\n");
        state.trace_file = Some(file);
      }
      Err(_) => return,
    }
  }

  let mut temp = String::new();
  temp.reserve(TEMP_RESERVE);

  // Formatting state
  let mut unfinished_enter = false;
  let mut unfinished_args = false;

  for ev in events {
    match ev.r#type {
      EventType::Enter => {
        if unfinished_args {
          temp.push('}');
          unfinished_args = false;
        }
        if unfinished_enter {
          temp.push_str("},\n");
        }

        let token = state.tokens[ev.token as usize];
        let name = token.name;
        let category = token.category;
        // SAFETY：union 两臂（microsec/data_pos）同为 u32，按写入臂的
        // 事件类型读取（Enter 恒写 microsec）。
        let microsec = unsafe { ev.data.microsec };

        format_append(
          &mut temp,
          format_args!(
            r#"{{"name": "{}", "cat": "{}", "ph": "B", "ts": {}, "pid": 0, "tid": {}"#,
            name, category, microsec, thread_id
          ),
        );
        unfinished_enter = true;
      }
      EventType::Leave => {
        if unfinished_args {
          temp.push('}');
          unfinished_args = false;
        }
        if unfinished_enter {
          temp.push_str("},\n");
          unfinished_enter = false;
        }

        // SAFETY：同上，Leave 恒写 microsec。
        let microsec = unsafe { ev.data.microsec };
        format_append(
          &mut temp,
          format_args!(
            "{{\"ph\": \"E\", \"ts\": {}, \"pid\": 0, \"tid\": {}}},\n",
            microsec, thread_id
          ),
        );
      }
      EventType::ArgName => {
        LUAU_ASSERT!(unfinished_enter);

        // SAFETY：ArgName 恒写 data_pos。
        let pos = unsafe { ev.data.data_pos };
        let arg = data_str(data, pos);
        if !unfinished_args {
          format_append(&mut temp, format_args!(r#", "args": {{ "{}": "#, arg));
          unfinished_args = true;
        } else {
          format_append(&mut temp, format_args!(r#", "{}": "#, arg));
        }
      }
      EventType::ArgValue => {
        LUAU_ASSERT!(unfinished_args);
        // SAFETY：ArgValue 恒写 data_pos。
        let pos = unsafe { ev.data.data_pos };
        let value = data_str(data, pos);
        format_append(&mut temp, format_args!(r#""{}""#, value));
      }
    }

    // Don't want to hit the string capacity and reallocate
    if temp.len() > FLUSH_WATERMARK
      && let Some(file) = state.trace_file.as_mut()
    {
      let _ = file.write_all(temp.as_bytes());
      temp.clear();
    }
  }

  if unfinished_args {
    temp.push('}');
  }
  if unfinished_enter {
    temp.push_str("},\n");
  }

  if let Some(file) = state.trace_file.as_mut() {
    let _ = file.write_all(temp.as_bytes());
    let _ = file.flush();
  }
}
