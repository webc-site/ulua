//! 类型检查组引擎（`--group=analysis`）：ulua-analysis 公开前端 API
//! （Frontend/FileResolver/ConfigResolver），与 ulua-analyze CLI 同构的最小搭建。

use ulua_analysis::{
  enums::solver_mode::SolverMode,
  functions::freeze::freeze,
  records::{
    config_resolver::ConfigResolver, file_resolver::FileResolver, frontend::Frontend,
    frontend_options::FrontendOptions, source_code::SourceCode, type_check_limits::TypeCheckLimits,
  },
  type_aliases::{frontend_callbacks::TaskQueue, module_name_type::ModuleName},
};
use ulua_ast::enums::mode::Mode;
use ulua_config::records::config::Config as LuauConfig;

/// 把当前被测源码喂给 Frontend 的内存版 FileResolver（实现公开 trait，无 internals）。
struct BenchFileResolver {
  source: String,
}

impl FileResolver for BenchFileResolver {
  fn read_source(&mut self, _name: &ModuleName) -> Option<SourceCode> {
    Some(SourceCode {
      source: self.source.clone(),
      r#type: SourceCode::MODULE,
    })
  }
}

/// bench 用配置解析器：恒定返回构造期定格的固定模式配置。所有权移交
/// `Frontend` 独占（`Box<dyn ConfigResolver>`），本实现无可变状态，故无需
/// 共享槽与指针回推（原 `#[repr(C)]` base + vtable 形态随 trait 化退役）。
struct BenchConfigResolver {
  config: &'static LuauConfig,
}

impl BenchConfigResolver {
  fn new(mode: Mode) -> Self {
    // bench 进程定位：模式配置只读、每 runner 至多一次泄漏（数十字节量级，
    // 进程随测毕退出），换取 `get_config` 返回引用的地址稳定且跨返回存活。
    Self {
      config: Box::leak(Box::new(LuauConfig {
        mode,
        ..Default::default()
      })),
    }
  }
}

impl ConfigResolver for BenchConfigResolver {
  fn get_config(&self, _name: &ModuleName, _limits: &TypeCheckLimits) -> &LuauConfig {
    self.config
  }
}

/// Frontend 搭建（可选执行模块检查）：`check=false` 时只注册/冻结内置全局，
/// 作为 `check=true` 的固定开销基线（分析组两个引擎只差这一开关，共用本函数）。
/// 两个解析器的所有权均随参数移交 `Frontend` 独占，本函数不再承载长寿契约。
pub(crate) fn run(check: bool, src: &str) -> Result<Option<String>, String> {
  let file_resolver = BenchFileResolver {
    source: src.to_owned(),
  };
  let config_resolver = BenchConfigResolver::new(Mode::Strict);
  let mut frontend = Frontend::new_boxed(
    SolverMode::New,
    Box::new(file_resolver),
    Box::new(config_resolver),
    FrontendOptions::default(),
  );

  frontend.register_builtin_globals(false);
  freeze(frontend.globals.global_types_mut());

  if check {
    let module = ModuleName::from("BenchModule");
    frontend.queue_module_check_vector_module_name(&[module]);
    let execute_tasks: TaskQueue = Box::new(|tasks, run| {
      for task in tasks {
        run(task);
      }
    });
    let checked = frontend.check_queued_modules(None, execute_tasks, |_done, _total| true);
    if checked.is_empty() {
      return Err("类型检查未产出任何模块（源码可能未通过解析）".to_owned());
    }
  }
  Ok(None)
}
