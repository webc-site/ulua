use alloc::{string::String, sync::Arc, vec::Vec};

use ulua_common::{
  fflag,
  macros::luau_timetrace_scope::{LUAU_TIMETRACE_ARGUMENT, LUAU_TIMETRACE_SCOPE},
};
use ulua_config::records::config::Config;

use crate::{
  functions::{
    arc_as_mut::arc_as_mut, get_timestamp::get_timestamp, trace_requires::trace_requires,
  },
  records::{
    arena_handle::Handle, frontend::Frontend, source_module::SourceModule, source_node::SourceNode,
    type_check_limits::TypeCheckLimits,
  },
  type_aliases::module_name_type::ModuleName,
};
impl Frontend {
  /// cpp `std::pair<SourceNode*, SourceModule*> Frontend::getSourceNode` 的
  /// 孪生收口形态（与 [`Frontend::get_source_module`] 族同法，b11 步⑤）：
  /// 两个返回位各以 `Option<Handle<T>>` 承载原 `null_mut()` 哨兵——判空与
  /// null 语义一一对应，调用点不再触碰裸指针返回值。
  ///
  /// 第二元素为 `None` 仅出现在「`source_nodes` 有而 `source_modules` 无」
  /// 的 C++ `LUAU_ASSERT` 违约路径（原实现即返回裸 null 交调用点自证）。
  pub fn get_source_node(
    &mut self,
    name: &ModuleName,
    limits: &TypeCheckLimits,
  ) -> (Option<Handle<SourceNode>>, Option<Handle<SourceModule>>) {
    let already_present = match self.source_nodes.get(name) {
      Some(node) if !node.has_dirty_source_module() => {
        // Safety: `node`/`module` 为 `Arc` 宿主表内实例的别名句柄取法，与
        // `Frontend::get_source_module_mut` 同一契约（目标随表存活、单线程
        // 串行驱动），指针直接取自 `Arc::as_ptr` 保持堆分配原始 provenance。
        let node_handle = Some(unsafe { Handle::from_raw(Arc::as_ptr(node).cast_mut()) });
        let module_handle = self
          .source_modules
          .get(name)
          .map(|module| unsafe { Handle::from_raw(Arc::as_ptr(module).cast_mut()) });
        if module_handle.is_some() {
          return (node_handle, module_handle);
        } else {
          // LUAU_ASSERT(!"Everything in sourceNodes should also be in sourceModules");
          return (node_handle, None);
        }
      }
      Some(_) => true,
      None => false,
    };

    LUAU_TIMETRACE_SCOPE!("Frontend::getSourceNode", "Frontend");
    LUAU_TIMETRACE_ARGUMENT!("name", name.as_str());

    let timestamp = get_timestamp();

    let source = self.file_resolver_mut().read_source(name);
    let environment_name = self.file_resolver_ref().get_environment_for_module(name);

    self.stats.time_read += get_timestamp() - timestamp;

    let source = match source {
      Some(source) => source,
      None => {
        // C++ `if (FFlag::LuauFrontendSourceNodeErase)`: a source that vanished
        // from the resolver is evicted from every cache; with the flag off only
        // the parsed `SourceModule` is dropped and the stale `SourceNode` stays.
        if fflag::LuauFrontendSourceNodeErase.get() {
          self.detach_source_node(name);
          self.erase_module_caches(name);
        } else {
          self.source_modules.remove(name);
        }

        return (None, None);
      }
    };

    let mut opts = {
      // Safety: `self.config_resolver` 的非空解引用收敛于 `config_resolver_ref`
      // chokepoint，返回的借用指向与 Frontend 同存亡的解析器。`.get_config`
      // 是经 `.expect` 校验
      // 已设置（None 即 panic）的宿主回调函数指针。以 `as_ptr()` 作 self 参数同步调用它符合
      // C++ `ConfigResolver::getConfig` 返回 `const Config&` 的约定——返回的 `*const Config`
      // 指向解析器持有、在本次同步调用期间存活的配置对象，且我们只在下一语句 `clone` 其
      // `parse_options`，借用不超出该表达式。
      let config: &Config = unsafe {
        let resolver = self.config_resolver_ref();
        let get_config = resolver
          .get_config
          .expect("ConfigResolver::getConfig is not set");
        &*get_config(self.config_resolver.as_ptr(), name, limits)
      };
      config.parse_options.clone()
    };
    opts.capture_comments = true;
    let mut result = self.parse_module_name_string_view_parse_options(name, &source.source, &opts);
    result.r#type = source.r#type;

    // SAFETY: result.root 指向本函数刚解析出的 AST，此处独占（C++ 契约）；
    // RequireTracer 按 cpp `AstStatBlock*` 非 const 语义取 `&mut`。
    let require = unsafe {
      trace_requires(
        self.file_resolver_mut(),
        &mut *result.root,
        name.clone(),
        limits,
      )
    };
    self.require_trace.insert(name.clone(), require.clone());

    // std::shared_ptr<SourceNode>& sourceNode = sourceNodes[name];
    // if (!sourceNode) sourceNode = std::make_shared<SourceNode>();
    let source_node_arc = self
      .source_nodes
      .entry(name.clone())
      .or_insert_with(|| Arc::new(default_source_node()));
    let source_node_ptr = arc_as_mut(source_node_arc);

    let source_module_arc = self
      .source_modules
      .entry(name.clone())
      .or_insert_with(|| Arc::new(SourceModule::new()));
    let source_module_ptr = arc_as_mut(source_module_arc);

    // Safety: `source_node_ptr`/`source_module_ptr` 是刚才经 `entry(..).or_insert_with(..)`
    // 取得、再 `arc_as_mut` 得到的 `Arc` 可变句柄，所指的 SourceNode/SourceModule 在 `&mut self`
    // 借用期内持续存活；块内对二者的写入只触及各自的 Arc 内容。内层循环对 `dep_ptr`（另一
    // map 项的 arc_as_mut 句柄）的写入指向不同节点。Frontend 全程单线程串行驱动（lib.rs 不变量），
    // 且各句柄分属不同分配，故写入与共享借用之间无并存别名，无 UB。
    unsafe {
      // *sourceModule = std::move(result);
      *source_module_ptr = result;
      (*source_module_ptr).environment_name = environment_name;

      (*source_node_ptr).name = (*source_module_ptr).name.clone();
      (*source_node_ptr).human_readable_name = (*source_module_ptr).human_readable_name.clone();

      // C++: `depIt->second->dependents.erase(sourceNode->name);`
      let prior_locations = (*source_node_ptr).require_locations.clone();
      let self_name = (*source_node_ptr).name.clone();
      for (module_name, _) in prior_locations.iter() {
        if let Some(dep) = self.source_nodes.get(module_name) {
          let dep_ptr = arc_as_mut(dep);
          (*dep_ptr).dependents.erase(&self_name);
        }
      }

      (*source_node_ptr).require_set.clear();
      (*source_node_ptr).require_locations.clear();
      (*source_node_ptr).dirty_source_module = false;

      // `it == sourceNodes.end()` in the C++ corresponds to the node not having
      // existed prior to this call (a brand-new source node).
      if !already_present {
        (*source_node_ptr).dirty_module = true;
        (*source_node_ptr).dirty_module_for_autocomplete = true;
      }

      for (module_name, _) in require.require_list.iter() {
        (*source_node_ptr).require_set.insert(module_name.clone());
      }

      (*source_node_ptr).require_locations = require.require_list.clone();
    }

    // 句柄化交付：`from_opt_ptr` 对非空入参即 `Some`，此处两指针均由上方
    // `entry(..).or_insert_with(..)` 物化、恒非空，与原裸返回逐位同构。
    (
      Handle::from_opt_ptr(source_node_ptr),
      Handle::from_opt_ptr(source_module_ptr),
    )
  }
}

fn default_source_node() -> SourceNode {
  use ulua_common::records::dense_hash_set::DenseHashSet;
  SourceNode {
    name: ModuleName::default(),
    human_readable_name: String::new(),
    require_set: DenseHashSet::new(ModuleName::default()),
    require_locations: Vec::new(),
    dependents: DenseHashSet::new(ModuleName::default()),
    dirty_source_module: true,
    dirty_module: true,
    dirty_module_for_autocomplete: true,
    invalid_module_dependency: true,
    invalid_module_dependency_for_autocomplete: true,
    autocomplete_limits_mult: 1.0,
  }
}
