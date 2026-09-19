use alloc::format;

// 设置串（C++ 内联 "true"/"false"，解析比较与生成共用）
pub(crate) const SETTING_TRUE: &str = "true";
pub(crate) const SETTING_FALSE: &str = "false";

// 合法选项文本
pub(crate) const OPT_BOOL: &str = "true and false";
pub(crate) const OPT_LINT_COMPAT: &str = "enabled, disabled, and fatal";

/// `Bad setting '<value>'.  Valid options are <options>`（C++ 两处重复的报错模板）
pub(crate) fn bad_setting(value: &str, options: &str) -> String {
  format!("Bad setting '{value}'.  Valid options are {options}")
}
