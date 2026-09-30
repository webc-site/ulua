use ulua_analysis::{
  records::{
    proposition_control_flow_graph::Proposition, refine::Refine, sym_def_registry::resolve_sym_def,
  },
  type_aliases::{
    def_id_control_flow_graph::DefId, refinement_control_flow_graph::RefinementMember,
  },
};

/// 句柄 → 版本名：def 由构建期经注册表发放（NotNull 语义非空），解析未命中
/// 即契约被破坏，panic 比 cpp 悬垂解引用保守。
fn versioned_name(def: DefId) -> String {
  resolve_sym_def(def)
    .expect("DefId 非空（NotNull 语义）")
    .versioned_name()
}

/// 断言 refine 节点的定义名、来源定义名、真假性与类型命题匹配。
pub fn check_refine(
  r: &Refine,
  def: &str,
  source: &str,
  sense: bool,
  r#type: Option<&str>,
  is_typeof: bool,
) {
  assert_eq!(def, versioned_name(r.definition));

  // r.prop 由类型 `RefinementId = Handle<Refinement>` 编码非空（b14 起），
  // `get()` 物化只读借用，get_if 读 tag 判变体，Option 引用随帧结束。
  let prop = <Proposition as RefinementMember>::get_if(r.prop.get());
  let Some(prop) = prop else {
    panic!("r.prop 物化必须命中变体");
  };

  assert!(!prop.ptr.is_null());
  assert_eq!(source, versioned_name(prop.ptr));
  assert_eq!(sense, prop.sense);

  if let Some(expected_type) = r#type {
    assert_eq!(Some(expected_type), prop.r#type.as_deref());
    assert_eq!(is_typeof, prop.is_typeof);
  } else {
    assert!(prop.r#type.is_none());
  }
}
