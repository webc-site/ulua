use alloc::{string::String, vec::Vec};

use crate::records::to_string_span::ToStringSpan;
#[derive(Debug, Clone, Default)]
pub struct ElementResult {
  pub str: String,
  pub spans: Vec<ToStringSpan>,
}

// Safety: `spans` 内 `ToStringSpan::r#type` 为 `TypeId = *const Type`，令自动 Send
// 失效。该指针只作渲染用的类型身份键、从不解引用；只要所属类型 arena 在 `ElementResult`
// 存活期内有效，把裸指针身份值转移到其它线程即可靠，故 ElementResult 满足 Send。
unsafe impl Send for ElementResult {}
// Safety: 同上——裸 `TypeId` 仅作只读身份键，共享 `&ElementResult` 在类型 arena 有效
// 期间不产生数据竞争，故 ElementResult 满足 Sync。
unsafe impl Sync for ElementResult {}
