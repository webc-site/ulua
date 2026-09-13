//! 来源：`Common/src/TimeTrace.cpp:142-251`
//! 将单个线程缓冲的事件追加到 Chrome-trace 的 `trace.json` 文件中。
use alloc::string::String;
use core::str::from_utf8;
use std::{fs::File, io::Write};

use crate::{
  enums::event_type::EventType,
  functions::format_append::formatAppend,
  macros::luau_assert::LUAU_ASSERT,
  records::{event::Event, global_context::GlobalContext},
};

/// 读取数据缓冲区中位于字节偏移量 `pos` 处以 NUL 结尾的 UTF-8 字符串
/// (`rawData + ev.data.dataPos`)。
fn data_str(data: &[u8], pos: u32) -> &str {
  let pos = pos as usize;
  if pos >= data.len() {
    return "";
  }
  let slice = &data[pos..];
  let len = slice.iter().position(|&b| b == 0).unwrap_or(slice.len());
  from_utf8(&slice[..len]).unwrap_or("")
}

pub fn flush_events(context: &GlobalContext, thread_id: u32, events: &[Event], data: &[u8]) {
  let mut state = context
    .state
    .lock()
    .expect("TimeTrace GlobalContext mutex poisoned");

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
  const TEMP_RESERVE: usize = 64 * 1024;
  temp.reserve(TEMP_RESERVE);

  // Formatting state
  let mut unfinished_enter = false;
  let mut unfinished_args = false;

  for ev in events {
    match ev.r#type {
      EventType::Enter => {
        if unfinished_args {
          formatAppend(&mut temp, format_args!("}}"));
          unfinished_args = false;
        }
        if unfinished_enter {
          formatAppend(&mut temp, format_args!("}},\n"));
        }

        let token = state.tokens[ev.token as usize];
        let name = token.name;
        let category = token.category;
        let microsec = unsafe { ev.data.microsec };

        formatAppend(
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
          formatAppend(&mut temp, format_args!("}}"));
          unfinished_args = false;
        }
        if unfinished_enter {
          formatAppend(&mut temp, format_args!("}},\n"));
          unfinished_enter = false;
        }

        let microsec = unsafe { ev.data.microsec };
        formatAppend(
          &mut temp,
          format_args!(
            "{{\"ph\": \"E\", \"ts\": {}, \"pid\": 0, \"tid\": {}}},\n",
            microsec, thread_id
          ),
        );
      }
      EventType::ArgName => {
        LUAU_ASSERT!(unfinished_enter);

        let pos = unsafe { ev.data.data_pos };
        let arg = data_str(data, pos);
        if !unfinished_args {
          formatAppend(&mut temp, format_args!(r#", "args": {{ "{}": "#, arg));
          unfinished_args = true;
        } else {
          formatAppend(&mut temp, format_args!(r#", "{}": "#, arg));
        }
      }
      EventType::ArgValue => {
        LUAU_ASSERT!(unfinished_args);
        let pos = unsafe { ev.data.data_pos };
        let value = data_str(data, pos);
        formatAppend(&mut temp, format_args!(r#""{}""#, value));
      }
    }

    // Don't want to hit the string capacity and reallocate
    if temp.len() > TEMP_RESERVE - 1024 {
      if let Some(file) = state.trace_file.as_mut() {
        let _ = file.write_all(temp.as_bytes());
      }
      temp.clear();
    }
  }

  if unfinished_args {
    formatAppend(&mut temp, format_args!("}}"));
  }
  if unfinished_enter {
    formatAppend(&mut temp, format_args!("}},\n"));
  }

  if let Some(file) = state.trace_file.as_mut() {
    let _ = file.write_all(temp.as_bytes());
    let _ = file.flush();
  }
}
