use alloc::{collections::BTreeMap, string::String};

use ulua_ast::{
  enums::ast_table_access::AstTableAccess,
  records::{ast_type::AstType, ast_type_table::AstTypeTable, location::Location},
};
use ulua_common::fflag;

use crate::{
  enums::{polarity::Polarity, table_state::TableState},
  functions::{get_mutable_type, polarity_of_access::polarity_of_access},
  records::{
    constraint_generator::ConstraintGenerator, generic_error::GenericError,
    property_type::Property, scope::Scope, table_indexer::TableIndexer, table_type::TableType,
  },
  type_aliases::{name_type::Name, type_error_data::TypeErrorData, type_id::TypeId},
};
impl ConstraintGenerator {
  /// # Safety
  /// `scope` 须为调用方保活的 Arc<Scope> 堆写句柄，`tab` 须指向 parse arena 存活
  /// 的 AstTypeTable 节点（C++ `resolveTableType(NotNull<Scope*>, AstTypeTable*)`
  /// 契约，由 resolve_type_inner 的 RTTI 命中分支成立）。
  pub unsafe fn resolve_table_type(
    &mut self,
    scope: *mut Scope,
    _ty: *mut AstType,
    tab: *mut AstTypeTable,
    in_type_arguments: bool,
    _replace_error_with_fresh: bool,
  ) -> TypeId {
    let mut props: BTreeMap<Name, Property> = BTreeMap::new();
    let mut indexer: Option<TableIndexer> = None;

    let p = self.polarity;

    // Safety: tab 满足本 fn 的形参契约——调用方（resolve_type_inner）经
    // ast_node_try_as::<AstTypeTable> 命中确认其为 parse arena 存活节点
    // （repr(C) 基址重合、非空），resolve 全程 AST 只读不变；函数头一次取得
    // 共享借用，后续 props/indexer 读取均走 tab_ref。
    let tab_ref = unsafe { &*tab };
    let props_array = &tab_ref.props;

    for prop_ref in props_array.as_slice() {
      let name: String = prop_ref.name.as_str_or_empty().to_string();
      let prop_access = prop_ref.access;

      // Set the polarity for the inner type
      self.polarity = polarity_of_access(prop_access, p);
      let cur_polarity = self.polarity;

      let prop_ty = self.resolve_type(
        scope,
        prop_ref.r#type,
        in_type_arguments,
        false,
        cur_polarity,
      );

      let prop_ref_mut = props.entry(name).or_default();

      prop_ref_mut.type_location = Some(prop_ref.location);

      match prop_access {
        AstTableAccess::ReadWrite => {
          prop_ref_mut.read_ty = Some(prop_ty);
          prop_ref_mut.write_ty = Some(prop_ty);
        }
        AstTableAccess::Read => {
          prop_ref_mut.read_ty = Some(prop_ty);
        }
        AstTableAccess::Write => {
          prop_ref_mut.write_ty = Some(prop_ty);
        }
      }
    }

    if !tab_ref.indexer.is_null() {
      // Safety: tab_ref.indexer 已在上行判空——parser 保证其非空时指向存活
      // AstTypeTableIndexer arena 节点（AST 在 resolve 期不可变），此处共享
      // 借用只读取 access/location/类型子指针字段。
      let ast_indexer = unsafe { &*tab_ref.indexer };
      let indexer_access = ast_indexer.access;

      if indexer_access == AstTableAccess::Read {
        if !fflag::LuauReadOnlyIndexers.get() {
          self.report_error(
            ast_indexer.access_location.unwrap_or(Location::new(
              ast_indexer.location.begin,
              ast_indexer.location.begin,
            )),
            TypeErrorData::GenericError(GenericError::new(
              "read keyword is illegal here".to_string(),
            )),
          );
        } else {
          self.polarity = p;
          let cur_polarity = self.polarity;
          let index_ty = self.resolve_type(
            scope,
            ast_indexer.index_type,
            in_type_arguments,
            false,
            cur_polarity,
          );
          let cur_polarity2 = self.polarity;
          let result_ty = self.resolve_type(
            scope,
            ast_indexer.result_type,
            in_type_arguments,
            false,
            cur_polarity2,
          );
          indexer = Some(TableIndexer {
            index_type: index_ty,
            index_result_type: result_ty,
            is_read_only: true,
          });
        }
      } else if indexer_access == AstTableAccess::Write {
        self.report_error(
          ast_indexer.access_location.unwrap_or(Location::new(
            ast_indexer.location.begin,
            ast_indexer.location.begin,
          )),
          TypeErrorData::GenericError(GenericError::new(
            "write keyword is illegal here".to_string(),
          )),
        );
      } else if indexer_access == AstTableAccess::ReadWrite {
        self.polarity = Polarity::Mixed;
        let cur_polarity = self.polarity;
        let index_ty = self.resolve_type(
          scope,
          ast_indexer.index_type,
          in_type_arguments,
          false,
          cur_polarity,
        );
        let cur_polarity2 = self.polarity;
        let result_ty = self.resolve_type(
          scope,
          ast_indexer.result_type,
          in_type_arguments,
          false,
          cur_polarity2,
        );
        indexer = Some(TableIndexer {
          index_type: index_ty,
          index_result_type: result_ty,
          is_read_only: false,
        });
      }
      // else: Unexpected property access - handled by C++ ice
    }

    self.polarity = p;

    let table_ty = unsafe {
      // Safety: scope 是调用方按本 fn 形参契约以 arc_as_mut(&ScopePtr) 派生的
      // Arc<Scope> 堆写句柄（非空、对齐、随调用栈帧存活），level 为 Copy 只读；
      // self.arena.as_ptr() 为构造期接线的非空会话类型 arena 裸指针，add_type 只在 bump
      // 块尾追加新节点（块地址不移动），tt 全部由上方本地 props/indexer 构造，
      // 借用区间内无其它可变访问。
      let tt = TableType::table_type_props_optional_table_indexer_type_level_scope_table_state(
        &props,
        indexer,
        (*scope).level,
        scope,
        TableState::Sealed,
      );

      self.arena.get_mut().add_type(tt)
    };

    // table_ty 刚由 add_type(TableType) 分配，必命中；对照 C++:4673-4676
    // `TableType* ttv = getMutable<TableType>(tableTy); ttv->definition...`
    let ttv = get_mutable_type::get_mutable::<TableType>(table_ty)
      .expect("table_ty 刚由 add_type(TableType) 分配，必命中（cpp:4673）");
    // Safety: ttv 取自上方刚 add_type 的 TableType——RTTI 命中且 unwrap 保证
    // 非空，节点驻留 self.arena.as_ptr()、尚无其它活动借用，两处字段写为唯一写窗口；
    // tab 为调用方 RTTI 确认的 parse arena 存活节点（顶部经 tab_ref 只读借用，
    // 与对 ttv 的可变访问分属 AST 与类型 arena 两个对象），此处仅读 base 链
    // 上的 location。
    unsafe {
      ttv.definition_module_name = self
        .module
        .as_ref()
        .expect("ConstraintGenerator 构造期以 ModulePtr 接线并 LUAU_ASSERT(is_some)，恒为 Some")
        .name
        .clone();
      ttv.definition_location = (*tab).base.base.location;
    }

    table_ty
  }
}
