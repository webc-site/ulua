use alloc::{string::String, vec::Vec};

use ulua_ast::records::location::Location;

use crate::{
  enums::scope_type::ScopeType,
  records::{
    cell::Cell,
    data_flow_graph_builder::DataFlowGraphBuilder,
    def_registry::{def_as, def_ref},
    dfg_scope::DfgScope,
    phi::Phi,
    symbol::Symbol,
  },
  type_aliases::def_id_def::DefId,
};

/// 存活句柄的符号名（cpp `def->getName()`）；空/异线程句柄属契约违例。
fn def_name(def: DefId) -> Symbol {
  def_ref(def).expect("Def arena 存活句柄契约").name.clone()
}

impl DataFlowGraphBuilder {
  pub fn lookup_symbol_location(&mut self, symbol: Symbol, location: Location) -> DefId {
    let scope = self.current_scope();

    let mut outside_loop_scope = false;
    let mut current: *mut DfgScope = scope;
    while !current.is_null() {
      // Safety: `current` 非空由 `while !current.is_null()` 守卫，指向 arena 存活的
      // `DfgScope` 节点；`scope = self.current_scope()` 断言 scope_stack 非空并返回其
      // 栈顶元素，该元素始终是 `newScope` 产出的非空 arena 指针；`self.def_arena` 构造
      // 时接线、非空，向其新增 def 不移动既有节点；写 `(*scope).bindings` 为单线程串行
      // 独占，与当前读无别名冲突。
      unsafe {
        outside_loop_scope = outside_loop_scope || (*current).scope_type == ScopeType::Loop;

        if let Some(found) = (*current).bindings.find(&symbol) {
          return *found;
        } else if (*current).scope_type == ScopeType::Function {
          let capture = self.captures.get_or_insert(symbol.clone());
          let capture_def = self.def_arena.get_mut().phi_vector_def_id(&Vec::new());
          capture.capture_defs.push(capture_def);

          if !outside_loop_scope {
            *(*scope).bindings.get_or_insert(symbol.clone()) = capture_def;
          }

          return capture_def;
        }
      }

      // Safety: `current` 仍非空（同 while 守卫），此处仅读取其 `parent` 字段
      // （`*mut DfgScope`，可为 null，下一轮循环判空），为存活节点的共享只读访问。
      unsafe {
        current = (*current).parent;
      }
    }

    // Safety: `self.def_arena` 构造接线、非空且块地址稳定；`scope` 为 current_scope
    // 顶元素的非空存活 `DfgScope`；写 `(*scope).bindings` 单线程独占，无别名冲突。
    unsafe {
      let result = self
        .def_arena
        .get_mut()
        .fresh_cell(symbol.clone(), location, false);
      *(*scope).bindings.get_or_insert(symbol.clone()) = result;
      self
        .captures
        .get_or_insert(symbol)
        .all_versions
        .push(result);
      result
    }
  }

  /// `DefId DataFlowGraphBuilder::lookup(DefId def, const std::string& key, Location location)`.
  /// Reference: `DataFlowGraph.cpp:328-364`.
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn lookup_def_id_string_location(
    &mut self,
    def: DefId,
    key: &String,
    location: Location,
  ) -> DefId {
    let scope = self.current_scope();

    let mut current: *mut DfgScope = scope;
    while !current.is_null() {
      // Safety: `current` 非空由 `while !current.is_null()` 守卫，为 arena 存活的
      // `DfgScope`；`def` 是注册表句柄，def 节点读取全部经安全的 `def_as`；
      // `self.def_arena`、`scope`（current_scope 顶元素）均构造接线非空，arena 新增
      // 不移动既有节点，写 `(*scope).props` 单线程独占无别名冲突。
      unsafe {
        if let Some(props) = (*current).props.find(&def) {
          if let Some(found) = props.get(key) {
            return *found;
          }
        } else if let Some(phi) = def_as::<Phi>(def)
          && phi.operands.is_empty()
          && (*current).scope_type == ScopeType::Function
        {
          let result = self
            .def_arena
            .get_mut()
            .fresh_cell(def_name(def), location, false);
          (*scope)
            .props
            .get_or_insert(def)
            .insert(key.clone(), result);
          return result;
        }

        current = (*current).parent;
      }
    }

    // `def` 是注册表句柄，节点探测经安全的 `def_as` 下转；`self.def_arena`、
    // `scope` 构造接线非空、块地址稳定；递归传入的 operand 仍是存活合法句柄；
    // 兜底分支的 `self.handle` 由 build() 接线非空（C++ `NotNull` 形参契约）。
    if let Some(phi) = def_as::<Phi>(def) {
      let mut defs = Vec::new();
      for operand in &phi.operands {
        // Safety: 递归沿用本函数同一契约（def 为存活句柄、scope 接线非空）。
        defs.push(unsafe { self.lookup_def_id_string_location(*operand, key, location) });
      }

      let result = self.def_arena.get_mut().phi_vector_def_id(&defs);
      unsafe {
        (*scope)
          .props
          .get_or_insert(def)
          .insert(key.clone(), result)
      };
      return result;
    }

    if def_as::<Cell>(def).is_some() {
      let result = self
        .def_arena
        .get_mut()
        .fresh_cell(def_name(def), location, false);
      unsafe {
        (*scope)
          .props
          .get_or_insert(def)
          .insert(key.clone(), result)
      };
      return result;
    }

    self
      .handle
      .expect("DataFlowGraphBuilder::handle 须由 build() 接线非空")
      .get()
      .ice_string("Inexhaustive lookup cases in DataFlowGraphBuilder::lookup");
    unreachable!()
  }
}
