//! Node: `cxx:Method:Luau.Analysis:Analysis/src/ToString.cpp:887:type_stringifier_operator_call`
//! Source: `Analysis/src/ToString.cpp:887-997` (hand-ported)

use alloc::vec::Vec;
use core::{ffi::c_void, mem::take};

use ulua_common::{FFlag, macros::luau_assert::LUAU_ASSERT};

use crate::{
  functions::{follow_type::follow, get_type_alt_j::get, is_nil::is_nil},
  records::{
    element_result::ElementResult, function_type::FunctionType,
    intersection_type::IntersectionType, to_string_span::ToStringSpan, type_iterator::TypeIterator,
    type_stringifier::TypeStringifier, union_type::UnionType,
  },
  type_aliases::type_id::TypeId,
};
impl TypeStringifier {
  /// C++ `void operator()(TypeId, const UnionType& uv)`.
  pub fn operator_call_20(&mut self, _ty: TypeId, uv: &UnionType) {
    unsafe {
      if (*self.state).has_seen(uv as *const UnionType as *const c_void) {
        (*(*self.state).result).cycle = true;
        (*self.state).emit("*CYCLE*");
        return;
      }

      LUAU_ASSERT!(uv.options.len() > 1);

      let mut optional = false;
      let mut has_non_nil_disjunct = false;

      let mut results: Vec<ElementResult> = Vec::new();
      let mut results_length: usize = 0;
      let mut length_limit_hit = false;

      // for (auto el : &uv) — UnionTypeIterator flattens nested unions.
      let mut it = TypeIterator::<UnionType>::type_iterator_type(uv as *const UnionType);
      let end_it = TypeIterator::<UnionType>::type_iterator_default();
      while it.operator_ne(&end_it) {
        let el = follow(it.operator_deref());
        it.operator_inc();

        if (*(*self.state).opts).use_question_marks && is_nil(el) {
          optional = true;
          continue;
        } else {
          has_non_nil_disjunct = true;
        }

        let saved = take(&mut (*(*self.state).result).name);
        let saved_spans_size = (*(*self.state).result).type_spans.len();

        let need_parens = !(*self.state).cycle_names.contains(&el)
          && (!get::<IntersectionType>(el).is_none() || !get::<FunctionType>(el).is_none());

        if need_parens {
          (*self.state).emit("(");
        }

        self.stringify_type_id(el);

        if need_parens {
          (*self.state).emit(")");
        }

        let mut elem = ElementResult {
          str: take(&mut (*(*self.state).result).name),
          ..Default::default()
        };

        for i in saved_spans_size..(*(*self.state).result).type_spans.len() {
          elem.spans.push((&(*(*self.state).result).type_spans)[i]);
        }
        (*(*self.state).result)
          .type_spans
          .truncate(saved_spans_size);

        results_length += elem.str.len();
        results.push(elem);

        (*(*self.state).result).name = saved;

        let max_type_length = (*(*self.state).opts).max_type_length;
        length_limit_hit = max_type_length > 0 && results_length > max_type_length;

        if length_limit_hit {
          break;
        }
      }

      (*self.state).unsee(uv as *const UnionType as *const c_void);

      if !length_limit_hit && !FFlag::DebugLuauToStringNoLexicalSort.get() {
        results.sort_unstable_by(|a, b| a.str.cmp(&b.str));
      }

      if optional && results.len() > 1 {
        (*self.state).emit("(");
      }

      let mut first = true;
      let should_place_on_newlines =
        results.len() > (*(*self.state).opts).composite_types_single_line_limit;
      for elem in results.iter() {
        if !first {
          if should_place_on_newlines {
            (*self.state).newline();
          } else {
            (*self.state).emit(" ");
          }
          (*self.state).emit("| ");
        }

        let base_pos = (&*(*self.state).result).name.len();
        (*self.state).emit(elem.str.as_str());
        for span in elem.spans.iter() {
          (&mut *(*self.state).result).type_spans.push(ToStringSpan {
            start_pos: base_pos + span.start_pos,
            end_pos: base_pos + span.end_pos,
            r#type: span.r#type,
          });
        }

        first = false;
      }

      if optional {
        let mut s = "?";
        if results.len() > 1 {
          s = ")?";
        }

        if !has_non_nil_disjunct {
          s = "nil";
        }

        (*self.state).emit(s);
      }
    }
  }
}
