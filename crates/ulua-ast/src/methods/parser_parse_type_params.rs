use core::ptr::{NonNull, from_ref};

use crate::{
  enums::type_lexer::Type,
  functions::{
    is_type_follow::is_type_follow,
    optional_node::{node_opt, opt_node, slot_opt, slot_ref},
    should_parse_type_pack::should_parse_type_pack,
  },
  records::{
    ast_array::AstArray, ast_node::AstNode, ast_type_group::AstTypeGroup,
    ast_type_or_pack::AstTypeOrPack, ast_type_pack_explicit::AstTypePackExplicit,
    cst_type_group::CstTypeGroup, cst_type_pack_explicit::CstTypePackExplicit,
    match_lexeme::MatchLexeme, node_handle::Node, parser::Parser, position::Position,
    temp_vector::TempVector,
  },
  rtti::{ast_node_try_as, cst_node_try_as},
};

impl Parser {
  pub fn parse_type_params(
    &mut self,
    opening_position: Option<&mut Position>,
    mut comma_positions: Option<&mut TempVector<'_, Position>>,
    closing_position: Option<&mut Position>,
  ) -> AstArray<AstTypeOrPack> {
    let mut parameters: Vec<AstTypeOrPack> = Vec::new();

    if self.lexer.current().r#type == Type::LESS {
      let begin = *self.lexer.current();
      if let Some(pos) = opening_position {
        *pos = begin.location.begin;
      }
      self.next_lexeme();

      loop {
        if should_parse_type_pack(&mut self.lexer) {
          parameters.push(AstTypeOrPack::from_type_pack(opt_node(
            self.parse_type_pack(),
          )));
        } else if self.lexer.current().r#type == Type::LPAREN {
          let begin_loc = self.lexer.current().location;
          let c = self.lexer.current().r#type;

          // cpp 用两个 null 局部量暂存「尚未解析出前导类型」（以 `|`/`&` 开头的联合/交叉），
          // 此处以 `Error` 变体表达同一中间态，再按 `as_pack()/as_type()` 取用。
          let type_or_pack = if c != Type::PIPE && c != Type::AMPERSAND {
            self.parse_simple_type(true, false)
          } else {
            AstTypeOrPack::Error
          };

          if let Some(type_pack) = type_or_pack.as_pack() {
            // `type_pack` 由 parse_simple_type 的 arena 分配产出（见 `AstTypeOrPack`
            // 构造契约）；ast_node_try_as 安全门面依 class_index 判型，未命中返回
            // None，命中即 repr(C) 基址重合的引用下转。
            let explicit_type_pack = ast_node_try_as::<AstTypePackExplicit>(type_pack);
            if let Some(explicit_type_pack) = explicit_type_pack
              && node_opt(explicit_type_pack.type_list.tail_type).is_none()
              && explicit_type_pack.type_list.types.len() == 1
              && is_type_follow(self.lexer.current().r#type)
            {
              // `types.size == 1` 已由上一条件验证：as_slice() 给出长度 1 的合法切片，[0] 不越界。
              let parenthesized_type = explicit_type_pack.type_list.types.as_slice()[0];

              // cpp 无 LuauCstTypeGroup flag：AstTypeGroup 无条件创建，
              // CstTypeGroup 仅受 storeCstData 门控
              let type_group = self.alloc_type(AstTypeGroup::new(
                // parenthesized_type 是 types 切片首元素，parser 建列时写入的
                // 非空 arena 存活 `*mut AstType`；slot_ref 只读其基类 location。
                slot_ref(parenthesized_type).base.location,
                Node::from_raw(parenthesized_type),
              ));

              if self.options.store_cst_data
                && let Some(cst_node) = self
                  .cst_node_map
                  .find(&from_ref(explicit_type_pack).cast::<AstNode>().cast_mut())
                  .copied()
                // map 值是 parser 登记的 CST 节点指针——null（未登记槽位）或 arena
                // 存活节点；slot_opt 判空折叠为 None（等价旧 `!cst_node.is_null()` 守卫）。
                && let Some(cst_node) = slot_opt(cst_node)
                // CST 下转走 safe 门面：class_index 命中才 Some，未命中折叠为 None
                // （等价旧 `cst_node_as` 判空），命中即 repr(C) 基址重合的正确下转。
                && let Some(cst_explicit_type_pack) =
                  cst_node_try_as::<CstTypePackExplicit>(cst_node)
              {
                let close_pos = cst_explicit_type_pack.close_parentheses_position;
                self.attach_cst(type_group, |alloc| {
                  alloc.alloc(CstTypeGroup::new(close_pos))
                });
              }

              parameters.push(AstTypeOrPack::from_type(
                self.parse_type_suffix(NonNull::new(type_group), &begin_loc),
              ));
            } else {
              parameters.push(AstTypeOrPack::Pack(type_pack));
            }
          } else {
            parameters.push(AstTypeOrPack::from_type(
              self.parse_type_suffix(type_or_pack.as_type().map(NonNull::from), &begin_loc),
            ));
          }
        } else if self.lexer.current().r#type == Type::GREATER && parameters.is_empty() {
          break;
        } else {
          parameters.push(AstTypeOrPack::from_type(self.parse_type(false)));
        }

        if self.lexer.current().r#type == Type::COMMA {
          if let Some(vec) = comma_positions.as_deref_mut() {
            vec.push_back(self.lexer.current().location.begin);
          }
          self.next_lexeme();
        } else {
          break;
        }
      }

      let closing_bracket_found =
        self.expect_match_and_consume('>', &MatchLexeme::new(&begin), false);
      if let Some(pos) = closing_position
        && closing_bracket_found
      {
        *pos = self.lexer.previous_location().begin;
      }
    }

    self.copy_initializer_list_t(&parameters)
  }
}
