extern crate alloc;

// Source: `tests/JsonEmitter.test.cpp`
#[test]
fn json_emitter_afford_extensibility() {
  use ulua_analysis::{
    functions::write_json_emitter::write_json_emitter_vector_t, records::json_emitter::JsonEmitter,
  };
  use ulua_unit_test::records::special::Special;

  let vec = vec![Special { foo: 1, bar: 2 }, Special { foo: 3, bar: 4 }];
  let mut e = JsonEmitter::default();
  write_json_emitter_vector_t(&mut e, &vec);

  let result = e.str();
  assert_eq!("[{\"foo\":1,\"bar\":2},{\"foo\":3,\"bar\":4}]", result);
}

// Source: `tests/JsonEmitter.test.cpp`
#[test]
fn json_emitter_cannot_write_pair_after_finished() {
  use ulua_analysis::records::json_emitter::JsonEmitter;

  let mut emitter = JsonEmitter::default();
  let mut o = emitter.write_object();
  o.finish();
  o.write_pair("a", "b");
  drop(o);

  assert_eq!("{}", emitter.str());
}

// Source: `tests/JsonEmitter.test.cpp`
#[test]
fn json_emitter_cannot_write_value_after_finished() {
  use ulua_analysis::records::json_emitter::JsonEmitter;

  let mut emitter = JsonEmitter::default();
  let mut a = emitter.write_array();
  a.finish();
  a.write_value(1);
  drop(a);

  assert_eq!("[]", emitter.str());
}

// Source: `tests/JsonEmitter.test.cpp`
#[test]
fn json_emitter_finish_when_destructing_array() {
  use ulua_analysis::records::json_emitter::JsonEmitter;

  let mut emitter = JsonEmitter::default();
  emitter.write_array();

  assert_eq!("[]", emitter.str());
}

// Source: `tests/JsonEmitter.test.cpp`
#[test]
fn json_emitter_finish_when_destructing_object() {
  use ulua_analysis::records::json_emitter::JsonEmitter;

  let mut emitter = JsonEmitter::default();
  emitter.write_object();

  assert_eq!("{}", emitter.str());
}

// Source: `tests/JsonEmitter.test.cpp`
#[test]
fn json_emitter_prevent_multiple_array_finish() {
  use ulua_analysis::records::json_emitter::JsonEmitter;

  let mut emitter = JsonEmitter::default();
  let mut a = emitter.write_array();
  a.write_value(1);
  a.finish();
  a.finish();
  drop(a);

  assert_eq!("[1]", emitter.str());
}

// Source: `tests/JsonEmitter.test.cpp`
#[test]
fn json_emitter_prevent_multiple_object_finish() {
  use ulua_analysis::records::json_emitter::JsonEmitter;

  let mut emitter = JsonEmitter::default();
  let mut o = emitter.write_object();
  o.write_pair("a", "b");
  o.finish();
  o.finish();
  drop(o);

  assert_eq!("{\"a\":\"b\"}", emitter.str());
}

// Source: `tests/JsonEmitter.test.cpp`
#[test]
fn json_emitter_push_and_pop_comma() {
  use ulua_analysis::{
    functions::write_json_emitter::write_json_emitter_bool, records::json_emitter::JsonEmitter,
  };

  let mut emitter = JsonEmitter::default();
  emitter.write_comma();
  write_json_emitter_bool(&mut emitter, true);
  emitter.write_comma();
  emitter.write_raw_byte(b'[');
  let comma = emitter.push_comma();
  emitter.write_comma();
  write_json_emitter_bool(&mut emitter, true);
  emitter.write_comma();
  write_json_emitter_bool(&mut emitter, false);
  emitter.write_raw_byte(b']');
  emitter.pop_comma(comma);
  emitter.write_comma();
  write_json_emitter_bool(&mut emitter, false);

  assert_eq!("true,[true,false],false", emitter.str());
}

// Source: `tests/JsonEmitter.test.cpp`
#[test]
fn json_emitter_write_array() {
  use ulua_analysis::records::json_emitter::JsonEmitter;

  let mut emitter = JsonEmitter::default();
  let mut a = emitter.write_array();
  a.write_value(123);
  a.write_value("foo");
  a.finish();
  drop(a);

  let result = emitter.str();
  assert_eq!("[123,\"foo\"]", result);
}

