use crate::{
  enums::type_lexer::Type,
  functions::{is_enough_values::is_enough_values, optional_node::slot_ref},
  records::{
    ast_array::AstArray, ast_attr::AstAttr, ast_stat::AstStat, ast_stat_local::AstStatLocal,
    ast_stat_local_function::AstStatLocalFunction, cst_stat_local::CstStatLocal,
    cst_stat_local_function::CstStatLocalFunction, location::Location, node_handle::Node,
    parser::Parser, position::Position, temp_vector::TempVector,
  },
};

impl Parser {
  pub fn parse_local(
    &mut self,
    start: Location,
    keyword_position: Position,
    attributes: &AstArray<*mut AstAttr>,
    is_const: bool,
  ) -> *mut AstStat {
    if !is_const {
      self.next_lexeme();
    }

    if self.lexer.current().r#type == Type::RESERVED_FUNCTION {
      let mut match_function = *self.lexer.current();
      self.next_lexeme();

      // cpp Parser.cpp:1334-1338：`local function` 同行时把 matchFunction 的
      // 列号改写为 `local` 的列号，令 end-mismatch 嫌疑启发式对齐缩进列；
      // 必须改写传入 parse_function_body 的本体，而非副本。
      let function_keyword_position = match_function.location.begin;
      if match_function.location.begin.line == start.begin.line {
        match_function.location.begin.column = start.begin.column;
      }

      let name = self.parse_name("variable name");

      self.match_recovery_stop_on_token[Type::RESERVED_END.0 as usize] += 1;

      let (body, var) = self.parse_function_body(
        false,
        &match_function,
        &name.name,
        Some(&name),
        attributes,
        is_const,
      );

      self.match_recovery_stop_on_token[Type::RESERVED_END.0 as usize] -= 1;

      let location = Location::new(start.begin, slot_ref(body).base.base.location.end);

      let node = self.alloc_stat(AstStatLocalFunction::new(
        location,
        // cpp Parser.cpp:1350 直传 `parseFunctionBody` 的 `funLocal`：本调用点
        // `localName = Some(&name)`，pushLocal（Parser.cpp:5147 `alloc<AstLocal>`）
        // 恒非空 → name 槽位恒非空（cpp 处 null 即 UB，不可触发）。
        Node::from_non_null(
          var.expect("local function 的名字局部变量由 pushLocal 恒非空建出（cpp Parser.cpp:2345）"),
        ),
        Node::from_raw(body),
        is_const,
      ));

      self.attach_cst(node, |alloc| {
        alloc.alloc(CstStatLocalFunction::new(
          keyword_position,
          function_keyword_position,
        ))
      });

      node
    } else {
      if !attributes.is_empty() {
        let current = *self.lexer.current();
        return self.report_stat_error(
          current.location,
          AstArray::EMPTY,
          AstArray::EMPTY,
          format_args!(
            "Expected 'function' after local declaration with attribute, but got {current} instead",
          ),
        );
      }

      self.match_recovery_stop_on_token[Type::EQUAL_SIGN.0 as usize] += 1;

      let mut names = TempVector::new(&mut self.scratch_binding);
      let mut vars_comma_positions = AstArray::EMPTY;

      // 两分支仅 CST 位置 out 参数不同：if 表达式归一（C++ 同款传 null 关闭记录）
      self.parse_binding_list(
        &mut names,
        false,
        if self.options.store_cst_data {
          Some(&mut vars_comma_positions)
        } else {
          None
        },
        None,
        None,
        is_const,
      );

      self.match_recovery_stop_on_token[Type::EQUAL_SIGN.0 as usize] -= 1;

      let mut vars = TempVector::new(&mut self.scratch_local);
      let mut values = TempVector::new(&mut self.scratch_expr);
      let mut values_comma_positions = TempVector::new(&mut self.scratch_position);

      let mut equals_sign_location = None;

      if self.lexer.current().r#type == Type::EQUAL_SIGN {
        equals_sign_location = Some(self.lexer.current().location);
        self.next_lexeme();

        self.parse_expr_list(
          &mut values,
          if self.options.store_cst_data {
            Some(&mut values_comma_positions)
          } else {
            None
          },
        );
      }

      for name in names.iter() {
        vars.push_back(self.push_local(name));
      }

      let end = if let Some(&expr) = values.last() {
        // 槽内为 parse_expr_list 收集的非空 arena 存活指针，仅读其基类 location。
        slot_ref(expr).base.location
      } else {
        *self.lexer.previous_location()
      };

      let vars_array = self.copy_temp_vector_t(&vars);
      let values_array = self.copy_temp_vector_t(&values);
      let node = self.alloc_stat(AstStatLocal::new(
        Location::new(start.begin, end.end),
        vars_array,
        values_array,
        equals_sign_location,
        is_const,
      ));

      if self.options.store_cst_data {
        let names_colon_positions = self.extract_annotation_colon_positions(&names);
        let values_comma_array = self.copy_temp_vector_t(&values_comma_positions);
        self.attach_cst(node, |alloc| {
          alloc.alloc(CstStatLocal::new(
            names_colon_positions,
            vars_comma_positions,
            values_comma_array,
          ))
        });
      }

      // const 声明必然值不够时（如 `const foo`、`const bar, baz = 42`）报错，
      // 但声明本身仍合法，按原样返回节点。
      if is_const && !is_enough_values(&values, vars.len()) {
        self.report(
          slot_ref(node).base.location,
          format_args!("Missing initializer in const declaration"),
        );
      }

      node
    }
  }
}
