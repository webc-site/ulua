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

/// cpp 的两个 bool（`unfinishedEnter`/`unfinishedArgs`）构成严格链式状态：
/// 参数只能在未闭合的 Enter 内出现（`ArgName` 处 `LUAU_ASSERT(unfinishedEnter)`
/// 保证），故用三态枚举取代标志对，非法组合 `(args && !enter)` 不可表示。
enum FormatState {
  /// 无未闭合区间。
  Idle,
  /// 未闭合的 `"ph": "B"`。
  Enter,
  /// 未闭合区间且 `"args": {` 已打开。
  Args,
}

/// 闭合已打开的 args 对象与区间条目——cpp `if (unfinishedArgs) ...;`
/// `if (unfinishedEnter) ...;` 的公共前缀与循环后收尾；调用方负责随后
/// 更新 `fmt`。
fn close_open(temp: &mut String, fmt: &FormatState) {
  match fmt {
    FormatState::Args => {
      temp.push('}');
      temp.push_str("},\n");
    }
    FormatState::Enter => temp.push_str("},\n"),
    FormatState::Idle => {}
  }
}

/// cpp `TimeTrace` 镜像工件（`Common/src/TimeTrace.cpp:142-251`），sync-cpp 维护；
/// 仅本 crate 打点机制（`ThreadContext::flush_events`）消费，降 `pub(crate)`。
pub(crate) fn flush_events(context: &GlobalContext, thread_id: u32, events: &[Event], data: &[u8]) {
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

  // 输出格式状态（见 FormatState）
  let mut fmt = FormatState::Idle;

  for ev in events {
    // `Event::data` 的含义由 `type` 决定（cpp union 的 microsec / dataPos 两臂）：
    // Enter/Leave 是微秒时间戳，ArgName/ArgValue 是 `data` 缓冲区的字节偏移。
    match ev.r#type {
      EventType::Enter => {
        close_open(&mut temp, &fmt);

        let token = state.tokens[ev.token as usize];
        format_append(
          &mut temp,
          format_args!(
            r#"{{"name": "{}", "cat": "{}", "ph": "B", "ts": {}, "pid": 0, "tid": {}"#,
            token.name, token.category, ev.data, thread_id
          ),
        );
        fmt = FormatState::Enter;
      }
      EventType::Leave => {
        close_open(&mut temp, &fmt);

        format_append(
          &mut temp,
          format_args!(
            "{{\"ph\": \"E\", \"ts\": {}, \"pid\": 0, \"tid\": {}}},\n",
            ev.data, thread_id
          ),
        );
        fmt = FormatState::Idle;
      }
      EventType::ArgName => {
        let arg = data_str(data, ev.data);
        match fmt {
          FormatState::Args => {
            format_append(&mut temp, format_args!(r#", "{}": "#, arg));
          }
          // Idle 分支在 cpp 里被 `LUAU_ASSERT(unfinishedEnter)` 挡死，
          // release 下与 Enter 同样开 args 头，保行为一致。
          FormatState::Idle | FormatState::Enter => {
            LUAU_ASSERT!(matches!(fmt, FormatState::Enter));
            format_append(&mut temp, format_args!(r#", "args": {{ "{}": "#, arg));
            fmt = FormatState::Args;
          }
        }
      }
      EventType::ArgValue => {
        LUAU_ASSERT!(matches!(fmt, FormatState::Args));

        let value = data_str(data, ev.data);
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

  close_open(&mut temp, &fmt);

  if let Some(file) = state.trace_file.as_mut() {
    let _ = file.write_all(temp.as_bytes());
    let _ = file.flush();
  }
}
