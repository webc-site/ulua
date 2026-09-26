use alloc::{sync::Arc, vec::Vec};
use core::ptr;

use ulua_ast::records::{
  ast_attr::AstAttrType, ast_type::AstType, ast_type_function::AstTypeFunction,
  ast_type_pack::AstTypePack, ast_type_pack_explicit::AstTypePackExplicit, location::Location,
};

use crate::{
  functions::{arc_as_mut::arc_as_mut, invert_polarity::invert_polarity},
  records::{
    constraint_generator::ConstraintGenerator, function_argument::FunctionArgument,
    function_type::FunctionType, r#type::Type, type_level::TypeLevel,
  },
  type_aliases::{scope_ptr_type::ScopePtr, type_id::TypeId, type_variant::TypeVariant},
};
impl ConstraintGenerator {
  pub fn resolve_function_type(
    &mut self,
    scope: &ScopePtr,
    _ty: *mut AstType,
    fn_node: &AstTypeFunction,
    in_type_arguments: bool,
    replace_error_with_fresh: bool,
  ) -> TypeId {
    let has_generics = fn_node.generics.size > 0 || fn_node.generic_packs.size > 0;
    let mut signature_scope = scope.clone();

    let mut generic_types = Vec::new();
    let mut generic_type_packs = Vec::new();

    if has_generics {
      signature_scope =
        // fn_node 为调用方借出的存活 AstTypeFunction，child_scope 仅读取其
        // 基类 location 并登记（C++ childScope(fnNode, scope) 同构）。
        self.child_scope(&fn_node.base.base, scope);

      let generic_definitions =
        self.create_generics(&signature_scope, fn_node.generics, false, true);
      let generic_pack_definitions =
        self.create_generic_packs(&signature_scope, fn_node.generic_packs, false, true);

      for (_, g) in generic_definitions {
        generic_types.push(g.ty);
      }

      for (_, g) in generic_pack_definitions {
        generic_type_packs.push(g.tp);
      }
    }

    let mut temp_arg_types = AstTypePackExplicit::new(Location::default(), fn_node.arg_types);

    let p = self.polarity;
    self.polarity = invert_polarity(self.polarity);
    // Safety: resolve_type_pack_* 为 unsafe fn，其逐参预置由被调 # Safety 契约
    // 核对——`arc_as_mut(&signature_scope)` 按仓库惯用法取 Arc 的可变身份指针，
    // signature_scope（Arc<Scope>）在本调用期内被本函数独占存活；`temp_arg_types`
    // 为栈上局部，语句内存活，AstTypePackExplicit 以 repr(C) 首字段 base:
    // AstTypePack 布局，指针转换即 C++ 派生→基类 upcast 的基址重合。
    let arg_types = unsafe {
      self.resolve_type_pack_scope_ptr_ast_type_pack_bool_bool(
        arc_as_mut(&signature_scope),
        ptr::from_mut(&mut temp_arg_types).cast::<AstTypePack>(),
        in_type_arguments,
        replace_error_with_fresh,
      )
    };
    self.polarity = p;

    // Safety: 与 arg_types 侧同契约——signature_scope 仍为本函数独占持有的
    // Arc；fn_node.return_types 由 parser 保证为非空存活 AstTypePack 节点
    //（类型函数语法必选返回类型段），约束生成期无他途可变借用该 AST。
    let return_types = unsafe {
      self.resolve_type_pack_scope_ptr_ast_type_pack_bool_bool(
        arc_as_mut(&signature_scope),
        fn_node.return_types,
        in_type_arguments,
        replace_error_with_fresh,
      )
    };

    let mut ftv = FunctionType {
      definition: None,
      generics: generic_types,
      generic_packs: generic_type_packs,
      arg_names: Vec::with_capacity(fn_node.arg_names.size),
      tags: Default::default(),
      level: TypeLevel::new(0, 0),
      arg_types,
      ret_types: return_types,
      magic: None,
      has_self: false,
      has_no_free_or_generic_types: false,
      is_checked_function: fn_node.is_checked_function(),
      is_deprecated_function: false,
      deprecated_info: None,
    };

    let deprecated_attr = fn_node.get_attribute(AstAttrType::Deprecated);
    if !deprecated_attr.is_null() {
      ftv.is_deprecated_function = true;
      // Safety: deprecated_attr 判空紧邻在先；其指向 fn_node 属性列表中 parser
      // arena 持有的存活 AstAttr(Deprecated) 节点，`(*_).deprecated_info()` 取
      // &self 只读构造按值返回的 DeprecatedInfo，随即被 Arc 接管，不再借 AST。
      ftv.deprecated_info = Some(Arc::new(unsafe { (*deprecated_attr).deprecated_info() }));
    }

    for &el in fn_node.arg_names.as_slice() {
      if let Some(arg) = el {
        ftv.arg_names.push(Some(FunctionArgument {
          name: (arg.0.as_str_or_empty().to_string()),
          location: arg.1,
        }));
      } else {
        ftv.arg_names.push(None);
      }
    }

    // Safety: self.arena.as_ptr() 是 ConstraintGenerator 构造期接线的非空 TypeArena 裸句柄
    //（C++ NotNull<TypeArena>，与会话同寿）；此前对 arena 的任何借用均已随各
    // resolve 调用返回结束，此处 &mut 再借用为单线程串行窗口内的唯一借用，
    // add_type 追加 bump arena 节点、地址稳定。
    self
      .arena
      .get_mut()
      .add_type(Type::new(TypeVariant::Function(ftv)))
  }
}
