use core::ptr::from_ref;

use ulua_ast::records::{
  ast_array::AstArray, ast_name::AstName, ast_type::AstType, ast_type_reference::AstTypeReference,
  location::Location,
};
use ulua_common::enums::luau_bytecode_type::LuauBytecodeType;

#[derive(Debug, Clone)]
pub struct BuiltinAstTypes {
  pub boolean_type: AstTypeReference,
  pub number_type: AstTypeReference,
  pub integer_type: AstTypeReference,
  pub string_type: AstTypeReference,
  pub vector_type: AstTypeReference,
  pub host_vector_type: AstTypeReference,
}

impl BuiltinAstTypes {
  /// cpp `&builtinTypes->XType` 惯用取址的收拢：派生类型经 `#[repr(C)]` 首字段
  /// 基址重合转成基类只读指针。节点本体由 `Compiler.builtin_types` 持有，
  /// 存入类型图后不再随 `Compiler` 移动（与 cpp 成员地址语义一致）。
  #[inline]
  fn node(ty: &AstTypeReference) -> *const AstType {
    from_ref(ty).cast::<AstType>()
  }

  #[inline]
  pub(crate) fn boolean_node(&self) -> *const AstType {
    Self::node(&self.boolean_type)
  }

  #[inline]
  pub(crate) fn number_node(&self) -> *const AstType {
    Self::node(&self.number_type)
  }

  #[inline]
  pub(crate) fn integer_node(&self) -> *const AstType {
    Self::node(&self.integer_type)
  }

  #[inline]
  pub(crate) fn string_node(&self) -> *const AstType {
    Self::node(&self.string_type)
  }

  #[inline]
  pub(crate) fn vector_node(&self) -> *const AstType {
    Self::node(&self.vector_type)
  }

  /// 字节码基础类型对应的内建 AST 类型节点指针。
  #[inline]
  pub(crate) fn node_for_bytecode_type(&self, ty: LuauBytecodeType) -> Option<*const AstType> {
    match ty {
      LuauBytecodeType::LBC_TYPE_BOOLEAN => Some(self.boolean_node()),
      LuauBytecodeType::LBC_TYPE_NUMBER => Some(self.number_node()),
      LuauBytecodeType::LBC_TYPE_INTEGER => Some(self.integer_node()),
      LuauBytecodeType::LBC_TYPE_STRING => Some(self.string_node()),
      LuauBytecodeType::LBC_TYPE_VECTOR => Some(self.vector_node()),
      _ => None,
    }
  }

  pub fn new(host_vector_type_name: *const u8) -> Self {
    // C++ 以 `AstTypeReference{{}, nullopt, AstName{"<name>"}, nullopt, {}}`
    // 初始化每个内建类型，其构造函数会打上 AstTypeReference 的 RTTI class
    // index。旧移植把 `empty_ref` 全零填充（class_index 0、空名）后克隆分发，
    // 致使 get_type 认不出内建类型引用、把所有字面量解析为 ANY——number/
    // vector 类型信息整体失效（如可交换的 ADDK/MULK 优化）。
    let loc = Location::default();
    let empty_arr = AstArray::default();

    let make_ref =
      |name: AstName| AstTypeReference::new(loc, None, name, None, loc, false, empty_arr);

    Self {
      boolean_type: make_ref(AstName::from_static(b"boolean")),
      number_type: make_ref(AstName::from_static(b"number")),
      integer_type: make_ref(AstName::from_static(b"integer")),
      string_type: make_ref(AstName::from_static(b"string")),
      vector_type: make_ref(AstName::from_static(b"vector")),
      host_vector_type: make_ref(AstName::ast_name_u8(host_vector_type_name)),
    }
  }
}

impl Default for BuiltinAstTypes {
  fn default() -> Self {
    use ulua_ast::records::{
      ast_name::AstName, ast_type_reference::AstTypeReference, location::Location,
      position::Position,
    };

    let loc = Location::new(Position::new(0, 0), Position::new(0, 0));
    let empty_arr = AstArray::default();

    let make_ref = |name: &'static [u8]| {
      AstTypeReference::new(
        loc,
        None,
        AstName::from_static(name),
        None,
        loc,
        false,
        empty_arr,
      )
    };

    Self {
      boolean_type: make_ref(b"boolean\0"),
      number_type: make_ref(b"number\0"),
      integer_type: make_ref(b"integer\0"),
      string_type: make_ref(b"string\0"),
      vector_type: make_ref(b"vector\0"),
      host_vector_type: make_ref(b"vector\0"),
    }
  }
}
