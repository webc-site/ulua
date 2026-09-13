pub mod ambiguous_function_call;
pub mod and_predicate;
pub mod annotation_types_at_location;
pub mod any_type;
pub mod anyification;
pub mod apply_mapped_generics;
pub mod apply_type_function;
pub mod arc_collector;
pub mod arcs;
pub mod are_equal_state;
pub mod array_emitter;
pub mod assign;
pub mod assign_index_constraint;
pub mod assign_prop_constraint;
pub mod ast_array_predicate;
pub mod ast_expr_table_finder;
pub mod ast_json_encoder;
pub mod autocomplete_entry;
pub mod autocomplete_node_finder;
pub mod autocomplete_result;
pub mod bidirectional_type_pusher;
pub mod binding;
pub mod binding_snapshot;
pub mod block;
pub mod block_scope;
pub mod blocked_type;
pub mod blocked_type_finder;
pub mod blocked_type_in_literal_visitor;
pub mod blocked_type_pack;
pub mod boolean_singleton;
pub mod bound;
pub mod boundary_snapshot;
pub mod build_queue_item;
pub mod build_queue_work_state;
pub mod built_in_type_function_error;
pub mod builtin_type_functions;
pub mod builtin_types;
pub mod cannot_assign_to_never;
pub mod cannot_call_non_function;
pub mod cannot_check_dynamic_string_format_calls;
pub mod cannot_compare_unrelated_types;
pub mod cannot_extend_table;
pub mod cannot_infer_binary_operation;
pub mod cell;
pub mod cfg_allocator;
pub mod cfg_builder;
pub mod check_result;
pub mod checked_function_call_error;
pub mod checked_function_incorrect_args;
pub mod checkpoint;
pub mod class_decl_record;
pub mod class_user_data;
pub mod clone_public_interface;
pub mod clone_state;
pub mod code_too_complex;
pub mod config_resolver;
pub mod conjunction_control_flow_graph;
pub mod conjunction_refinement;
pub mod const_iterator;
pub mod constraint;
pub mod constraint_block;
pub mod constraint_generation_log;
pub mod constraint_generator;
pub mod constraint_graph;
pub mod constraint_list;
pub mod constraint_set;
pub mod constraint_snapshot;
pub mod constraint_solver;
pub mod constraint_solving_incomplete_error;
pub mod constraint_step_snapshot;
pub mod contains_any_generic_deprecated;
pub mod contains_function_call;
pub mod contains_generics;
pub mod contains_refinable_type;
pub mod control_flow_graph;
pub mod count_mismatch;
pub mod counter_state;
pub mod data_flow_graph;
pub mod data_flow_graph_builder;
pub mod data_flow_result;
pub mod dcr_logger;
pub mod declare;
pub mod def;
pub mod def_arena;
pub mod demoter;
pub mod deprecated_api_used;
pub mod dfg_scope;
pub mod disjunction_control_flow_graph;
pub mod disjunction_refinement;
pub mod duplicate_generic_parameter;
pub mod duplicate_type_definition;
pub mod dynamic_property_lookup_on_extern_types_unsafe;
pub mod element_result;
pub mod eq_predicate;
pub mod equality_constraint;
pub mod equivalence;
pub mod error;
pub mod error_converter;
pub mod error_snapshot;
pub mod error_suppression;
pub mod expected_type_visitor;
pub mod explicit_function_annotation_recommended;
pub mod expr_or_local;
pub mod expr_printer;
pub mod expr_types_at_location;
pub mod extern_type;
pub mod extra_information;
pub mod failed_to_compile;
pub mod field;
pub mod file_resolver;
pub mod find_all_union_members;
pub mod find_cyclic_types;
pub mod find_expr_or_local;
pub mod find_full_ancestry;
pub mod find_function_type_in;
pub mod find_node;
pub mod find_refinement_blockers;
pub mod find_simplification_blockers;
pub mod find_user_type_function_blockers;
pub mod fragment_autocomplete_ancestry_result;
pub mod fragment_autocomplete_result;
pub mod fragment_autocomplete_status_result;
pub mod fragment_autocomplete_type_cloner;
pub mod fragment_context;
pub mod fragment_parse_result;
pub mod fragment_region;
pub mod fragment_type_check_result;
pub mod free_type;
pub mod free_type_pack;
pub mod free_type_searcher;
pub mod freeze_type_function_types;
pub mod frontend;
pub mod frontend_cancellation_token;
pub mod frontend_module_resolver;
pub mod frontend_options;
pub mod fuel_initializer;
pub mod function_argument;
pub mod function_call_constraint;
pub mod function_capture;
pub mod function_check_constraint;
pub mod function_definition;
pub mod function_does_not_take_self;
pub mod function_exits_without_returning;
pub mod function_graph_reduction_result;
pub mod function_info;
pub mod function_parameter_documentation;
pub mod function_requires_self;
pub mod function_signature;
pub mod function_type;
pub mod generalization_constraint;
pub mod generalization_params;
pub mod generalization_result;
pub mod generalize_step_snapshot;
pub mod generic_bounds;
pub mod generic_bounds_mismatch;
pub mod generic_counter;
pub mod generic_error;
pub mod generic_pack_mapping;
pub mod generic_type;
pub mod generic_type_count_mismatch;
pub mod generic_type_definition;
pub mod generic_type_definitions;
pub mod generic_type_finder;
pub mod generic_type_pack;
pub mod generic_type_pack_count_mismatch;
pub mod generic_type_pack_definition;
pub mod generic_type_visitor;
pub mod global_linter;
pub mod global_linter_alt_b;
pub mod global_linter_alt_c;
pub mod global_name_collector;
pub mod global_prepopulator;
pub mod global_types;
pub mod has_free_type;
pub mod has_indexer_constraint;
pub mod has_prop_constraint;
pub mod hash_blocked_constraint_id;
pub mod hash_bool_name_pair;
pub mod hash_instantiation_signature;
pub mod hash_subtype_constraint_record;
pub mod hold_conditional_execution;
pub mod i_fragment_autocomplete_reporter;
pub mod identifier;
pub mod identifier_hash;
pub mod illegal_require;
pub mod in_conditional_context;
pub mod incomplete_inference;
pub mod incorrect_generic_parameter_count;
pub mod index;
pub mod index_collector;
pub mod indexer_index_collector;
pub mod inference;
pub mod inference_pack;
pub mod infinite_type_finder;
pub mod inplace_demoter;
pub mod instance_collector;
pub mod instance_collector_2;
pub mod instantiate_generics_on_non_function;
pub mod instantiation;
pub mod instantiation_2;
pub mod instantiation_queuer;
pub mod instantiation_queuer_deprecated;
pub mod instantiation_signature;
pub mod interior_free_types;
pub mod internal_compiler_error;
pub mod internal_error;
pub mod internal_error_reporter;
pub mod internal_type_finder;
pub mod internal_type_function_finder;
pub mod intersection_builder;
pub mod intersection_type;
pub mod invalid_name_checker;
pub mod is_a_predicate;
pub mod iterable_constraint;
pub mod iterative_type_function_type_visitor;
pub mod iterative_type_visitor;
pub mod iterator;
pub mod join;
pub mod json_emitter;
pub mod klass;
pub mod l_value_hasher;
pub mod lazy_type;
pub mod lint_comparison_precedence;
pub mod lint_context;
pub mod lint_deprecated_api;
pub mod lint_duplicate_condition;
pub mod lint_duplicate_function;
pub mod lint_duplicate_local;
pub mod lint_for_range;
pub mod lint_format_string;
pub mod lint_global_local;
pub mod lint_implicit_return;
pub mod lint_integer_parsing;
pub mod lint_local_hygiene;
pub mod lint_misleading_and_or;
pub mod lint_multi_line_statement;
pub mod lint_redundant_native_attribute;
pub mod lint_result;
pub mod lint_same_line_statement;
pub mod lint_table_literal;
pub mod lint_table_operations;
pub mod lint_unbalanced_assignment;
pub mod lint_uninitialized_local;
pub mod lint_unknown_type;
pub mod lint_unreachable_code;
pub mod lint_unused_function;
pub mod load_definition_file_result;
pub mod local_linter;
pub mod luau_temp_thread_popper;
pub mod magic_find;
pub mod magic_function;
pub mod magic_function_call_context;
pub mod magic_function_type_check_context;
pub mod magic_gmatch;
pub mod magic_refinement_context;
pub mod mapped_generic_environment;
pub mod mapped_generic_frame;
pub mod metatable_type;
pub mod missing_properties;
pub mod missing_union_property;
pub mod module;
pub mod module_has_cyclic_dependency;
pub mod module_info;
pub mod module_resolver;
pub mod multiple_nonviable_overloads;
pub mod name_constraint;
pub mod native_stack_guard;
pub mod nearest_likely_block_finder;
pub mod nearest_statement_finder;
pub mod negation_control_flow_graph;
pub mod negation_refinement;
pub mod negation_type;
pub mod negation_type_finder;
pub mod never_type;
pub mod no_refine_type;
pub mod node;
pub mod non_exceptional_recursion_limiter;
pub mod non_strict_context;
pub mod non_strict_function_definition_error;
pub mod non_strict_type_checker;
pub mod normalization_too_complex;
pub mod normalized_extern_type;
pub mod normalized_function_type;
pub mod normalized_string_type;
pub mod normalized_type;
pub mod normalizer;
pub mod normalizer_hit_limits;
pub mod not_a_table;
pub mod not_bindable;
pub mod not_null;
pub mod not_predicate;
pub mod nothing;
pub mod null_file_resolver;
pub mod null_module_resolver;
pub mod obj;
pub mod object_emitter;
pub mod occurs_check_failed;
pub mod only_tables_can_have_methods;
pub mod optional_value_access;
pub mod or_predicate;
pub mod ordered_map;
pub mod overload_error_entry;
pub mod overload_resolution;
pub mod overload_resolver;
pub mod pack_slice;
pub mod pack_subtype_constraint;
pub mod pack_where_clause_needed;
pub mod path;
pub mod path_builder;
pub mod path_hash;
pub mod pending_expansion_type;
pub mod pending_type;
pub mod pending_type_pack;
pub mod phi;
pub mod pinned_storage;
pub mod primitive_type;
pub mod primitive_type_constraint;
pub mod promote_type_levels;
pub mod property_access_violation;
pub mod property_type;
pub mod property_type_path;
pub mod proposition_control_flow_graph;
pub mod proposition_refinement;
pub mod push_function_type_constraint;
pub mod push_scope;
pub mod push_type_constraint;
pub mod push_type_result;
pub mod quantifier;
pub mod reasonings;
pub mod recursion_counter;
pub mod recursion_limit_exception;
pub mod recursion_limiter;
pub mod recursive_restraint_violation;
pub mod reduce_constraint;
pub mod reduce_pack_constraint;
pub mod reduction;
pub mod reference_count_initializer;
pub mod refine;
pub mod refine_type_scrubber;
pub mod refinement_arena_control_flow_graph;
pub mod refinement_arena_refinement;
pub mod refinement_key;
pub mod refinement_key_arena;
pub mod refinement_partition;
pub mod replace_generics;
pub mod replacer;
pub mod require_alias;
pub mod require_cycle;
pub mod require_node;
pub mod require_suggester;
pub mod require_suggestion;
pub mod require_trace_result;
pub mod require_tracer;
pub mod reserved_identifier;
pub mod resetter;
pub mod result;
pub mod runtime_error;
pub mod scope;
pub mod scope_snapshot;
pub mod scoped_assign;
pub mod scoped_exit;
pub mod selected_overload;
pub mod serialized_function_scope;
pub mod serialized_generic;
pub mod set;
pub mod simplify_constraint;
pub mod simplify_result;
pub mod singleton_type;
pub mod skip_cache_for_type;
pub mod source_code;
pub mod source_module;
pub mod source_node;
pub mod stack_pusher_non_strict_type_checker;
pub mod stack_pusher_type_checker_2;
pub mod state_dot;
pub mod statement;
pub mod stats;
pub mod string_singleton;
pub mod stringifier_state;
pub mod substitution;
pub mod subtype_constraint;
pub mod subtype_constraint_record;
pub mod subtyping;
pub mod subtyping_environment;
pub mod subtyping_reasoning;
pub mod subtyping_reasoning_hash;
pub mod subtyping_result;
pub mod subtyping_unifier;
pub mod swapped_generic_type_parameter;
pub mod sym_def;
pub mod symbol;
pub mod syntax_error;
pub mod table_indexer;
pub mod table_prop_lookup_result;
pub mod table_type;
pub mod tarjan;
pub mod tarjan_node;
pub mod tarjan_worklist_vertex;
pub mod time_limit_error;
pub mod to_dot_options;
pub mod to_string_name_map;
pub mod to_string_options;
pub mod to_string_result;
pub mod to_string_span;
pub mod traversal_state;
pub mod truthy_predicate;
pub mod try_pair;
pub mod txn_log;
pub mod r#type;
pub mod type_alias_expansion_constraint;
pub mod type_arena;
pub mod type_attacher;
pub mod type_binding_snapshot;
pub mod type_cacher;
pub mod type_check_limits;
pub mod type_check_log;
pub mod type_checker;
pub mod type_checker_2;
pub mod type_cloner;
pub mod type_error;
pub mod type_error_summary;
pub mod type_error_to_string_options;
pub mod type_fun;
pub mod type_function;
pub mod type_function_any_type;
pub mod type_function_boolean_singleton;
pub mod type_function_cloner;
pub mod type_function_context;
pub mod type_function_deserializer;
pub mod type_function_error;
pub mod type_function_error_converter;
pub mod type_function_extern_type;
pub mod type_function_finder;
pub mod type_function_function_type;
pub mod type_function_generic_type;
pub mod type_function_generic_type_pack;
pub mod type_function_inference_result;
pub mod type_function_instance_type;
pub mod type_function_instance_type_pack;
pub mod type_function_intersection_type;
pub mod type_function_missing;
pub mod type_function_negation_type;
pub mod type_function_never_type;
pub mod type_function_primitive_type;
pub mod type_function_property;
pub mod type_function_reducer;
pub mod type_function_reduction_guess_result;
pub mod type_function_reduction_guesser;
pub mod type_function_reduction_result;
pub mod type_function_runtime;
pub mod type_function_runtime_builder_state;
pub mod type_function_serializer;
pub mod type_function_singleton_type;
pub mod type_function_string_singleton;
pub mod type_function_table_indexer;
pub mod type_function_table_type;
pub mod type_function_type;
pub mod type_function_type_pack;
pub mod type_function_type_pack_var;
pub mod type_function_union_type;
pub mod type_function_unknown_type;
pub mod type_function_variadic_type_pack;
pub mod type_guard;
pub mod type_guard_predicate;
pub mod type_id_pair_hash;
pub mod type_ids;
pub mod type_instantiation_constraint;
pub mod type_instantiation_count_mismatch;
pub mod type_iterator;
pub mod type_level;
pub mod type_mismatch;
pub mod type_once_visitor;
pub mod type_pack;
pub mod type_pack_function;
pub mod type_pack_iterator;
pub mod type_pack_mismatch;
pub mod type_pack_rehydration_visitor;
pub mod type_pack_stringifier;
pub mod type_pack_var;
pub mod type_pair_hash;
pub mod type_reduction_reentrancy_guard;
pub mod type_rehydration_options;
pub mod type_rehydration_visitor;
pub mod type_remover;
pub mod type_searcher;
pub mod type_simplifier;
pub mod type_solve_log;
pub mod type_stringifier;
pub mod type_visitor;
pub mod typed_allocator;
pub mod types_are_unrelated;
pub mod unapplied_type_function;
pub mod unblocked_types;
pub mod unexpected_array_like_table_item;
pub mod unexpected_type_in_subtyping;
pub mod unexpected_type_pack_in_subtyping;
pub mod unifiable;
pub mod unification_too_complex;
pub mod unifier;
pub mod unifier_2;
pub mod unifier_counters;
pub mod unifier_options;
pub mod unifier_shared_state;
pub mod uninhabited_type_function;
pub mod uninhabited_type_pack_function;
pub mod union_builder;
pub mod union_type;
pub mod unknown_prop_but_found_like_prop;
pub mod unknown_property;
pub mod unknown_require;
pub mod unknown_symbol;
pub mod unknown_type;
pub mod unmapped;
pub mod unpack_constraint;
pub mod unscoped_generic_finder;
pub mod unsupported_type;
pub mod unsupported_type_pack;
pub mod usage_finder;
pub mod user_cancel_error;
pub mod user_defined_function_data;
pub mod user_defined_type_function_error;
pub mod variadic;
pub mod variadic_type_pack;
pub mod variant;
pub mod visitor;
pub mod warning_comparator;
pub mod weird_iter;
pub mod where_clause_needed;
pub mod widen;
pub mod with_predicate;
pub mod work_item_iterative_type_function_type_visitor;
pub mod work_item_iterative_type_visitor;

