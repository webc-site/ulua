use ulua_ast::{
  records::{
    ast_name::AstName, ast_stat_type_alias::AstStatTypeAlias, ast_type::AstType,
    ast_type_function::AstTypeFunction, ast_type_group::AstTypeGroup,
    ast_type_intersection::AstTypeIntersection, ast_type_optional::AstTypeOptional,
    ast_type_reference::AstTypeReference, ast_type_singleton_bool::AstTypeSingletonBool,
    ast_type_singleton_string::AstTypeSingletonString, ast_type_table::AstTypeTable,
    ast_type_union::AstTypeUnion,
  },
  rtti::{ast_node_is, ast_node_try_as},
};
use ulua_bytecode::records::bytecode_builder::BytecodeBuilder;
use ulua_common::{
  enums::luau_bytecode_type::LuauBytecodeType,
  fflag,
  records::{dense_hash_map::DenseHashMap, dense_hash_set::DenseHashSet},
};

use crate::{
  functions::{
    get_primitive_type::get_primitive_type,
    is_generic::{GenericList, is_generic},
  },
  records::node::Node,
};

/// 对应 cpp `getType`（cpp/Compiler/src/Types.cpp:45）：把注解 AST 类型求值为字节码
/// `LuauBytecodeType`，别名经 `type_aliases` 解析、union 逐成员下钻取公共类型。
///
/// `ty` 取 [`Node`] 身份句柄而非裸指针，承担「存活 arena 节点」契约（§2）。
///
/// DELIBERATE DEVIATION（§9.3 内存安全必需）：`ty` 为 `None` 时返回
/// `LBC_TYPE_ANY`。cpp 侧 `getType(nullptr, …)` 在第一句 `ty->as<AstTypeReference>()`
/// 即解空指针（未定义行为，无可观察的“正确”结果可对齐）；本端口在
/// `recordResolvedType` 路径上确实可能把别名解析后的空槽交进来（块级别名出作用域
/// 后旧绑定是 null），故以 `LBC_TYPE_ANY` 表达「类型不可判定」——与 cpp 中
/// `ref->prefix` 存在、别名递归命中等「放弃精确编码」的分支同一语义档位。
/// `generics` 各元素须为 parser 登记的存活 `AstGenericType` 指针（`is_generic` 读
/// `name` 作比较）；`host_vector_type` 为已在边界包装好的只读字节切片；
/// 宿主向量类型名（None 表示未配置）。
///
/// 仅本 crate 的类型图阶段消费（`TypeMapVisitor`/`get_function_type`），故 `pub(crate)`。
pub(crate) fn get_type<G: GenericList + ?Sized>(
  ty: Option<Node<AstType>>,
  generics: &G,
  type_aliases: &DenseHashMap<AstName, Option<Node<AstStatTypeAlias>>>,
  host_vector_type: Option<&[u8]>,
  userdata_types: &DenseHashMap<AstName, u8>,
  bytecode: &mut BytecodeBuilder,
  seen_aliases: &mut DenseHashSet<AstName>,
) -> LuauBytecodeType {
  let Some(ty_node) = ty else {
    return LuauBytecodeType::LBC_TYPE_ANY;
  };
  let ty_ref = ty_node.get();

  if let Some(ref_node) = ast_node_try_as::<AstTypeReference>(&ty_ref.base) {
    if ref_node.prefix.is_some() {
      return LuauBytecodeType::LBC_TYPE_ANY;
    }

    // 对应 C++ `if (alias && *alias)`——条目可能缺席或以 null 值存在（块级别名
    // 出块时被还原为先前的空绑定），此时它不在作用域内，必须落到
    // 通用/userdata 解析路径。`Option<Node>` 值 + `flatten` 即该双折叠。
    if let Some(alias_node) = type_aliases.find(&ref_node.name).copied().flatten() {
      let alias = alias_node.get();
      if seen_aliases.contains(&alias.name) {
        if !fflag::LuauCompileRecursiveAliases.get() {
          seen_aliases.clear();
        }
        return LuauBytecodeType::LBC_TYPE_ANY;
      }

      // 递归/非递归两臂只差解析后是否回溯撤销登记：插入→解析→按旗标抹除
      // 收口为一条路径，alias.type_ptr 的递归调用点只保留一份。cpp
      // `getType((*alias)->type, ...)`（Types.cpp:72/79）不判 null 直接递归，
      // type_ptr 槽已句柄化为非空 `Node`，无需再经 `try_new` 折叠 null。
      let recursive = fflag::LuauCompileRecursiveAliases.get();
      seen_aliases.insert(ref_node.name);
      let resolved = get_type(
        Some(Node::from_ast_handle(alias.type_ptr)),
        &alias.generics,
        type_aliases,
        host_vector_type,
        userdata_types,
        bytecode,
        seen_aliases,
      );
      if recursive {
        seen_aliases.erase(&ref_node.name);
      }
      return resolved;
    }

    if is_generic(ref_node.name, generics) {
      return LuauBytecodeType::LBC_TYPE_ANY;
    }

    // ref_node.name 为空名时比较直接 false，非空即与 host 串比较字节内容。
    if host_vector_type.is_some_and(|host| ref_node.name == host) {
      return LuauBytecodeType::LBC_TYPE_VECTOR;
    }

    let prim = get_primitive_type(ref_node.name);
    if prim != LuauBytecodeType::LBC_TYPE_INVALID {
      return prim;
    }

    if let Some(userdata_index) = userdata_types.find(&ref_node.name).copied() {
      bytecode.use_userdata_type(userdata_index as u32);
      return LuauBytecodeType(
        LuauBytecodeType::LBC_TYPE_TAGGED_USERDATA_BASE.0 + userdata_index as u16,
      );
    }

    // 非原生/别名/泛型 => 宿主提供类型，目前按 userdata 处理
    return LuauBytecodeType::LBC_TYPE_USERDATA;
  } else if ast_node_is::<AstTypeTable>(&ty_ref.base) {
    return LuauBytecodeType::LBC_TYPE_TABLE;
  } else if ast_node_is::<AstTypeFunction>(&ty_ref.base) {
    return LuauBytecodeType::LBC_TYPE_FUNCTION;
  } else if let Some(un) = ast_node_try_as::<AstTypeUnion>(&ty_ref.base) {
    let mut optional = false;
    let mut r#type = LuauBytecodeType::LBC_TYPE_INVALID;

    for &ty in un.types.as_slice() {
      // union 成员由 parser 保证为非空存活 AstType；可空成员解析为
      // AstTypeOptional 节点而非 null 指针，`Node::try_new` 把 null 归一为 ANY。
      let et = get_type(
        Node::try_new(ty),
        generics,
        type_aliases,
        host_vector_type,
        userdata_types,
        bytecode,
        seen_aliases,
      );

      if et == LuauBytecodeType::LBC_TYPE_NIL {
        optional = true;
        continue;
      }

      if r#type == LuauBytecodeType::LBC_TYPE_INVALID {
        r#type = et;
        continue;
      }

      if r#type != et {
        return LuauBytecodeType::LBC_TYPE_ANY;
      }
    }

    if r#type == LuauBytecodeType::LBC_TYPE_INVALID {
      return LuauBytecodeType::LBC_TYPE_ANY;
    }

    return LuauBytecodeType(
      r#type.0
        | (if optional && (r#type != LuauBytecodeType::LBC_TYPE_ANY) {
          LuauBytecodeType::LBC_TYPE_OPTIONAL_BIT.0
        } else {
          0
        }),
    );
  } else if ast_node_is::<AstTypeIntersection>(&ty_ref.base) {
    return LuauBytecodeType::LBC_TYPE_ANY;
  } else if let Some(group) = ast_node_try_as::<AstTypeGroup>(&ty_ref.base) {
    // group.type_ 为 parser 保证非空存活的被括号包裹类型节点，槽已句柄化
    // 为非空 `Node`（cpp `getType(group->type, ...)` 同样不判 null）。
    return get_type(
      Some(Node::from_ast_handle(group.type_)),
      generics,
      type_aliases,
      host_vector_type,
      userdata_types,
      bytecode,
      seen_aliases,
    );
  } else if ast_node_is::<AstTypeOptional>(&ty_ref.base) {
    return LuauBytecodeType::LBC_TYPE_NIL;
  } else if ast_node_is::<AstTypeSingletonBool>(&ty_ref.base) {
    return LuauBytecodeType::LBC_TYPE_BOOLEAN; // C++ returns LuauBytecodeType::LBC_TYPE_BOOLEAN for `true`/`false`
  } else if ast_node_is::<AstTypeSingletonString>(&ty_ref.base) {
    return LuauBytecodeType::LBC_TYPE_STRING;
  }

  LuauBytecodeType::LBC_TYPE_ANY
}
