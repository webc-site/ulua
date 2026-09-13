extern crate alloc;

mod json_emitter_afford_extensibility {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/JsonEmitter.test.cpp:185:json_emitter_afford_extensibility`
  //! Source: `tests/JsonEmitter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/JsonEmitter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/JsonEmitter.h
  //! - incoming:
  //!   - declares <- source_file tests/JsonEmitter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record Special (tests/JsonEmitter.test.cpp)
  //!   - calls -> type_alias vec (Common/include/Luau/InsertionOrderedMap.h)
  //!   - type_ref -> record JsonEmitter (Analysis/include/Luau/JsonEmitter.h)
  //!   - calls -> function write (tests/JsonEmitter.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method SubtypeFixture::str (tests/Subtyping.test.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - translates_to -> rust_item json_emitter_afford_extensibility

  #[cfg(test)]
  #[test]
  fn json_emitter_afford_extensibility() {
    use ulua_analysis::{
      functions::write_json_emitter::write_json_emitter_vector_t,
      records::json_emitter::JsonEmitter,
    };
    use ulua_unit_test::records::special::Special;

    let vec = vec![Special { foo: 1, bar: 2 }, Special { foo: 3, bar: 4 }];
    let mut e = JsonEmitter::default();
    write_json_emitter_vector_t(&mut e, &vec);

    let result = e.str();
    assert_eq!("[{\"foo\":1,\"bar\":2},{\"foo\":3,\"bar\":4}]", result);
  }
}

mod json_emitter_cannot_write_pair_after_finished {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/JsonEmitter.test.cpp:131:json_emitter_cannot_write_pair_after_finished`
  //! Source: `tests/JsonEmitter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/JsonEmitter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/JsonEmitter.h
  //! - incoming:
  //!   - declares <- source_file tests/JsonEmitter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record JsonEmitter (Analysis/include/Luau/JsonEmitter.h)
  //!   - type_ref -> record ObjectEmitter (Analysis/include/Luau/JsonEmitter.h)
  //!   - calls -> method JsonEmitter::writeObject (Analysis/src/JsonEmitter.cpp)
  //!   - calls -> method ObjectEmitter::writePair (Analysis/include/Luau/JsonEmitter.h)
  //!   - calls -> method SubtypeFixture::str (tests/Subtyping.test.cpp)
  //!   - translates_to -> rust_item json_emitter_cannot_write_pair_after_finished

  #[cfg(test)]
  #[test]
  fn json_emitter_cannot_write_pair_after_finished() {
    use ulua_analysis::records::json_emitter::JsonEmitter;

    let mut emitter = JsonEmitter::default();
    let mut o = emitter.write_object();
    o.finish();
    o.write_pair("a", "b");

    assert_eq!("{}", emitter.str());
  }
}

mod json_emitter_cannot_write_value_after_finished {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/JsonEmitter.test.cpp:141:json_emitter_cannot_write_value_after_finished`
  //! Source: `tests/JsonEmitter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/JsonEmitter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/JsonEmitter.h
  //! - incoming:
  //!   - declares <- source_file tests/JsonEmitter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record JsonEmitter (Analysis/include/Luau/JsonEmitter.h)
  //!   - type_ref -> record ArrayEmitter (Analysis/include/Luau/JsonEmitter.h)
  //!   - calls -> method JsonEmitter::writeArray (Analysis/src/JsonEmitter.cpp)
  //!   - calls -> method ArrayEmitter::writeValue (Analysis/include/Luau/JsonEmitter.h)
  //!   - calls -> method SubtypeFixture::str (tests/Subtyping.test.cpp)
  //!   - translates_to -> rust_item json_emitter_cannot_write_value_after_finished

  #[cfg(test)]
  #[test]
  fn json_emitter_cannot_write_value_after_finished() {
    use ulua_analysis::records::json_emitter::JsonEmitter;

    let mut emitter = JsonEmitter::default();
    let mut a = emitter.write_array();
    a.finish();
    a.write_value(1);

    assert_eq!("[]", emitter.str());
  }
}

