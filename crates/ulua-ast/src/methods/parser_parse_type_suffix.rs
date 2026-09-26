use core::ptr::NonNull;

use ulua_common::{fint::LuauTypeLengthLimit, macros::luau_assert::LUAU_ASSERT};

use crate::{
  enums::type_lexer::Type,
  functions::optional_node::slot_ref,
  records::{
    ast_type::AstType, ast_type_intersection::AstTypeIntersection,
    ast_type_optional::AstTypeOptional, ast_type_union::AstTypeUnion,
    cst_type_intersection::CstTypeIntersection, cst_type_union::CstTypeUnion, location::Location,
    parse_error::ParseError, parser::Parser, position::Position, temp_vector::TempVector,
  },
};

impl Parser {
  /// `type_` 为已解析的前导类型，`None` 对应 cpp 的 `nullptr`
  /// （以 `|` / `&` 开头的联合/交叉）。
  pub fn parse_type_suffix(
    &mut self,
    type_: Option<NonNull<AstType>>,
    begin: &Location,
  ) -> *mut AstType {
    let mut parts = TempVector::new(&mut self.scratch_type);
    let mut separator_positions = TempVector::new(&mut self.scratch_position);
    let mut leading_position = Position::missing();

    if let Some(t) = type_ {
      parts.push_back(t.as_ptr());
    }

    self.increment_recursion_counter("type annotation");

    let mut is_union = false;
    let mut is_intersection = false;
    let mut optional_count = 0;

    let mut location = *begin;

    loop {
      let c = self.lexer.current().r#type;
      let separator_position = self.lexer.current().location.begin;

      if c == Type::PIPE {
        self.next_lexeme();

        let old_recursion_count = self.recursion_counter;
        let part = self.parse_simple_type(false, false);
        self.recursion_counter = old_recursion_count;

        // allowPack=false ⇒ 只可能产出 `Type` 变体（cpp 读 `.type` 恒非空：报错路径
        // 返回的是 AstTypeError 节点而非 null），故取不出类型属解析器 bug，不可触发。
        let ty = part
          .as_type()
          .expect("allowPack=false 时 parse_simple_type 恒产出 Type 变体");
        parts.push_back(NonNull::from(ty).as_ptr());
        is_union = true;

        if self.options.store_cst_data {
          if type_.is_none() && !leading_position.has_value() {
            leading_position = separator_position;
          } else {
            separator_positions.push_back(separator_position);
          }
        }
      } else if c == Type::QUESTION {
        LUAU_ASSERT!(!parts.is_empty());

        let loc = self.lexer.current().location;
        self.next_lexeme();

        parts.push_back(self.alloc_type(AstTypeOptional::new(loc)));
        optional_count += 1;

        is_union = true;
      } else if c == Type::AMPERSAND {
        self.next_lexeme();

        let old_recursion_count = self.recursion_counter;
        let part = self.parse_simple_type(false, false);
        self.recursion_counter = old_recursion_count;

        // 同上：allowPack=false ⇒ 必为 `Type` 变体。
        let ty = part
          .as_type()
          .expect("allowPack=false 时 parse_simple_type 恒产出 Type 变体");
        parts.push_back(NonNull::from(ty).as_ptr());
        is_intersection = true;

        if self.options.store_cst_data {
          if type_.is_none() && !leading_position.has_value() {
            leading_position = separator_position;
          } else {
            separator_positions.push_back(separator_position);
          }
        }
      } else if c == Type::DOT3 {
        self.report(
          self.lexer.current().location,
          format_args!("Unexpected '...' after type annotation"),
        );
        self.next_lexeme();
      } else {
        break;
      }

      let limit = LuauTypeLengthLimit.get() as u32;
      if parts.len() as u32 > limit + optional_count {
        ParseError::raise(
          slot_ref(*parts.last().expect("parts 非空")).base.location,
          format_args!(
            "Exceeded allowed type length; simplify your type annotation to make the code compile"
          ),
        );
      }
    }

    if parts.len() == 1 && !is_union && !is_intersection {
      return parts[0];
    }

    if is_union && is_intersection {
      // is_union/is_intersection 均为真 ⇒ 循环至少 push 过一次，parts 非空。
      let error_location = Location::between(
        *begin,
        slot_ref(*parts.last().expect("parts 非空")).base.location,
      );
      let error_types = self.copy_temp_vector_t(&parts);
      return self.report_type_error(
        error_location,
        error_types,
        format_args!(
          "Mixing union and intersection types is not allowed; consider wrapping in parentheses."
        ),
      );
    }

    // 抵达此处说明循环内至少 push 过一个 part（否则前面 len==1 分支已返回）。
    location.end = slot_ref(*parts.last().expect("parts 非空"))
      .base
      .location
      .end;
    let parts_array = self.copy_temp_vector_t(&parts);

    if is_union {
      let node = self.alloc_type(AstTypeUnion::new(location, parts_array));
      if self.options.store_cst_data {
        let separators = self.copy_temp_vector_t(&separator_positions);
        self.attach_cst(node, |alloc| {
          alloc.alloc(CstTypeUnion::new(leading_position, separators))
        });
      }
      return node;
    }

    if is_intersection {
      let node = self.alloc_type(AstTypeIntersection::new(location, parts_array));
      if self.options.store_cst_data {
        let separators = self.copy_temp_vector_t(&separator_positions);
        self.attach_cst(node, |alloc| {
          alloc.alloc(CstTypeIntersection::new(leading_position, separators))
        });
      }
      return node;
    }

    LUAU_ASSERT!(false);
    ParseError::raise(
      *begin,
      format_args!("Composite type was not an intersection or union."),
    )
  }
}
