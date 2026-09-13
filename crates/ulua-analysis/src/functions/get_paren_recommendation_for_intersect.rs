use ulua_ast::records::ast_node::AstNode;

use crate::{
  enums::parentheses_recommendation::ParenthesesRecommendation,
  functions::{
    follow_type::follow_type_id,
    get_paren_recommendation_for_func::get_paren_recommendation_for_func,
    get_type_alt_j::get_type_id,
  },
  records::{function_type::FunctionType, intersection_type::IntersectionType},
};
pub fn get_paren_recommendation_for_intersect(
  intersect: &IntersectionType,
  nodes: &[*mut AstNode],
) -> ParenthesesRecommendation {
  let mut rec = ParenthesesRecommendation::None;

  for &part_id in intersect.parts.iter() {
    let part_id = follow_type_id(part_id);

    if let Some(part_func) = get_type_id::<FunctionType>(part_id) {
      let other = get_paren_recommendation_for_func(part_func, nodes);
      if other as i32 > rec as i32 {
        rec = other;
      }
    } else {
      return ParenthesesRecommendation::None;
    }
  }

  rec
}
