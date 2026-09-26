use core::ptr::NonNull;

use ulua_common::{
  fflag,
  records::{dense_hash_set::DenseHashSet, variant::Variant2},
};

use crate::{
  enums::type_lexer::Type,
  functions::optional_node::{node_opt, opt_node, slot_opt, slot_ref},
  records::{
    ast_array::AstArray, ast_class_method::AstClassMethod, ast_class_property::AstClassProperty,
    ast_expr::AstExpr, ast_local::AstLocal, ast_name::AstName, ast_stat::AstStat,
    ast_stat_class::AstStatClass, ast_type::AstType, lexeme::lexeme_name_is, location::Location,
    name::Name, parser::Parser, temp_vector::TempVector,
  },
};

const ALLOWED_METAMETHODS: &[&str] = &[
  "__init",
  "__call",
  "__concat",
  "__unm",
  "__add",
  "__sub",
  "__mul",
  "__div",
  "__mod",
  "__pow",
  "__tostring",
  "__eq",
  "__lt",
  "__le",
  "__iter",
  "__len",
  "__idiv",
];

const EXPLICITLY_DISALLOWED_METAMETHODS: &[&str] =
  &["__index", "__newindex", "__mode", "__metatable", "__type"];

impl Parser {
  pub fn parse_class_stat(&mut self, start: &Location, exported: bool, open: bool) -> *mut AstStat {
    ulua_common::LUAU_ASSERT!(fflag::DebugLuauUserDefinedClasses.get());
    let name_opt = self.parse_name_opt("type name");

    let name = name_opt.unwrap_or_else(|| Name {
      name: self.name_error,
      location: self.lexer.current().location,
    });

    // cpp:1540 `allocator.alloc<AstLocal>(...)` 仅分配供 AstStatClass 记录，
    // 不注册进 local_map；类名对 `extends` 引用解析为 AstExprGlobal。
    let function_depth = self.function_stack.len().saturating_sub(1);
    let loop_depth = self
      .function_stack
      .last()
      .map_or(0, |frame| frame.loop_depth as usize);
    let name_local_ptr = self.alloc(AstLocal {
      name: name.name,
      location: name.location,
      shadow: opt_node(None),
      function_depth,
      loop_depth,
      is_const: true,
      is_exported: false,
      annotation: opt_node(None),
    });
    let name_local = slot_ref(name_local_ptr);

    let mut super_: Option<NonNull<AstExpr>> = None;
    if lexeme_name_is(self.lexer.current(), "extends") {
      self.next_lexeme();
      super_ = node_opt(self.parse_class_ref_expr());
    }

    let mut declarations = TempVector::new(&mut self.scratch_class_declarations);

    let mut class_member_namespace: DenseHashSet<AstName> = DenseHashSet::default();

    while self.lexer.current().r#type != Type::RESERVED_END
      && self.lexer.current().r#type != Type::EOF
    {
      let mut qualifier_location: Option<Location> = None;
      if lexeme_name_is(self.lexer.current(), "public") {
        qualifier_location = Some(self.lexer.current().location);
        self.next_lexeme();
      }

      if qualifier_location.is_some() && self.lexer.current().r#type != Type::RESERVED_FUNCTION {
        let Some(prop_name) = self.parse_name_opt("class property name") else {
          continue;
        };

        let mut prop_type: Option<NonNull<AstType>> = None;
        let mut type_colon_location: Option<Location> = None;

        if self.lexer.current().r#type == Type::COLON {
          type_colon_location = Some(self.lexer.current().location);
          self.next_lexeme();
          prop_type = node_opt(self.parse_type(false));
        }

        // cpp:1594-1604 链式校验：new / __ 前缀 / 重复——前两类与重复项均
        // 不入 namespace 与 declarations
        if prop_name.name.as_bytes() == b"new" {
          self.report(
            prop_name.location,
            format_args!(
              "Class properties cannot be named 'new'. Define a method named '__init' to define a constructor."
            ),
          );
        } else if prop_name.name.as_bytes().starts_with(b"__") {
          self.report(
            prop_name.location,
            format_args!("Class properties cannot start with '__'"),
          );
        } else if class_member_namespace.contains(&prop_name.name) {
          self.report(
            prop_name.location,
            format_args!("Duplicate class member '{}'", prop_name.name),
          );
        } else {
          class_member_namespace.insert(prop_name.name);

          ulua_common::LUAU_ASSERT!(prop_type.is_none() == type_colon_location.is_none());
          declarations.push_back(Variant2::V0(AstClassProperty {
            qualifier_location: qualifier_location.unwrap_or_default(),
            name: prop_name.name,
            name_location: prop_name.location,
            type_colon_location,
            ty: opt_node(prop_type),
          }));
        }
      } else if self.lexer.current().r#type == Type::RESERVED_FUNCTION {
        let match_function = *self.lexer.current();
        self.next_lexeme();

