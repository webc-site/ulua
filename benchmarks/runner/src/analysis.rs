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

/// `#[repr(C)]` 的 `base` 首字段布局：vtable 回调收到的 `this` 即整体实例指针，
/// 与 ulua-analyze-cli 的 `CliConfigResolver` 同一契约（本 crate 无法复用其私有类型）。
#[repr(C)]
struct BenchConfigResolver {
  base: ConfigResolver,
  config: *const LuauConfig,
}

/// `getConfig` 静态实现：恒定返回构造期 `Box::leak` 的固定模式配置。
///
/// # Safety
/// `this` 必须是持有本实例 `base` 槽位（首字段）的 `BenchConfigResolver` 指针；
/// `config` 指向泄漏的 `Config`，比任何调用点长寿。
unsafe fn bench_get_config(
  this: *const ConfigResolver,
  _name: *const ModuleName,
  _limits: *const TypeCheckLimits,
) -> *const LuauConfig {
  // Safety: 契约见上——`base` 为 `#[repr(C)]` 首字段，指针回推整体安全；
  // `config` 来自 `Box::leak`，地址与内容恒定。
  unsafe { (*this.cast::<BenchConfigResolver>()).config }
}

impl BenchConfigResolver {
  fn new(mode: Mode) -> Self {
    // bench 进程定位：模式配置只读、每 runner 至多一次泄漏（数十字节量级，
    // 进程随测毕退出），换取 `getConfig` 返回引用的地址稳定。
    let config: &'static LuauConfig = Box::leak(Box::new(LuauConfig {
      mode,
      ..Default::default()
    }));
    Self {
      base: ConfigResolver {
        get_config: Some(bench_get_config),
      },
      config,
    }
  }
}

/// Frontend 搭建（可选执行模块检查）：`check=false` 时只注册/冻结内置全局，
/// 作为 `check=true` 的固定开销基线（分析组两个引擎只差这一开关，共用本函数）。
/// 解析器所有权移交 `Frontend` 独占；config resolver 本地且后于 frontend 声明，
/// frontend（Box）先析构，满足 Frontend 持有裸指针的长寿契约。
pub(crate) fn run(check: bool, src: &str) -> Result<Option<String>, String> {
  let file_resolver = BenchFileResolver {
    source: src.to_owned(),
  };
  let mut config_resolver = BenchConfigResolver::new(Mode::Strict);
  let mut frontend = Frontend::new_boxed(
    SolverMode::New,
    Box::new(file_resolver),
    Some(&mut config_resolver.base),
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