mod json_emitter_finish_when_destructing_array {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/JsonEmitter.test.cpp:159:json_emitter_finish_when_destructing_array`
  //! Source: `tests/JsonEmitter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/JsonEmitter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/JsonEmitter.h
  //! - incoming:
  //!   - declares <- source_file tests/JsonEmitter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record JsonEmitter (Analysis/include/Luau/JsonEmitter.h)
  //!   - calls -> method JsonEmitter::writeArray (Analysis/src/JsonEmitter.cpp)
  //!   - calls -> method SubtypeFixture::str (tests/Subtyping.test.cpp)
  //!   - translates_to -> rust_item json_emitter_finish_when_destructing_array

  #[cfg(test)]
  #[test]
  fn json_emitter_finish_when_destructing_array() {
    use ulua_analysis::records::json_emitter::JsonEmitter;

    let mut emitter = JsonEmitter::default();
    emitter.write_array();

    assert_eq!("[]", emitter.str());
  }
}

mod json_emitter_finish_when_destructing_object {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/JsonEmitter.test.cpp:151:json_emitter_finish_when_destructing_object`
  //! Source: `tests/JsonEmitter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/JsonEmitter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/JsonEmitter.h
  //! - incoming:
  //!   - declares <- source_file tests/JsonEmitter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record JsonEmitter (Analysis/include/Luau/JsonEmitter.h)
  //!   - calls -> method JsonEmitter::writeObject (Analysis/src/JsonEmitter.cpp)
  //!   - calls -> method SubtypeFixture::str (tests/Subtyping.test.cpp)
  //!   - translates_to -> rust_item json_emitter_finish_when_destructing_object

  #[cfg(test)]
  #[test]
  fn json_emitter_finish_when_destructing_object() {
    use ulua_analysis::records::json_emitter::JsonEmitter;

    let mut emitter = JsonEmitter::default();
    emitter.write_object();

    assert_eq!("{}", emitter.str());
  }
}

mod json_emitter_prevent_multiple_array_finish {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/JsonEmitter.test.cpp:120:json_emitter_prevent_multiple_array_finish`
  //! Source: `tests/JsonEmitter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/JsonEmitter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/JsonEmitter.h
  //! - incoming:
  //!   - declares <- source_file tests/JsonEmitter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record JsonEmitter (Analysis/include/Luau/JsonEmitter.h)
  //!   - type_ref -> record ArrayEmitter (Analysis/include/Luau/JsonEmitter.h)
  //!   - calls -> method JsonEmitter::writeArray (Analysis/src/JsonEmitter.cpp)
  //!   - calls -> method ArrayEmitter::writeValue (Analysis/include/Luau/JsonEmitter.h)
  //!   - calls -> method SubtypeFixture::str (tests/Subtyping.test.cpp)
  //!   - translates_to -> rust_item json_emitter_prevent_multiple_array_finish

  #[cfg(test)]
  #[test]
  fn json_emitter_prevent_multiple_array_finish() {
    use ulua_analysis::records::json_emitter::JsonEmitter;

    let mut emitter = JsonEmitter::default();
    let mut a = emitter.write_array();
    a.write_value(1);
    a.finish();
    a.finish();

    assert_eq!("[1]", emitter.str());
  }
}

mod json_emitter_prevent_multiple_object_finish {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/JsonEmitter.test.cpp:109:json_emitter_prevent_multiple_object_finish`
  //! Source: `tests/JsonEmitter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/JsonEmitter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/JsonEmitter.h
  //! - incoming:
  //!   - declares <- source_file tests/JsonEmitter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record JsonEmitter (Analysis/include/Luau/JsonEmitter.h)
  //!   - type_ref -> record ObjectEmitter (Analysis/include/Luau/JsonEmitter.h)
  //!   - calls -> method JsonEmitter::writeObject (Analysis/src/JsonEmitter.cpp)
  //!   - calls -> method ObjectEmitter::writePair (Analysis/include/Luau/JsonEmitter.h)
  //!   - calls -> method SubtypeFixture::str (tests/Subtyping.test.cpp)
  //!   - translates_to -> rust_item json_emitter_prevent_multiple_object_finish

