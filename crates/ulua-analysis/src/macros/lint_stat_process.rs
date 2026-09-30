//! `Lint*::process` 入口模板单点。
//!
//! cpp 侧每个 lint 检查器的 `process(LintContext&)` 都以同一形状开场：取
//! `context.root`、以宿主借用构造 pass、`ast_stat_visit` 遍历整棵语句树。
//! 本仓库原先把这副骨架在 16 个 `lint_*_process.rs` 里各手抄一遍。
//! [`lint_stat_process!`] 展开逐字等价（含统一 SAFETY 注释），调用点只剩
//! 「pass 类型名（+ 字面量初始化的额外字段）」。

/// 生成 `Lint*` 的 `pub fn process(context: &mut LintContext)` 模板。
///
/// 用法：
/// ```ignore
/// lint_stat_process!(LintForRange);
/// lint_stat_process!(#[inline(never)] LintUnbalancedAssignment);
/// lint_stat_process!(LintDuplicateFunction { defns: DenseHashMap::default() });
/// ```
/// `$pass` 构造中 `context: LintContextHandle::from_ref(context)` 固定为首字段，
/// 额外字段以调用点字面量补写；`LintContext`/`LintContextHandle`/`ast_stat_visit`
/// 按宏展开点解析（各 `lint_*_process.rs` 已 import）。
macro_rules! lint_stat_process {
  ($(#[$attr:meta])? $pass:ident) => {
    lint_stat_process!($(#[$attr])? $pass {});
  };
  ($(#[$attr:meta])? $pass:ident { $($field:ident : $value:expr),* $(,)? }) => {
    $(#[$attr])?
    pub fn process(context: &mut LintContext) {
      let root = context.root;
      let mut pass = $pass {
        context: LintContextHandle::from_ref(context),
        $($field: $value),*
      };
      // SAFETY: root 为 null 或贯穿整趟 lint pass 存活的 arena AstStat；遍历为
      // 单线程串行，宿主 LintContext 的写句柄由本 pass 独占。
      unsafe {
        ast_stat_visit(root, &mut pass);
      }
    }
  };
}

pub(crate) use lint_stat_process;
