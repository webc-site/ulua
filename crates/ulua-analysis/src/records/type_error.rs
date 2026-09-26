use ulua_ast::records::location::Location;

use crate::type_aliases::{module_name_type::ModuleName, type_error_data::TypeErrorData};

#[derive(Debug, Clone, PartialEq)]
pub struct TypeError {
  pub location: Location,
  pub module_name: ModuleName,
  pub data: TypeErrorData,
}

// Safety: `data: TypeErrorData` 的众多变体内嵌 `TypeId = *const Type` 等借用自类型 arena
// 的裸指针，令自动 Send 失效。这些指针仅承载错误信息的类型身份、从不解引用；
// 只要 arena 在 `TypeError` 存活期内有效，将其转移到其它线程即可靠。
unsafe impl Send for TypeError {}
// Safety: 同上——变体内裸 arena id 只被只读访问以格式化错误，跨线程共享 `&TypeError`
// 仅复制这些只读身份值，在底层 arena 有效期间不产生数据竞争。
unsafe impl Sync for TypeError {}