  #[cfg(test)]
  #[test]
  fn json_emitter_prevent_multiple_object_finish() {
    use ulua_analysis::records::json_emitter::JsonEmitter;

    let mut emitter = JsonEmitter::default();
    let mut o = emitter.write_object();
    o.write_pair("a", "b");
    o.finish();
    o.finish();

    assert_eq!("{\"a\":\"b\"}", emitter.str());
  }
}

mod json_emitter_push_and_pop_comma {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/JsonEmitter.test.cpp:70:json_emitter_push_and_pop_comma`
  //! Source: `tests/JsonEmitter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/JsonEmitter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/JsonEmitter.h
  //! - incoming:
  //!   - declares <- source_file tests/JsonEmitter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record JsonEmitter (Analysis/include/Luau/JsonEmitter.h)
  //!   - calls -> method JsonEmitter::writeComma (Analysis/src/JsonEmitter.cpp)
  //!   - calls -> function write (tests/JsonEmitter.test.cpp)
  //!   - calls -> method SubtypeFixture::str (tests/Subtyping.test.cpp)
  //!   - translates_to -> rust_item json_emitter_push_and_pop_comma

  #[cfg(test)]
  #[test]
  fn json_emitter_push_and_pop_comma() {
    use core::ffi::c_char;

    use ulua_analysis::{
      functions::write_json_emitter_alt_w::write_json_emitter_bool,
      records::json_emitter::JsonEmitter,
    };

    let mut emitter = JsonEmitter::default();
    emitter.write_comma();
    write_json_emitter_bool(&mut emitter, true);
    emitter.write_comma();
    emitter.write_raw_c_char(b'[' as c_char);
    let comma = emitter.push_comma();
    emitter.write_comma();
    write_json_emitter_bool(&mut emitter, true);
    emitter.write_comma();
    write_json_emitter_bool(&mut emitter, false);
    emitter.write_raw_c_char(b']' as c_char);
    emitter.pop_comma(comma);
    emitter.write_comma();
    write_json_emitter_bool(&mut emitter, false);

    assert_eq!("true,[true,false],false", emitter.str());
  }
}

mod json_emitter_write_array {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/JsonEmitter.test.cpp:10:json_emitter_write_array`
  //! Source: `tests/JsonEmitter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/JsonEmitter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/JsonEmitter.h
  //! - incoming:
  //!   - declares <- source_file tests/JsonEmitter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record JsonEmitter (Analysis/include/Luau/JsonEmitter.h)
  //!   - type_ref -> record ArrayEmitter (Analysis/include/Luau/JsonEmitter.h)
  //!   - calls -> method JsonEmitter::writeArray (Analysis/src/JsonEmitter.cpp)
  //!   - calls -> method ArrayEmitter::writeValue (Analysis/include/Luau/JsonEmitter.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method SubtypeFixture::str (tests/Subtyping.test.cpp)
  //!   - translates_to -> rust_item json_emitter_write_array

  #[cfg(test)]
  #[test]
  fn json_emitter_write_array() {
    use ulua_analysis::records::json_emitter::JsonEmitter;

    let mut emitter = JsonEmitter::default();
    let mut a = emitter.write_array();
    a.write_value(123);
    a.write_value("foo");
    a.finish();

    let result = emitter.str();
    assert_eq!("[123,\"foo\"]", result);
  }
}

mod json_emitter_write_bool {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/JsonEmitter.test.cpp:34:json_emitter_write_bool`
  //! Source: `tests/JsonEmitter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/JsonEmitter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/JsonEmitter.h
  //! - incoming:
  //!   - declares <- source_file tests/JsonEmitter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record JsonEmitter (Analysis/include/Luau/JsonEmitter.h)
  //!   - calls -> function write (tests/JsonEmitter.test.cpp)
  //!   - calls -> method SubtypeFixture::str (tests/Subtyping.test.cpp)
  //!   - translates_to -> rust_item json_emitter_write_bool

