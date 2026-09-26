use alloc::vec::Vec;

use ulua_ast::records::{ast_expr::AstExpr, ast_expr_binary::AstExprBinaryOp, location::Location};
use ulua_common::macros::luau_unreachable::LUAU_UNREACHABLE;

use crate::{
  records::{
    constraint_generator::ConstraintGenerator, equality_constraint::EqualityConstraint,
    inference::Inference,
  },
  type_aliases::{
    constraint_v::ConstraintV, refinement_id_refinement::NULL_REFINEMENT_ID,
    scope_ptr_type::ScopePtr, type_id::TypeId,
  },
};

impl ConstraintGenerator {
  /// 对应 cpp `ConstraintGenerator::checkAstExprBinary`
  /// （`Analysis/src/ConstraintGenerator.cpp:3537-3600`）。
  ///
  /// 本函数自身是 safe 边界，但 `left`/`right` 沿用 C++ `AstExpr*` 子节点句柄
  /// 约定：二者必须是刚经 RTTI 分发层确认的非空、存活 `AstExpr` 节点指针
  /// （来自所属模块 AST arena，check 期间节点地址稳定且无人改写），本函数仅
  /// 把它们转发给 unsafe fn `check_binary` 的契约；现存调用点（check 分发器与
  /// compound-assign visitor）都满足该前提。
  pub(crate) fn check_ast_expr_binary(
    &mut self,
    scope: &ScopePtr,
    location: Location,
    op: AstExprBinaryOp,
    left: *mut AstExpr,
    right: *mut AstExpr,
    expected_type: Option<TypeId>,
  ) -> Inference {
    // Safety: `check_binary` 为 unsafe fn，其契约即上文的 left/right 存活 AST
    // 节点前提——两指针由分发层按 AstExprBinary 子字段原样传入（cpp `check` 里
    // 递归 binary->left/right 的同一对象），调用期间仅作只读遍历，改写发生在
    // generator 自身状态而非 AST 节点。
    let (left_type, right_type, refinement) =
      unsafe { self.check_binary(scope, op, left, right, expected_type) };
    // §2：`check_binary` 以 `Option` 表达「无 refinement」（原 null 哨兵）；
    // `Inference.refinement` 是直存可空句柄的数据槽，在此以定义处收口的具名
    // 哨兵落槽，值面与原 cpp 逐位同构。
    let refinement = refinement.unwrap_or(NULL_REFINEMENT_ID);

    match op {
      AstExprBinaryOp::Add => {
        let result_type = self.create_type_function_instance(
          // Safety: `builtin_types` 为构造期 NotNull 语义布线的独立分配，全程存活
          // 且 `type_functions` Box 内容装载后不再改写；这里只读出 `add_func` 描述符
          // 的共享引用（+ 号语义，cpp addFunc），与下面 `&mut self` 的 arena 写入
          // 分属不同分配，引用不占 self 借用图。
          { &self.builtin_types.get().type_functions.add_func },
          Vec::from([left_type, right_type]),
          Vec::new(),
          scope,
          location,
        );
        Inference::inference_type_id_refinement_id(result_type, refinement)
      }
      AstExprBinaryOp::Sub => {
        let result_type = self.create_type_function_instance(
          // Safety: 同 NotNull 构造不变量下的只读取描述符：本次借用 `-` 运算的
          // sub_func 表项，源指针恒指存活 BuiltinTypes，callee 不写回该分配。
          { &self.builtin_types.get().type_functions.sub_func },
          Vec::from([left_type, right_type]),
          Vec::new(),
          scope,
          location,
        );
        Inference::inference_type_id_refinement_id(result_type, refinement)
      }
      AstExprBinaryOp::Mul => {
        let result_type = self.create_type_function_instance(
          // Safety: `*` 分支取 `mul_func` 静态表项；引用派生自裸指针解引用故需
          // unsafe，其有效性由 builtin_types 随前端上下文存活的构造契约保证。
          { &self.builtin_types.get().type_functions.mul_func },
          Vec::from([left_type, right_type]),
          Vec::new(),
          scope,
          location,
        );
        Inference::inference_type_id_refinement_id(result_type, refinement)
      }
      AstExprBinaryOp::Div => {
        let result_type = self.create_type_function_instance(
          // Safety: `/` 分支读 `div_func` 描述符；该 Box 表项构造后不可变，
          // 只读借用与本次 &mut self 互不交叠。
          { &self.builtin_types.get().type_functions.div_func },
          Vec::from([left_type, right_type]),
          Vec::new(),
          scope,
          location,
        );
        Inference::inference_type_id_refinement_id(result_type, refinement)
      }
      AstExprBinaryOp::FloorDiv => {
        let result_type = self.create_type_function_instance(
          // Safety: `//` 分支借用 `idiv_func` 表项（cpp idivFunc），指针来源同
          // 上文 NotNull 字段，无别名写方。
          { &self.builtin_types.get().type_functions.idiv_func },
          Vec::from([left_type, right_type]),
          Vec::new(),
          scope,
          location,
        );
        Inference::inference_type_id_refinement_id(result_type, refinement)
      }
      AstExprBinaryOp::Pow => {
        let result_type = self.create_type_function_instance(
          // Safety: `^` 分支借用 `pow_func`；解引用的是构造期写入且检查期间
          // 恒有效的 builtin_types 句柄，取出的引用只读描述符本身。
          { &self.builtin_types.get().type_functions.pow_func },
          Vec::from([left_type, right_type]),
          Vec::new(),
          scope,
          location,
        );
        Inference::inference_type_id_refinement_id(result_type, refinement)
      }
      AstExprBinaryOp::Mod => {
        let result_type = self.create_type_function_instance(
          // Safety: `%` 分支只读 `mod_func` 静态定义；引用生命周期锚定在
          // BuiltinTypes 分配（随前端存活），不借用 generator 状态。
          { &self.builtin_types.get().type_functions.mod_func },
          Vec::from([left_type, right_type]),
          Vec::new(),
          scope,
          location,
        );
        Inference::inference_type_id_refinement_id(result_type, refinement)
      }
      AstExprBinaryOp::Concat => {
        let result_type = self.create_type_function_instance(
          // Safety: `..` 拼接分支借用 `concat_func` 表项，cpp concatFunc 同名
          // 入口；裸指针有效性由构造契约保证，读取不产生写冲突。
          { &self.builtin_types.get().type_functions.concat_func },
          Vec::from([left_type, right_type]),
          Vec::new(),
          scope,
          location,
        );
        Inference::inference_type_id_refinement_id(result_type, refinement)
      }
      AstExprBinaryOp::And => {
        let result_type = self.create_type_function_instance(
          // Safety: `and` 分支借用 `and_func`；与前面各表项同一 NotNull 来源，
          // create_type_function_instance 仅把该描述符拷入新 ApplyTypeFunction 节点。
          { &self.builtin_types.get().type_functions.and_func },
          Vec::from([left_type, right_type]),
          Vec::new(),
          scope,
          location,
        );
        Inference::inference_type_id_refinement_id(result_type, refinement)
      }
      AstExprBinaryOp::Or => {
        let result_type = self.create_type_function_instance(
          // Safety: `or` 分支借用 `or_func` 表项；只读访问存活的 BuiltinTypes
          // 分配，不引入对 self 的可变借用冲突。
          { &self.builtin_types.get().type_functions.or_func },
          Vec::from([left_type, right_type]),
          Vec::new(),
          scope,
          location,
        );
        Inference::inference_type_id_refinement_id(result_type, refinement)
      }
      AstExprBinaryOp::CompareLt
      | AstExprBinaryOp::CompareGe
      | AstExprBinaryOp::CompareLe
      | AstExprBinaryOp::CompareGt => {
        self.add_constraint_scope_ptr_location_constraint_v(
          scope,
          location,
          ConstraintV::Equality(EqualityConstraint {
            result_type: left_type,
            assignment_type: right_type,
          }),
        );
        Inference::inference_type_id_refinement_id(
          // Safety: 比较族（<、>=、<=、>）的返回类型是内建 boolean 句柄——从
          // NotNull 存活的 builtin_types 分配 Copy 出一个 TypeId，随即脱离该分配。
          { self.builtin_types.get().boolean_type },
          refinement,
        )
      }
      AstExprBinaryOp::CompareEq | AstExprBinaryOp::CompareNe => {
        Inference::inference_type_id_refinement_id(
          // Safety: `==`/`~=` 同样直接返回构造期缓存的 boolean TypeId（Copy 值，
          // cpp `Inference{builtinTypes->booleanType, ...}`），解引用有契约保证。
          { self.builtin_types.get().boolean_type },
          refinement,
        )
      }
      AstExprBinaryOp::OpCount => {
        // Safety: `ice` 是构造期注入、等价 C++ `NotNull<InternalErrorReporter>`
        // 的存活报告器指针；OpCount 不是合法 AST 运算符，此分支按 cpp 原样
        // 上报 ICE 后即落入 LUAU_UNREACHABLE!（对应 `ice->ice(...)`）。
        self
          .ice
          .get()
          .ice_string("OpCount should never be generated in an AST.");
        LUAU_UNREACHABLE!()
      }
    }
  }
}