        let name = self.parse_name("method name");

        self.match_recovery_stop_on_token[Type::RESERVED_END.0 as usize] += 1;

        let (body, _) = self.parse_function_body(
          false,
          &match_function,
          &name.name,
          None,
          &AstArray::EMPTY,
          false,
        );

        self.match_recovery_stop_on_token[Type::RESERVED_END.0 as usize] -= 1;

        // body 指向 arena 存活节点；annotation 可空由 `slot_opt` 折叠。
        let body_ref = slot_ref(body);
        if let Some(first_arg) = body_ref.args.iter_nodes().next()
          && first_arg.name == "self"
          && let Some(annotation) = slot_opt(first_arg.annotation)
        {
          self.report(
            annotation.base.location,
            format_args!("The 'self' parameter cannot have a type annotation"),
          );
        }

        let name_str = name.name.as_str_or_empty();
        if name_str == "new" {
          // cpp:1647（注意 cpp 文案句点后为两个空格，保留以逐字对齐）
          self.report(
            name.location,
            format_args!(
              "Class methods cannot be named 'new'.  Name it '__init' to define a constructor."
            ),
          );
        } else if name_str.starts_with("__") {
          if EXPLICITLY_DISALLOWED_METAMETHODS.contains(&name_str) {
            self.report(
              name.location,
              format_args!("Classes cannot define '{}' as a metamethod", name_str),
            );
          } else if !ALLOWED_METAMETHODS.contains(&name_str) {
            self.report(
              name.location,
              format_args!(
                "Cannot use '{}' as a method name: names starting with '__' are reserved",
                name_str
              ),
            );
          }
        }

        if class_member_namespace.contains(&name.name) {
          self.report(
            name.location,
            format_args!("Duplicate class member '{}'", name.name),
          );
        } else {
          class_member_namespace.insert(name.name);

          declarations.push_back(Variant2::V1(AstClassMethod {
            qualifier_location,
            keyword_location: match_function.location,
            function_name: name.name,
            name_location: name.location,
            function: body,
          }));
        }
      } else {
        self.report(
          self.lexer.current().location,
          format_args!("Only class properties and functions can be declared within a class"),
        );
        self.next_lexeme();
      }
    }

    let end = self.lexer.current().location;
    self.expect_and_consume_type(Type::RESERVED_END, "class");
    let location = Location::new(start.begin, end.end);

    if self.recursion_counter > 1 {
      self.report(
        name_local.location,
        format_args!(
          "Cannot declare class '{}' inside another statement or expression",
          name_local.name
        ),
      );
    }

    let copied_declarations = self.copy_temp_vector_t(&declarations);
    // 节点需以具体类型存入 classes_within_module，故保留 concrete 指针 +
    // 一次基类视图上转，不走 alloc_stat。
    let cls_class = self.alloc(AstStatClass::new(
      location,
      name_local_ptr,
      opt_node(super_),
      copied_declarations,
      exported,
      open,
    ));
    let cls = cls_class.cast::<AstStat>();

    let name_local_name = name_local.name;
    if self.classes_within_module.contains(&name_local_name) {
      let expressions = self.copy_initializer_list_t(&[]);
      let statements = self.copy_initializer_list_t(&[cls]);
      return self.report_stat_error(
        name_local.location,
        expressions,
        statements,
        format_args!(
          "A class named '{}' has already been declared in this module",
          name_local_name
        ),
      );
    }
    // cpp:1707 `classesWithinModule[nameLocal->name] = cls;`——映射到类声明本身，
    // 供赋值处（isExprLValue / reportLValueError）按名找回并报错其定义行。
    *self.classes_within_module.get_or_insert(name_local_name) = cls_class;
    cls
  }
}
