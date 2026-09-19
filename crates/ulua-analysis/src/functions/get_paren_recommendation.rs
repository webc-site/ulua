use ulua_ast::records::ast_node::AstNode;

use crate::{
  enums::{
    parentheses_recommendation::ParenthesesRecommendation, type_correct_kind::TypeCorrectKind,
  },
  functions::{
    follow_type::follow_type_id,
    get_paren_recommendation_for_func::get_paren_recommendation_for_func,
    get_paren_recommendation_for_intersect::get_paren_recommendation_for_intersect,
    get_type_alt_j::get_type_id,
  },
  records::{function_type::FunctionType, intersection_type::IntersectionType},
  type_aliases::type_id::TypeId,
};
pub fn get_paren_recommendation(
  id: TypeId,
  nodes: &[*mut AstNode],
  type_correct: TypeCorrectKind,
) -> ParenthesesRecommendation {
  if type_correct == TypeCorrectKind::Correct {
    return ParenthesesRecommendation::None;
  }

  let id = follow_type_id(id);

  if let Some(func) = get_type_id::<FunctionType>(id) {
    return get_paren_recommendation_for_func(func, nodes);
  }

  if let Some(intersect) = get_type_id::<IntersectionType>(id) {
    return get_paren_recommendation_for_intersect(intersect, nodes);
  }

  ParenthesesRecommendation::None
}