pub mod ast_array {
  pub use ulua_ast::records::ast_array::*;
}
pub mod ast_attr {
  pub use ulua_ast::records::ast_attr::*;
}
pub mod ast_expr {
  pub use ulua_ast::records::ast_expr::*;
}
pub mod ast_expr_binary {
  pub use ulua_ast::records::ast_expr_binary::*;
}
pub mod ast_expr_call {
  pub use ulua_ast::records::ast_expr_call::*;
}
pub mod ast_name {
  pub use ulua_ast::records::ast_name::*;
}
pub mod ast_node {
  pub use ulua_ast::records::ast_node::*;
}
pub mod ast_stat {
  pub use ulua_ast::records::ast_stat::*;
}
pub mod ast_stat_block {
  pub use ulua_ast::records::ast_stat_block::*;
}
pub mod ast_stat_repeat {
  pub use ulua_ast::records::ast_stat_repeat::*;
}
pub mod ast_type {
  pub use ulua_ast::records::ast_type::*;
}
pub mod ast_type_pack {
  pub use ulua_ast::records::ast_type_pack::*;
}
pub mod cst_node {
  pub use ulua_ast::records::cst_node::*;
}
pub mod cst_stat_local {
  pub use ulua_ast::records::cst_stat_local::*;
}
pub mod location {
  pub use ulua_ast::records::location::*;
}
pub mod position {
  pub use ulua_ast::records::position::*;
}
pub mod bound_type {
  pub use crate::type_aliases::bound_type::*;
}
pub mod constraint_vertex {
  pub use crate::type_aliases::constraint_vertex::*;
}
pub mod constraint_v {
  pub use crate::type_aliases::constraint_v::*;
}
pub mod dense_hash_map {
  pub use ulua_common::records::dense_hash_map::*;
}
pub mod dense_hash_set {
  pub use ulua_common::records::dense_hash_set::*;
}
pub mod error_type {
  pub use crate::type_aliases::error_type::*;
}
pub mod error_type_pack {
  pub use crate::type_aliases::error_type_pack::*;
}
pub mod lua_l_reg {
  pub use ulua_vm::records::lua_l_reg::*;
}
pub mod table_state {
  pub use crate::enums::table_state::*;
}
pub mod type_error_data {
  pub use crate::type_aliases::type_error_data::*;
}
pub mod type_function_instance_state {
  pub use crate::enums::type_function_instance_state::*;
}
pub mod bytecode_builder {
  pub use ulua_bytecode::records::bytecode_builder::*;
}
pub mod constant {
  pub use ulua_bytecode::records::constant::*;
}
pub mod constant_key {
  pub use ulua_bytecode::records::constant_key::*;
}
pub mod string_ref {
  pub use ulua_bytecode::records::string_ref::*;
}
pub mod unknown_symbol_alt_b {
  pub use crate::records::unknown_symbol::UnknownSymbol_Context;
}
pub mod unknown_symbol_alt_c {
  pub use crate::records::unknown_symbol::UnknownSymbol_Context;
}
pub mod unknown_symbol_alt_d {
  pub use crate::records::unknown_symbol::UnknownSymbol_Context;
}
pub mod fragment_type_check_status {
  pub use crate::enums::fragment_type_check_status::*;
}
pub mod type_variant {
  pub use crate::type_aliases::type_variant::*;
}
pub mod type_or_pack {
  pub use crate::type_aliases::type_or_pack::*;
}
pub mod type_pack_variant {
  pub use crate::type_aliases::type_pack_variant::*;
}
pub mod singleton_variant {
  pub use crate::type_aliases::singleton_variant::*;
}
pub mod table_shape {
  pub use ulua_compiler::records::table_shape::*;
}
pub mod mode {
  pub use ulua_ast::enums::mode::*;
}
