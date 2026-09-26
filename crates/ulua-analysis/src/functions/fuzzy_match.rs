use ulua_common::{fint::LuauSuggestionDistance, functions::edit_distance::edit_distance};

/// 在候选集中找编辑距离最近的建议词。候选收 `IntoIterator` 惰性消费：
/// best 直接存候选引用（不再先记下标、命中后回取下标容器），调用方无需
/// 为一次查询物化 Vec/切片——切片、数组引用、迭代器均可直接传入。
pub fn fuzzy_match<'a>(
  str: &str,
  candidates: impl IntoIterator<Item = &'a str>,
) -> Option<&'a str> {
  let suggestion_distance = LuauSuggestionDistance.get() as usize;
  if suggestion_distance == 0 {
    return None;
  }

  let bytes = str.as_bytes();

  // 平距时保留后见者（`<=`），与原下标实现逐项等价。
  let mut best_distance = suggestion_distance;
  let mut best: Option<&'a str> = None;

  for candidate in candidates {
    let ed = edit_distance(bytes, candidate.as_bytes());
    if ed <= best_distance {
      best_distance = ed;
      best = Some(candidate);
    }
  }

  best
}