  #[cfg(test)]
  #[test]
  fn json_emitter_write_bool() {
    use ulua_analysis::{
      functions::write_json_emitter_alt_w::write_json_emitter_bool,
      records::json_emitter::JsonEmitter,
    };

    let mut emitter = JsonEmitter::default();
    write_json_emitter_bool(&mut emitter, false);
    assert_eq!("false", emitter.str());

    emitter = JsonEmitter::default();
    write_json_emitter_bool(&mut emitter, true);
    assert_eq!("true", emitter.str());
  }
}

mod json_emitter_write_comma {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/JsonEmitter.test.cpp:60:json_emitter_write_comma`
  //! Source: `tests/JsonEmitter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/JsonEmitter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/JsonEmitter.h
  //! - incoming:
  //!   - declares <- source_file tests/JsonEmitter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record JsonEmitter (Analysis/include/Luau/JsonEmitter.h)
  //!   - calls -> method JsonEmitter::writeComma (Analysis/src/JsonEmitter.cpp)
  //!   - calls -> function write (tests/JsonEmitter.test.cpp)
  //!   - calls -> method SubtypeFixture::str (tests/Subtyping.test.cpp)
  //!   - translates_to -> rust_item json_emitter_write_comma

  #[cfg(test)]
  #[test]
  fn json_emitter_write_comma() {
    use ulua_analysis::{
      functions::write_json_emitter_alt_w::write_json_emitter_bool,
      records::json_emitter::JsonEmitter,
    };

    let mut emitter = JsonEmitter::default();
    emitter.write_comma();
    write_json_emitter_bool(&mut emitter, true);
    emitter.write_comma();
    write_json_emitter_bool(&mut emitter, false);
    assert_eq!("true,false", emitter.str());
  }
}

mod json_emitter_write_null {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/JsonEmitter.test.cpp:45:json_emitter_write_null`
  //! Source: `tests/JsonEmitter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/JsonEmitter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/JsonEmitter.h
  //! - incoming:
  //!   - declares <- source_file tests/JsonEmitter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record JsonEmitter (Analysis/include/Luau/JsonEmitter.h)
  //!   - calls -> function write (tests/JsonEmitter.test.cpp)
  //!   - calls -> method SubtypeFixture::str (tests/Subtyping.test.cpp)
  //!   - translates_to -> rust_item json_emitter_write_null

  #[cfg(test)]
  #[test]
  fn json_emitter_write_null() {
    use core::ptr::null;

    use ulua_analysis::{
      functions::write_json_emitter_alt_ai::write_json_emitter_nullptr_t,
      records::json_emitter::JsonEmitter,
    };

    let mut emitter = JsonEmitter::default();
    write_json_emitter_nullptr_t(&mut emitter, null());
    assert_eq!("null", emitter.str());
  }
}

mod json_emitter_write_object {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/JsonEmitter.test.cpp:22:json_emitter_write_object`
  //! Source: `tests/JsonEmitter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/JsonEmitter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/JsonEmitter.h
  //! - incoming:
  //!   - declares <- source_file tests/JsonEmitter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record JsonEmitter (Analysis/include/Luau/JsonEmitter.h)
  //!   - type_ref -> record ObjectEmitter (Analysis/include/Luau/JsonEmitter.h)
  //!   - calls -> method JsonEmitter::writeObject (Analysis/src/JsonEmitter.cpp)
  //!   - calls -> method ObjectEmitter::writePair (Analysis/include/Luau/JsonEmitter.h)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
  //!   - calls -> method SubtypeFixture::str (tests/Subtyping.test.cpp)
  //!   - translates_to -> rust_item json_emitter_write_object

