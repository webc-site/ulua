use ulua_analysis::{
  records::{proposition_control_flow_graph::Proposition, refine::Refine},
  type_aliases::refinement_control_flow_graph::RefinementMember,
};
/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn check_refine(
  r: *mut Refine,
  def: &str,
  source: &str,
  sense: bool,
  r#type: Option<&str>,
  is_typeof: bool,
) {
  assert!(!r.is_null());

  unsafe {
    let refine = &*r;

    assert_eq!(def, (*refine.definition).versioned_name());

    let prop = <Proposition as RefinementMember>::get_if(&*refine.prop);
    assert!(prop.is_some());
    let prop = prop.unwrap();

    assert!(!prop.ptr.is_null());
    assert_eq!(source, (*prop.ptr).versioned_name());
    assert_eq!(sense, prop.sense);

    if let Some(expected_type) = r#type {
      assert_eq!(Some(expected_type), prop.r#type.as_deref());
      assert_eq!(is_typeof, prop.is_typeof);
    } else {
      assert!(prop.r#type.is_none());
    }
  }
}
