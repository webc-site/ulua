use ulua_ast::{
  records::{
    ast_node::AstNode, ast_type::AstType, ast_type_error::AstTypeError,
    ast_type_function::AstTypeFunction, ast_type_group::AstTypeGroup,
    ast_type_intersection::AstTypeIntersection, ast_type_optional::AstTypeOptional,
    ast_type_or_pack::AstTypeOrPack, ast_type_reference::AstTypeReference,
    ast_type_singleton_bool::AstTypeSingletonBool,
    ast_type_singleton_string::AstTypeSingletonString, ast_type_table::AstTypeTable,
    ast_type_typeof::AstTypeTypeof, ast_type_union::AstTypeUnion,
  },
  rtti::AstNodeClass,
};
use ulua_common::LUAU_ASSERT;

use crate::{
  functions::{arena_ref::arena_ref, ast_node_downcast::ast_node_downcast as type_downcast},
  records::data_flow_graph_builder::DataFlowGraphBuilder,
};

impl DataFlowGraphBuilder {
  /// cpp `DataFlowGraphBuilder::visitType(AstType*)` 的分派入口。类型注解在
  /// DFG 侧只为触达其中 `typeof` 表达式与泛型/包结构，本体不产生 def。
  pub fn visit_type(&mut self, t: &AstType) {
    // 类索引 match 与原 `ast_node_is` 长链同一判据且互斥，臂序无关语义。
    let node: &AstNode = &t.base;
    match node.class_index {
      AstTypeReference::CLASS_INDEX => {
        self.visit_type_reference(type_downcast::<AstTypeReference>(node))
      }
      AstTypeTable::CLASS_INDEX => self.visit_type_table(type_downcast::<AstTypeTable>(node)),
      AstTypeFunction::CLASS_INDEX => {
        self.visit_type_function(type_downcast::<AstTypeFunction>(node))
      }
      AstTypeTypeof::CLASS_INDEX => self.visit_type_typeof(type_downcast::<AstTypeTypeof>(node)),
      AstTypeOptional::CLASS_INDEX => {} // cpp：optional 不含值命名空间信息
      AstTypeUnion::CLASS_INDEX => self.visit_type_union(type_downcast::<AstTypeUnion>(node)),
      AstTypeIntersection::CLASS_INDEX => {
        self.visit_type_intersection(type_downcast::<AstTypeIntersection>(node))
      }
      AstTypeError::CLASS_INDEX => self.visit_type_error(type_downcast::<AstTypeError>(node)),
      AstTypeSingletonBool::CLASS_INDEX | AstTypeSingletonString::CLASS_INDEX => {} // ok
      AstTypeGroup::CLASS_INDEX => {
        let group = type_downcast::<AstTypeGroup>(node);
        // AstTypeGroup.type_ 是 parser 必绑定的内层类型指针（cpp 直接
        // `visitType(g->type)`）。
        let inner = arena_ref(group.type_, "AstTypeGroup.type_");
        self.visit_type(inner);
      }
      _ => LUAU_ASSERT!(false),
    }
  }

  /// cpp `visitType(AstTypeReference*)`：递归泛型实参（类型或类型包槽位）。
  pub fn visit_type_reference(&mut self, r: &AstTypeReference) {
    for param in r.parameters.as_slice() {
      // 二选一注解槽位按变体分发（判序与 cpp `if (param.type)` 一致）；载荷即
      // arena 存活节点，Error 形态在 parser 产物中不可达，对应 cpp 对 null
      // `typePack` 的解引用只剩断言。
      match *param {
        AstTypeOrPack::Type(t) => self.visit_type(t),
        AstTypeOrPack::Pack(pack) => self.visit_type_pack(pack),
        AstTypeOrPack::Error => LUAU_ASSERT!(false),
      }
    }
  }

  /// cpp `visitType(AstTypeTable*)`：逐属性 visit，索引器存在时 visit 两侧类型。
  pub fn visit_type_table(&mut self, t: &AstTypeTable) {
    for prop in t.props.as_slice() {
      // cpp 直接 visitType(p.type)：属性类型是 parser 必写的活节点。
      let ty = arena_ref(prop.r#type, "AstTableProp.r#type");
      self.visit_type(ty);
    }

    // indexer 是显式可空字段，使用前判空（cpp `if (t->indexer)` 同款）。
    // SAFETY: as_ref 把判空折叠进取引用，命中即 arena 存活 indexer 结构，
    // 分析期只读、无并存 &mut。
    if let Some(indexer) = unsafe { t.indexer.as_ref() } {
      let index_type = arena_ref(indexer.index_type, "AstTableIndexer.index_type");
      let result_type = arena_ref(indexer.result_type, "AstTableIndexer.result_type");
      self.visit_type(index_type);
      self.visit_type(result_type);
    }
  }

  /// cpp `visitType(AstTypeFunction*)`：泛型表、参数字段与返回类型包。
  pub fn visit_type_function(&mut self, f: &AstTypeFunction) {
    self.visit_generics(f.generics);
    self.visit_generic_packs(f.generic_packs);
    self.visit_type_list(f.arg_types);

    let return_types = arena_ref(f.return_types, "AstTypeFunction.return_types");
    self.visit_type_pack(return_types);
  }

  /// cpp `visitType(AstTypeTypeof*)`：`typeof e` 的表达式进入值命名空间遍历。
  pub fn visit_type_typeof(&mut self, t: &AstTypeTypeof) {
    let inner = arena_ref(t.expr, "AstTypeTypeof.expr");
    self.visit_expr(inner);
  }

  /// cpp `visitType(AstTypeUnion*)`。
  pub fn visit_type_union(&mut self, u: &AstTypeUnion) {
    for t in u.types.iter_nodes() {
      self.visit_type(t);
    }
  }

  /// cpp `visitType(AstTypeIntersection*)`。
  pub fn visit_type_intersection(&mut self, i: &AstTypeIntersection) {
    for t in i.types.iter_nodes() {
      self.visit_type(t);
    }
  }

  /// cpp `visitType(AstTypeError*)`：错误恢复节点的各候选类型继续遍历。
  pub fn visit_type_error(&mut self, error: &AstTypeError) {
    for t in error.types.iter_nodes() {
      self.visit_type(t);
    }
  }
}