  #[cfg(test)]
  #[test]
  fn json_emitter_write_object() {
    use ulua_analysis::records::json_emitter::JsonEmitter;

    let mut emitter = JsonEmitter::default();
    let mut o = emitter.write_object();
    o.write_pair("foo", "bar");
    o.write_pair("bar", "baz");
    o.finish();

    let result = emitter.str();
    assert_eq!("{\"foo\":\"bar\",\"bar\":\"baz\"}", result);
  }
}

mod json_emitter_write_optional {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/JsonEmitter.test.cpp:90:json_emitter_write_optional`
  //! Source: `tests/JsonEmitter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/JsonEmitter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/JsonEmitter.h
  //! - incoming:
  //!   - declares <- source_file tests/JsonEmitter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record JsonEmitter (Analysis/include/Luau/JsonEmitter.h)
  //!   - calls -> method JsonEmitter::writeComma (Analysis/src/JsonEmitter.cpp)
  //!   - calls -> function write (tests/JsonEmitter.test.cpp)
  //!   - calls -> method SubtypeFixture::str (tests/Subtyping.test.cpp)
  //!   - translates_to -> rust_item json_emitter_write_optional

  #[cfg(test)]
  #[test]
  fn json_emitter_write_optional() {
    use ulua_analysis::{
      functions::write_json_emitter_alt_b::write, records::json_emitter::JsonEmitter,
    };

    let mut emitter = JsonEmitter::default();
    emitter.write_comma();
    write(&mut emitter, &Some(true));
    emitter.write_comma();
    write::<bool>(&mut emitter, &None);

    assert_eq!("true,null", emitter.str());
  }
}

mod json_emitter_write_string {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/JsonEmitter.test.cpp:52:json_emitter_write_string`
  //! Source: `tests/JsonEmitter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/JsonEmitter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/JsonEmitter.h
  //! - incoming:
  //!   - declares <- source_file tests/JsonEmitter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record JsonEmitter (Analysis/include/Luau/JsonEmitter.h)
  //!   - calls -> function write (tests/JsonEmitter.test.cpp)
  //!   - calls -> function foo (tests/NotNull.test.cpp)
  //!   - calls -> function bar (tests/NotNull.test.cpp)
  //!   - calls -> method SubtypeFixture::str (tests/Subtyping.test.cpp)
  //!   - translates_to -> rust_item json_emitter_write_string

  #[cfg(test)]
  #[test]
  fn json_emitter_write_string() {
    use ulua_analysis::{
      functions::write_json_emitter_alt_ae::write_json_emitter_string_view,
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
}

mod json_emitter_write_vector {
  //! Generated skeleton item. @skeleton-stub
  //! Node: `cxx:Test:Luau.UnitTest:tests/JsonEmitter.test.cpp:101:json_emitter_write_vector`
  //! Source: `tests/JsonEmitter.test.cpp`
  //! Graph edges:
  //! - declared_by: source_file tests/JsonEmitter.test.cpp
  //! - source_includes:
  //!   - includes -> source_file Analysis/include/Luau/JsonEmitter.h
  //! - incoming:
  //!   - declares <- source_file tests/JsonEmitter.test.cpp
  //! - outgoing:
  //!   - type_ref -> record JsonEmitter (Analysis/include/Luau/JsonEmitter.h)
  //!   - calls -> function write (tests/JsonEmitter.test.cpp)
  //!   - calls -> method SubtypeFixture::str (tests/Subtyping.test.cpp)
  //!   - translates_to -> rust_item json_emitter_write_vector

  #[cfg(test)]
  #[test]
  fn json_emitter_write_vector() {
    use ulua_analysis::{
      functions::write_json_emitter::write_json_emitter_vector_t,
      records::json_emitter::JsonEmitter,
    };

    let values = vec![1, 2, 3, 4];
    let mut emitter = JsonEmitter::default();
    write_json_emitter_vector_t(&mut emitter, &values);
    assert_eq!("[1,2,3,4]", emitter.str());
  }
}
