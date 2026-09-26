use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_call::AstExprCall, ast_expr_constant_string::AstExprConstantString,
    ast_expr_error::AstExprError, ast_expr_interp_string::AstExprInterpString, ast_node::AstNode,
    position::Position,
  },
  rtti::{ast_node_is_ptr, ast_node_try_as_ptr},
};

use crate::{
  functions::{
    convert_require_suggestions_to_autocomplete_entry_map::convert_require_suggestions_to_autocomplete_entry_map,
    follow_type, get_method_containing_extern_type::get_method_containing_extern_type,
    get_string_contents::get_string_contents, get_type,
    process_require_suggestions::process_require_suggestions,
  },
  records::{
    file_resolver::FileResolver, function_type::FunctionType, intersection_type::IntersectionType,
  },
  type_aliases::{
    autocomplete_entry_map::AutocompleteEntryMap, module_ptr_module::ModulePtr,
    string_completion_callback::StringCompletionCallback,
  },
};
const K_REQUIRE_TAG_NAME: &str = "require";

pub fn autocomplete_string_params(
  module: &ModulePtr,
  nodes: &[*mut AstNode],
  position: Position,
  // `dyn` 保留：`FileResolver` 实现方集合运行期开放（宿主跨 crate 注入）。
  file_resolver: Option<&dyn FileResolver>,
  callback: StringCompletionCallback,
) -> Option<AutocompleteEntryMap> {
  if nodes.len() < 2 {
    return None;
  }

  let last = *nodes.last()?;
  if !unsafe { ast_node_is_ptr::<AstExprConstantString>(last) }
    && !is_simple_interpolated_string(last)
    && !unsafe { ast_node_is_ptr::<AstExprError>(last) }
  {
    return None;
  }

  if !unsafe { ast_node_is_ptr::<AstExprError>(last) } {
    // Safety: 走到此处需 `ast_node_is` 三判之一为真（null 节点三判皆假已提前返回），
    // 故 `last` 非空且指向 arena 存活 AST 节点；`nodes` 随 Module 解析树在本次
    // 调用全程存活，此处只读基类 location 字段，不产生任何借用冲突。
    let last_location = unsafe { (*last).location };
    if last_location.end == position || last_location.begin == position {
      return None;
    }
  }

  let candidate_node = nodes[nodes.len() - 2];
  // Safety: `candidate_node` 取自 `nodes`——arena 存活 AST 节点句柄，满足
  // `ast_node_try_as_ptr` 的「null 或存活 repr(C) 节点」契约；命中即 RTTI
  // 确认实际类型为 AstExprCall，只读借用随 AST arena 在本次调用内存活。
  let candidate_ref = (unsafe { ast_node_try_as_ptr::<AstExprCall>(candidate_node) })?;

  if candidate_ref.args.size > 1 {
    // size>1 ⇒ 元素 0 存在，用安全 `as_slice` 取代裸指针 `*args.data` 解引用。
    let first_arg = candidate_ref.args.as_slice()[0];
    // Safety: `first_arg` 是 parser 绑定进 arena 的非空 AstExpr 句柄（与
    // candidate_ref 同树存活），此处仅读取其基类 location 做位置判定。
    let first_arg_location = unsafe { (*first_arg).base.location };
    if !first_arg_location.contains(position) {
      return None;
    }
  }

  let it = module
    .ast_types
    .find(&(candidate_ref.func as *const AstExpr))?;
  // Safety: `last` 为 arena 存活 AstNode 句柄（函数前段已保证其非空），满足
  // `get_string_contents` 契约；被调方仅按 RTTI 只读取字符串载荷，类型不符
  // 或空时自行返回 None。
  let candidate_string = unsafe { get_string_contents(last as *const AstNode) };

  let perform_callback = |func_type: &FunctionType| -> Option<AutocompleteEntryMap> {
    for tag in &func_type.tags {
      if tag == K_REQUIRE_TAG_NAME
        && let Some(file_resolver) = file_resolver
      {
        let suggestions = file_resolver.require_suggester().and_then(|suggester| {
          suggester.get_require_suggestions_impl(&module.name, &candidate_string)
        });
        return convert_require_suggestions_to_autocomplete_entry_map(process_require_suggestions(
          suggestions,
        ));
      }

      if let Some(ret) = callback(
        tag.clone(),
        // Safety: `candidate_ref.func` 是已判空的 AstExprCall 内 parser 绑定的
        // 非空 AstExpr 句柄，随 AST arena 存活；被调方仅做 RTTI 下转与只读遍历，
        // 满足其 `func_expr` 指向存活节点的契约。
        unsafe { get_method_containing_extern_type(module, candidate_ref.func) },
        candidate_string.clone(),
      ) {
        return Some(ret);
      }
    }

    None
  };

  let followed_id = follow_type::follow(*it);
  if let Some(function_type) = get_type::get::<FunctionType>(followed_id) {
    return perform_callback(function_type);
  }

  if let Some(intersect) = get_type::get::<IntersectionType>(followed_id) {
    for part in &intersect.parts {
      let part = follow_type::follow(*part);
      if let Some(candidate_function_type) = get_type::get::<FunctionType>(part)
        && let Some(ret) = perform_callback(candidate_function_type)
      {
        return Some(ret);
      }
    }
  }

  None
}

fn is_simple_interpolated_string(node: *const AstNode) -> bool {
  // Safety: `node` 由调用方自 `nodes` 的 `*mut AstNode` 隐式降级而来，回升
  // *mut 不新增权限（门面仅按引用→*const 使用），且满足
  // `ast_node_try_as_ptr` 的「null 或存活 repr(C) 节点」契约；命中即 RTTI
  // 确认实际类型为 AstExprInterpString，repr(C) 基址重合保证下转有效，随后
  // 仅只读 expressions 长度，AST arena 在本次调用内无写穿。
  let Some(interp_string) =
    (unsafe { ast_node_try_as_ptr::<AstExprInterpString>(node.cast_mut()) })
  else {
    return false;
  };
  interp_string.expressions.size == 0
}