// Source: `tests/JsonEmitter.test.cpp`
#[test]
fn json_emitter_write_bool() {
  use ulua_analysis::{
    functions::write_json_emitter::write_json_emitter_bool, records::json_emitter::JsonEmitter,
  };

  let mut emitter = JsonEmitter::default();
  write_json_emitter_bool(&mut emitter, false);
  assert_eq!("false", emitter.str());

  emitter = JsonEmitter::default();
  write_json_emitter_bool(&mut emitter, true);
  assert_eq!("true", emitter.str());
}

// Source: `tests/JsonEmitter.test.cpp`
#[test]
fn json_emitter_write_comma() {
  use ulua_analysis::{
    functions::write_json_emitter::write_json_emitter_bool, records::json_emitter::JsonEmitter,
  };

  let mut emitter = JsonEmitter::default();
  emitter.write_comma();
  write_json_emitter_bool(&mut emitter, true);
  emitter.write_comma();
  write_json_emitter_bool(&mut emitter, false);
  assert_eq!("true,false", emitter.str());
}

// Source: `tests/JsonEmitter.test.cpp`
#[test]
fn json_emitter_write_null() {
  use core::ptr::null;

  use ulua_analysis::{
    functions::write_json_emitter::write_json_emitter_nullptr_t, records::json_emitter::JsonEmitter,
  };

  let mut emitter = JsonEmitter::default();
  write_json_emitter_nullptr_t(&mut emitter, null());
  assert_eq!("null", emitter.str());
}

// Source: `tests/JsonEmitter.test.cpp`
#[test]
fn json_emitter_write_object() {
  use ulua_analysis::records::json_emitter::JsonEmitter;

  let mut emitter = JsonEmitter::default();
  let mut o = emitter.write_object();
  o.write_pair("foo", "bar");
  o.write_pair("bar", "baz");
  o.finish();
  drop(o);

  let result = emitter.str();
  assert_eq!("{\"foo\":\"bar\",\"bar\":\"baz\"}", result);
}

// Source: `tests/JsonEmitter.test.cpp`
#[test]
fn json_emitter_write_optional() {
  use ulua_analysis::{functions::write_json_emitter::write, records::json_emitter::JsonEmitter};

  let mut emitter = JsonEmitter::default();
  emitter.write_comma();
  write(&mut emitter, &Some(true));
  emitter.write_comma();
  write::<bool>(&mut emitter, &None);

  assert_eq!("true,null", emitter.str());
}

// Source: `tests/JsonEmitter.test.cpp`
#[test]
fn json_emitter_write_string() {
  use ulua_analysis::{
    functions::write_json_emitter::write_json_emitter_string_view,
    records::json_emitter::JsonEmitter,
  };

  let mut emitter = JsonEmitter::default();
  write_json_emitter_string_view(
    &mut emitter,
    r#"foo,bar,baz,
"this should be escaped""#,
  );
  assert_eq!(
    "\"foo,bar,baz,\\n\\\"this should be escaped\\\"\"",
    emitter.str()
  );
}

// Source: `tests/JsonEmitter.test.cpp`
#[test]
fn json_emitter_write_vector() {
  use ulua_analysis::{
    functions::write_json_emitter::write_json_emitter_vector_t, records::json_emitter::JsonEmitter,
  };

  let values = vec![1, 2, 3, 4];
  let mut emitter = JsonEmitter::default();
  write_json_emitter_vector_t(&mut emitter, &values);
  assert_eq!("[1,2,3,4]", emitter.str());
}

// Source: `tests/JsonEmitter.test.cpp`
#[test]
fn json_emitter_write_string_escapes() {
  use ulua_analysis::{
    functions::write_json_emitter::write_json_emitter_string_view,
    records::json_emitter::JsonEmitter,
  };

  // 标准转义：\b \f \n \r \t
  let mut shorthand = JsonEmitter::default();
  write_json_emitter_string_view(&mut shorthand, "x\u{8}\u{c}\n\r\ty");
  assert_eq!("\"x\\b\\f\\n\\r\\ty\"", shorthand.str());

  // 控制字符：转成 \uXXXX
  let mut control = JsonEmitter::default();
  write_json_emitter_string_view(&mut control, "\u{1}\u{1f}");
  assert_eq!("\"\\u0001\\u001f\"", control.str());

  // 合法 UTF-8 字节原样透传
  let mut utf8 = JsonEmitter::default();
  write_json_emitter_string_view(&mut utf8, "e\u{e9}\u{1f600}");
  assert_eq!("\"e\u{e9}\u{1f600}\"", utf8.str());
}
