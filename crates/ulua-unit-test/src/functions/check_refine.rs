use ulua_analysis::{
  records::{proposition_control_flow_graph::Proposition, refine::Refine},
  type_aliases::refinement_control_flow_graph::RefinementMember,
};

/// 断言 refine 节点的定义名、来源定义名、真假性与类型命题匹配。
pub fn check_refine(
  r: &Refine,
  def: &str,
  source: &str,
  sense: bool,
  r#type: Option<&str>,
  is_typeof: bool,
) {
  assert_eq!(def, unsafe { (*r.definition).versioned_name() });

  // r.prop 可空（C++ 指针语义）：非空时解引用取 Proposition 变体。
  let prop = r.prop;
  let prop = (!prop.is_null())
    .then(|| unsafe { <Proposition as RefinementMember>::get_if(&*prop) })
    .flatten();
  assert!(prop.is_some());
  let prop = prop.unwrap();

  assert!(!prop.ptr.is_null());
  assert_eq!(source, unsafe { (*prop.ptr).versioned_name() });
  assert_eq!(sense, prop.sense);

  if let Some(expected_type) = r#type {
    assert_eq!(Some(expected_type), prop.r#type.as_deref());
    assert_eq!(is_typeof, prop.is_typeof);
  } else {
    assert!(prop.r#type.is_none());
  }
}
