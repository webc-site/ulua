//! cpp `Inference{ty, refinement}`（refinement 默认实参 nullptr）的 Option
//! 收口：调用点的 `map_or_else(no_refinement, inference_...)` 同形折叠归此。
use crate::{
  records::inference::Inference,
  type_aliases::{refinement_id_refinement::RefinementId, type_id::TypeId},
};

pub(crate) fn inference_with_refinement(ty: TypeId, refinement: Option<RefinementId>) -> Inference {
  refinement.map_or_else(
    || Inference::no_refinement(ty),
    |r| Inference::inference_type_id_refinement_id(ty, r),
  )
}
