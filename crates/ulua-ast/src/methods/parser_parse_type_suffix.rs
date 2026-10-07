use core::ptr::NonNull;

use ulua_common::{fint::LuauTypeLengthLimit, macros::luau_assert::LUAU_ASSERT};

use crate::{
  enums::type_lexer::Type,
  records::{
    ast_type::AstType, ast_type_intersection::AstTypeIntersection,
    ast_type_optional::AstTypeOptional, ast_type_union::AstTypeUnion,
    cst_type_intersection::CstTypeIntersection, cst_type_union::CstTypeUnion, location::Location,
    node_handle::Node, parse_error::ParseError, parser::Parser, position::Position,
    temp_vector::TempVector,
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
      // 前导类型的 NonNull 即已证非空：升格为 scratch 句柄形态，不落 as_ptr。
      parts.push_back(Node::from_non_null(t));
    }

    self.increment_recursion_counter("type annotation");

    let mut is_union = false;
    let mut is_intersection = false;
    let mut optional_count = 0;

    let mut location = *begin;

    loop {
      let c = self.lexer.current().r#type;
      let separator_position = self.lexer.current().location.begin;

      // `|` 与 `&` 两分支逐字同构（cpp 同为「消费分隔符 → allowPack=false 解析
      // 成员 → 压入 parts → 置向标志 → CST 记分隔符」），仅 union/intersection
      // 标志位不同，以 `is_pipe` 落位；QUESTION/DOT3 语义不同，各走各臂。
      if c == Type::PIPE || c == Type::AMPERSAND {
        let is_pipe = c == Type::PIPE;
        self.next_lexeme();

        let old_recursion_count = self.recursion_counter;
        let part = self.parse_simple_type(false, false);
        self.recursion_counter = old_recursion_count;

        // allowPack=false ⇒ 只可能产出 `Type` 变体（cpp 读 `.type` 恒非空：报错路径
        // 返回的是 AstTypeError 节点而非 null），故取不出类型属解析器 bug，不可触发。
        let ty = part
          .as_type()
          .expect("allowPack=false 时 parse_simple_type 恒产出 Type 变体");
        parts.push_back(Node::from_ref(ty));
        if is_pipe {
          is_union = true;
        } else {
          is_intersection = true;
        }

        // 首个分隔符记 leading（type_ 为 None 即无前导类型的联合/交叉），其余入表。
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

        parts.push_back(Node::from_raw(self.alloc_type(AstTypeOptional::new(loc))));
        optional_count += 1;

        is_union = true;
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
          parts.last().expect("parts 非空").get().base.location,
          format_args!(
            "Exceeded allowed type length; simplify your type annotation to make the code compile"
          ),
        );
      }
    }

    if parts.len() == 1 && !is_union && !is_intersection {
      // 记录面返回形态（parse_type_suffix 的产物直供 AstArray<*mut AstType> 字段）：
      // 句柄在唯一的出参边界折回地址。
      return parts[0].as_ptr();
    }

    if is_union && is_intersection {
      // is_union/is_intersection 均为真 ⇒ 循环至少 push 过一次，parts 非空。
      let error_location = Location::between(
        *begin,
        parts.last().expect("parts 非空").get().base.location,
      );
      let error_types = self.copy_temp_vector_ptrs(&parts);
      return self.report_type_error(
        error_location,
        error_types,
        format_args!(
          "Mixing union and intersection types is not allowed; consider wrapping in parentheses."
        ),
      );
    }

    // 抵达此处说明循环内至少 push 过一个 part（否则前面 len==1 分支已返回）。
    location.end = parts.last().expect("parts 非空").get().base.location.end;
    let parts_array = self.copy_temp_vector_ptrs(&parts);

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
