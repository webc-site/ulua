use crate::{
  functions::baseof::baseof,
  records::{field::Field, symbol::Symbol},
  type_aliases::l_value::{LValue, LValueMember},
};

pub fn get_base_symbol(lvalue: &LValue) -> Symbol {
  let mut current: *const LValue = lvalue;

  // Safety: `current` 初值取自存活的 `&LValue`（非空、对齐）；循环内每次 `baseof(&*current)`
  // 沿 LValue 的 base 链上溯，返回的 base 指针指向仍由根 `lvalue` 借用所拥有、内嵌存活的
  // LValue 节点，故整条链在本借用期内始终有效。全程只做 `&*current` 只读借用、单线程串行
  // 遍历，重建的引用无别名冲突。
  unsafe {
    while <Field as LValueMember>::get_if(&*current).is_some() {
      current = baseof(&*current);
    }

    let symbol = <Symbol as LValueMember>::get_if(&*current);
    debug_assert!(symbol.is_some());
    // debug_assert 同前提：cpp 遍历契约保证 Field 链终点必为 Symbol。
    symbol
      .expect("Field 链终点按 cpp `get<Symbol>` 契约必为 Symbol")
      .clone()
  }
}
