//! `Location` / `Position` 端口对照 `cpp/Ast/src/Location.cpp` 的行为测试。
//!
//! C++ 没有独立单元测试文件，这些是「语义直译 + 边界」测试：覆盖
//! `contains`/`containsClosed`/`encloses`/`overlaps`/`extend`/`shift` 及
//! `Position::missing/has_value`，锁定半开区间 `begin..end` 与
//! `unsigned` 回绕等关键契约，防止后续重构出现越界或回退。

use ulua_ast::records::{location::Location, position::Position};

fn p(line: u32, column: u32) -> Position {
  Position { line, column }
}

fn loc(bl: u32, bc: u32, el: u32, ec: u32) -> Location {
  Location {
    begin: p(bl, bc),
    end: p(el, ec),
  }
}

#[test]
fn default_location_is_all_zero() {
  // 对照 cpp `Location() : begin(0,0), end(0,0)`
  let l = Location::default();
  assert_eq!(l, loc(0, 0, 0, 0));
}

#[test]
fn half_open_contains() {
  let l = loc(0, 2, 0, 5);
  // begin 闭、end 开：cpp `begin <= p && p < end`
  assert!(l.contains(p(0, 2)));
  assert!(l.contains(p(0, 4)));
  assert!(!l.contains(p(0, 5)));
  assert!(!l.contains(p(0, 1)));
  // 行序：cpp `Position::operator<` 先比较 line 再 column
  assert!(!l.contains(p(1, 0)));
}

#[test]
fn contains_closed_includes_end() {
  let l = loc(0, 2, 0, 5);
  assert!(l.contains_closed(p(0, 5)));
  assert!(!l.contains_closed(p(0, 6)));
}

#[test]
fn encloses_requires_full_coverage() {
  let outer = loc(0, 0, 2, 0);
  assert!(outer.encloses(&loc(0, 1, 1, 5)));
  assert!(outer.encloses(&outer));
  assert!(!outer.encloses(&loc(0, 0, 2, 1)));
}

#[test]
fn overlaps_three_branches() {
  let l = loc(1, 0, 3, 0);
  // 起点在 l 内部
  assert!(l.overlaps(&loc(2, 0, 5, 0)));
  // 终点在 l 内部
  assert!(l.overlaps(&loc(0, 0, 2, 0)));
  // l 完全在 other 内部
  assert!(l.overlaps(&loc(0, 0, 5, 0)));
  // 相切（end == other.begin）：cpp `<= && >=` 判定为 true
  assert!(l.overlaps(&loc(3, 0, 5, 0)));
  // 分离
  assert!(!l.overlaps(&loc(4, 0, 5, 0)));
}

#[test]
fn extend_grows_to_union() {
  let mut l = loc(1, 1, 2, 2);
  l.extend(&loc(0, 9, 1, 0));
  assert_eq!(l, loc(0, 9, 2, 2));

  let mut r = loc(1, 1, 2, 2);
  r.extend(&loc(1, 1, 2, 2)); // 相同区间不变
  assert_eq!(r, loc(1, 1, 2, 2));

  // 不相交且不包含：只可能扩展某一端，不会收缩
  let mut s = loc(5, 0, 6, 0);
  s.extend(&loc(1, 0, 2, 0));
  assert_eq!(s, loc(1, 0, 6, 0));
}

#[test]
fn shift_replaces_or_offsets() {
  // cpp：`*this >= start` 才生效；同段内改写到 newEnd.line 且平移 column 差。
  let mut pos = p(2, 10);
  pos.shift(&p(1, 0), &p(3, 20), &p(3, 30));
  // line(2) <= oldEnd.line(3)，落入 else 分支：line=newEnd.line=3,
  // column += (30-20)=10 → 10+10=20
  assert_eq!(pos, p(3, 20));

  // 位于 oldEnd 之后：整段行号平移
  let mut after = p(5, 0);
  after.shift(&p(1, 0), &p(3, 0), &p(4, 0));
  assert_eq!(after, p(6, 0));

  // 位于 start 之前：不改动
  let mut before = p(0, 5);
  before.shift(&p(1, 0), &p(3, 0), &p(4, 0));
  assert_eq!(before, p(0, 5));
}

#[test]
fn shift_wraps_like_unsigned() {
  // C++ `unsigned int` 减法回绕；替换（newEnd 短于 oldEnd）时 column 下溢。
  let mut pos = p(2, 5);
  pos.shift(&p(2, 0), &p(2, 100), &p(2, 10));
  // line(2) == oldEnd.line(2)，走 else 分支：line=2, column += 10-100 → u32 回绕
  assert_eq!(pos, p(2, 5u32.wrapping_add(10u32.wrapping_sub(100))));
}

#[test]
fn location_shift_moves_both_endpoints() {
  let mut l = loc(2, 0, 5, 0);
  l.shift(&p(1, 0), &p(4, 0), &p(6, 0));
  // begin: line 2 <= 4 → line=6, column += 0; end: line 5 > 4 → += (6-4) = 7
  assert_eq!(l, loc(6, 0, 7, 0));
}

#[test]
fn position_missing_and_has_value() {
  // cpp `Position::missing() = {UINT_MAX, UINT_MAX}`；`hasValue = line||column != UINT_MAX`
  let m = Position::missing();
  assert_eq!(m, p(u32::MAX, u32::MAX));
  assert!(!m.has_value());
  assert!(p(0, 0).has_value());
  // 单字段非 UINT_MAX 即视为有值（cpp `||` 语义）
  assert!(p(0, u32::MAX).has_value());
  assert!(p(u32::MAX, 0).has_value());
}

#[test]
fn location_with_length_and_between() {
  // `Location(begin, length)` 只改列，不跨行
  let l = Location::with_length(p(3, 4), 10);
  assert_eq!(l, loc(3, 4, 3, 14));

  // `Location(begin_loc, end_loc)`：跨两个子节点取包络
  let between = Location::between(loc(1, 2, 1, 9), loc(4, 0, 6, 3));
  assert_eq!(between, loc(1, 2, 6, 3));
}

#[test]
fn position_ordering_line_then_column() {
  // cpp `operator<` 先 line 后 column；Rust `Ord` derive 字段声明序一致
  assert!(p(0, 9) < p(1, 0));
  assert!(p(1, 0) < p(1, 1));
  assert_eq!(p(2, 3).max(p(2, 4)), p(2, 4));
}
