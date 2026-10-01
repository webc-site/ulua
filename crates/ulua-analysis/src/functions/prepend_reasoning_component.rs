//! `SubtypingResult::with_sub_component`/`with_super_component` 的同形核心：
//! 空 reasoning 时插入单组件路径，否则整组克隆并在指定侧前置组件
//! （cpp Subtyping.cpp 两方法逐行同形，仅 sub/super 路径字段不同）。
use alloc::vec::Vec;

use crate::{
  enums::subtyping_variance::SubtypingVariance,
  records::{path::Path, subtyping_reasoning::SubtypingReasoning},
  type_aliases::{component::Component, subtyping_reasonings::SubtypingReasonings},
};

/// 前置到哪一侧的推理路径。
pub(crate) enum ReasoningSide {
  Sub,
  Super,
}

/// 就地改写 `reasoning`：每条既有推理在 `side` 侧路径头部插入 `component`；
/// 空集则新插一条仅该侧带组件的推理（另一侧为空路径，variance 恒 Covariant，
/// 同 cpp `with_sub/super_component` 的两分支）。
pub(crate) fn prepend_component(
  reasoning: &mut SubtypingReasonings,
  component: Component,
  side: ReasoningSide,
) {
  if reasoning.empty() {
    let (sub_path, super_path) = match side {
      ReasoningSide::Sub => (Path::from_component(component), Path::default()),
      ReasoningSide::Super => (Path::default(), Path::from_component(component)),
    };
    reasoning.insert(SubtypingReasoning {
      sub_path,
      super_path,
      variance: SubtypingVariance::Covariant,
      is_property_modifier_violation: false,
    });
  } else {
    let count = reasoning.size();
    let mut items = Vec::with_capacity(count);
    let mut component = Some(component);
    for (i, r) in reasoning.iter().enumerate() {
      let comp = if i + 1 == count {
        component.take().unwrap()
      } else {
        component.as_ref().unwrap().clone()
      };
      let (sub_path, super_path) = match side {
        ReasoningSide::Sub => (r.sub_path.push_front(comp), r.super_path.clone()),
        ReasoningSide::Super => (r.sub_path.clone(), r.super_path.push_front(comp)),
      };
      items.push(SubtypingReasoning {
        sub_path,
        super_path,
        variance: r.variance,
        is_property_modifier_violation: r.is_property_modifier_violation,
      });
    }
    reasoning.clear();
    for item in items {
      reasoning.insert(item);
    }
  }
}
