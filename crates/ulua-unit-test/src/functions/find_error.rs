use ulua_analysis::{
  records::check_result::CheckResult, type_aliases::type_error_data::TypeErrorDataMember,
};

pub fn find_error<E: Clone + TypeErrorDataMember>(result: &CheckResult) -> Option<E> {
  // 单次遍历 + 单次 clone：直接从错误数据提取并克隆，找到即返回
  result
    .errors
    .iter()
    .find_map(|error| E::get_if(&error.data).cloned())
}
