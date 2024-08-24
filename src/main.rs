//! # Zuu
//!
//! is a command-line tool written in Rust that automates the process of checking and linting your Rust project's code.
//!
//! It likely stands for "Zero Unnecessary Untidiness" based on the comment "Code can be committed" displayed upon successful completion.
//!
//! ## Features
//!
//! * Attempts to execute each command using cargo
//! * If a command fails, it gracefully terminates the process and displays an error message
//!
//! ## Purpose
//!
//! * Code Quality Assurance: zuu helps maintain high code quality standards by automating the execution of various Rust checks and lints.
//! * Streamlined Development Workflow: It provides a convenient way to run multiple checks with a single command and visualize progress through a progress bar.
//! * Error Handling: It includes basic error handling to report issues during task execution.
//!
//! ## Notes
//!
//! The specific lints and checks performed depend on the chosen mode (ultra, high, medium, or low).
//!
use clap::{Arg, Command as Cmd};
use std::process::{Command, ExitCode};

#[doc = "The highest level of code quality and strictness"]
const RUST_ULTRA_TASKS: [&str; 8] = [
    "verify-project",
    "check --all-targets --profile=test",
    "deny check",
    "audit",
    "clippy \
        -- -D warnings \
        \
        -D clippy::all \
        -D clippy::pedantic \
        -D clippy::nursery \
        \
        -D clippy::style
        -D clippy::doc_markdown \
        -D clippy::items_after_statements \
        -D clippy::let_and_return \
        -D clippy::map_flatten \
        -D clippy::multiple_inherent_impl \
        -D clippy::needless_lifetimes \
        -D clippy::redundant_closure \
        -D clippy::shadow_reuse \
        -D clippy::similar_names \
        -D clippy::string_to_string \
        -D clippy::too_many_arguments \
        -D clippy::type_complexity \
        -D clippy::unnecessary_sort_by \
        -D clippy::wildcard_enum_match_arm \
        -D clippy::wildcard_imports \
        -D clippy::use_self \
        -D unused_braces \
        \
        -D clippy::suspicious
        \
        -D clippy::complexity
        -D clippy::cognitive_complexity \
        -D clippy::cognitive_complexity \
        \
        -D clippy::correctness
        -D clippy::as_conversions \
        -D clippy::clone_on_copy \
        -D clippy::create_dir \
        -D clippy::dbg_macro \
        -D clippy::default_trait_access \
        -D clippy::else_if_without_else \
        -D clippy::enum_glob_use \
        -D clippy::expect_used \
        -D clippy::explicit_into_iter_loop \
        -D clippy::fallible_impl_from \
        -D clippy::float_arithmetic \
        -D clippy::format_push_string \
        -D clippy::if_let_mutex \
        -D clippy::imprecise_flops \
        -D clippy::indexing_slicing \
        -D clippy::integer_division \
        -D clippy::large_enum_variant \
        -D clippy::lossy_float_literal \
        -D clippy::useless_conversion \
        -D clippy::manual_strip \
        -D clippy::match_bool \
        -D clippy::mem_forget \
        -D clippy::missing_errors_doc \
        -D clippy::missing_panics_doc \
        -D clippy::missing_safety_doc \
        -D clippy::mutex_integer \
        -D clippy::needless_collect \
        -D clippy::needless_pass_by_value \
        -D clippy::non_ascii_literal \
        -D clippy::option_if_let_else \
        -D clippy::panic \
        -D clippy::print_with_newline \
        -D clippy::ptr_arg \
        -D clippy::question_mark \
        -D clippy::rc_buffer \
        -D clippy::semicolon_if_nothing_returned \
        -D clippy::single_component_path_imports \
        -D clippy::string_add_assign \
        -D clippy::string_lit_as_bytes \
        -D clippy::trait_duplication_in_bounds \
        -D clippy::trivially_copy_pass_by_ref \
        -D clippy::unwrap_used \
        -D clippy::useless_conversion \
        -D clippy::used_underscore_binding \
        -D clippy::use_debug \
        \
        -D clippy::perf \
        -D clippy::future_not_send \
        -D clippy::get_unwrap \
        -D clippy::inefficient_to_string \
        -D clippy::let_unit_value \
        -D clippy::large_types_passed_by_value \
        -D clippy::linkedlist \
        -D clippy::map_entry \
        -D clippy::match_on_vec_items \
        -D clippy::match_same_arms \
        -D enum_intrinsics_non_enums \
        -D clippy::missing_const_for_fn \
        -D clippy::missing_enforced_import_renames \
        -D clippy::missing_inline_in_public_items \
        \
        -D clippy::cargo \
        -D future_incompatible \
        \
        -D anonymous_parameters \
        -D bare_trait_objects \
        -D dead_code \
        -D keyword_idents \
        -D let_underscore \
        -D macro_use_extern_crate \
        -D missing_copy_implementations \
        -D missing_docs \
        -D mutable_transmutes \
        -D path_statements \
        -D trivial_casts \
        -D trivial_numeric_casts \
        -D unused_allocation \
        -D unused_assignments \
        -D unused_extern_crates \
        -D unused_imports \
        -D unused_macro_rules \
        -D unused_must_use",
    "test -j 4 --no-fail-fast",
    "fmt --check",
    "outdated",
];
const RUST_TASKS: [&str; 8] = [
    "verify-project",
    "check --all-targets --profile=test",
    "deny check",
    "audit",
    "clippy \
        -- -D warnings \
        -D clippy::all \
        -D clippy::pedantic \
        -D clippy::nursery \
        \
        -D clippy::style
        -D clippy::doc_markdown \
        -D clippy::items_after_statements \
        -D clippy::let_and_return \
        -D clippy::map_flatten \
        -D clippy::multiple_inherent_impl \
        -D clippy::needless_lifetimes \
        -D clippy::redundant_closure \
        -D clippy::shadow_reuse \
        -D clippy::similar_names \
        -D clippy::string_to_string \
        -D clippy::too_many_arguments \
        -D clippy::type_complexity \
        -D clippy::unnecessary_sort_by \
        -D clippy::wildcard_enum_match_arm \
        -D clippy::wildcard_imports \
        -D clippy::use_self \
        -D unused_braces \
        \
        -D clippy::suspicious \
        \
        -D clippy::complexity \
        -D clippy::cognitive_complexity \
        \
        -D clippy::correctness \
        -D clippy::as_conversions \
        -D clippy::clone_on_copy \
        -D clippy::create_dir \
        -D clippy::dbg_macro \
        -D clippy::default_trait_access \
        -D clippy::derived_hash_with_manual_eq \
        -D clippy::else_if_without_else \
        -D clippy::enum_glob_use \
        -D clippy::expect_used \
        -D clippy::explicit_into_iter_loop \
        -D clippy::fallible_impl_from \
        -D clippy::filetype_is_file \
        -D clippy::float_arithmetic \
        -D clippy::format_push_string \
        -D clippy::future_not_send \
        -D clippy::get_unwrap \
        -D clippy::if_let_mutex \
        -D clippy::imprecise_flops \
        -D clippy::indexing_slicing \
        -D clippy::inefficient_to_string \
        -D clippy::integer_division \
        -D clippy::invalid_regex \
        -D clippy::large_enum_variant \
        -D clippy::large_stack_arrays \
        -D clippy::let_unit_value \
        -D clippy::linkedlist \
        -D clippy::lossy_float_literal \
        -D clippy::useless_conversion \
        -D clippy::manual_strip \
        -D clippy::map_entry \
        -D clippy::match_bool \
        -D clippy::match_on_vec_items \
        -D clippy::match_same_arms \
        -D enum_intrinsics_non_enums \
        -D clippy::mem_forget \
        -D clippy::missing_const_for_fn \
        -D clippy::missing_enforced_import_renames \
        -D clippy::missing_errors_doc \
        -D clippy::missing_inline_in_public_items \
        -D clippy::missing_panics_doc \
        -D clippy::missing_safety_doc \
        -D clippy::mutex_integer \
        -D clippy::needless_collect \
        -D clippy::needless_pass_by_value \
        -D clippy::non_ascii_literal \
        -D clippy::option_if_let_else \
        -D clippy::panic \
        -D clippy::print_with_newline \
        -D clippy::ptr_arg \
        -D clippy::question_mark \
        -D clippy::rc_buffer \
        -D clippy::semicolon_if_nothing_returned \
        -D clippy::single_component_path_imports \
        -D clippy::string_add_assign \
        -D clippy::string_lit_as_bytes \
        -D clippy::trait_duplication_in_bounds \
        -D clippy::trivially_copy_pass_by_ref \
        -D clippy::unwrap_used \
        -D clippy::useless_conversion \
        -D clippy::used_underscore_binding \
        -D clippy::use_debug \
        \
        -D clippy::cargo \
        -D future_incompatible \
        \
        -D anonymous_parameters \
        -D bare_trait_objects \
        -D dead_code \
        -D keyword_idents \
        -D let_underscore \
        -D macro_use_extern_crate \
        -D missing_copy_implementations \
        -D missing_docs \
        -D mutable_transmutes \
        -D no_mangle_generic_items \
        -D path_statements \
        -D trivial_casts \
        -D trivial_numeric_casts \
        -D unused_allocation \
        -D unused_assignments \
        -D unused_extern_crates \
        -D unused_imports \
        -D unused_macro_rules \
        -D unused_must_use",
    "test -j 4 --no-fail-fast",
    "fmt --check",
    "outdated",
];
#[doc = "The highest level of code quality and strictness"]
const RUST_STRICT_TASKS: [&str; 8] = [
    "verify-project",
    "check --all-targets --profile=test",
    "deny check",
    "audit",
    "clippy \
        -- -D warnings \
        -D clippy::all \
        -D clippy::pedantic \
        -D clippy::nursery \
        \
        -D clippy::style \
        -D clippy::doc_markdown \
        -D clippy::items_after_statements \
        -D clippy::let_and_return \
        -D clippy::map_flatten \
        -D clippy::multiple_inherent_impl \
        -D clippy::needless_lifetimes \
        -D clippy::redundant_closure \
        -D clippy::shadow_reuse \
        -D clippy::similar_names \
        -D clippy::string_to_string \
        -D clippy::too_many_arguments \
        -D clippy::type_complexity \
        -D clippy::unnecessary_sort_by \
        -D clippy::wildcard_enum_match_arm \
        -D clippy::wildcard_imports \
        -D clippy::use_self \
        -D unused_braces \
        \
        -D clippy::suspicious \
        \
        -D clippy::complexity \
        -D clippy::cognitive_complexity \
        \
        -D clippy::correctness \
        -D clippy::as_conversions \
        -D clippy::clone_on_copy \
        -D clippy::create_dir \
        -D clippy::dbg_macro \
        -D clippy::default_trait_access \
        -D clippy::derived_hash_with_manual_eq \
        -D clippy::else_if_without_else \
        -D clippy::enum_glob_use \
        -D clippy::expect_used \
        -D clippy::explicit_into_iter_loop \
        -D clippy::fallible_impl_from \
        -D clippy::filetype_is_file \
        -D clippy::float_arithmetic \
        -D clippy::format_push_string \
        -D clippy::future_not_send \
        -D clippy::get_unwrap \
        -D clippy::if_let_mutex \
        -D clippy::imprecise_flops \
        -D clippy::indexing_slicing \
        -D clippy::inefficient_to_string \
        -D clippy::integer_division \
        -D clippy::invalid_regex \
        -D clippy::large_enum_variant \
        -D clippy::large_stack_arrays \
        -D clippy::let_unit_value \
        -D clippy::linkedlist \
        -D clippy::lossy_float_literal \
        -D clippy::useless_conversion \
        -D clippy::manual_strip \
        -D clippy::map_entry \
        -D clippy::match_bool \
        -D clippy::match_on_vec_items \
        -D clippy::match_same_arms \
        -D enum_intrinsics_non_enums \
        -D clippy::mem_forget \
        -D clippy::missing_const_for_fn \
        -D clippy::missing_enforced_import_renames \
        -D clippy::missing_errors_doc \
        -D clippy::missing_inline_in_public_items \
        -D clippy::missing_panics_doc \
        -D clippy::missing_safety_doc \
        -D clippy::mutex_integer \
        -D clippy::needless_collect \
        -D clippy::needless_pass_by_value \
        -D clippy::non_ascii_literal \
        -D clippy::option_if_let_else \
        -D clippy::panic \
        -D clippy::print_with_newline \
        -D clippy::ptr_arg \
        -D clippy::question_mark \
        -D clippy::rc_buffer \
        -D clippy::semicolon_if_nothing_returned \
        -D clippy::single_component_path_imports \
        -D clippy::string_add_assign \
        -D clippy::string_lit_as_bytes \
        -D clippy::trait_duplication_in_bounds \
        -D clippy::trivially_copy_pass_by_ref \
        -D clippy::unwrap_used \
        -D clippy::useless_conversion \
        -D clippy::used_underscore_binding \
        -D clippy::use_debug \
        \
        -D clippy::cargo \
        -D future_incompatible \
        \
        -D anonymous_parameters \
        -D bare_trait_objects \
        -D dead_code \
        -D keyword_idents \
        -D let_underscore \
        -D macro_use_extern_crate \
        -D missing_copy_implementations \
        -D missing_docs \
        -D mutable_transmutes \
        -D no_mangle_generic_items \
        -D path_statements \
        -D trivial_casts \
        -D trivial_numeric_casts \
        -D unused_allocation \
        -D unused_assignments \
        -D unused_extern_crates \
        -D unused_imports \
        -D unused_macro_rules \
        -D unused_must_use",
    "test -j 4 --no-fail-fast",
    "fmt --check",
    "outdated",
];

#[doc = "Configuration maintains a strong focus on code quality and best practices"]
const RUST_HIGH_TASKS: [&str; 8] = [
    "verify-project",
    "check --all-targets --profile=test",
    "deny check",
    "audit",
    "clippy \
        -- -D warnings \
        \
        -D clippy::all \
        -D clippy::pedantic \
        \
        -D clippy::style
        -D clippy::doc_markdown \
        -D clippy::items_after_statements \
        -D clippy::let_and_return \
        -D clippy::map_flatten \
        -D clippy::multiple_inherent_impl \
        -D clippy::needless_lifetimes \
        -D clippy::redundant_closure \
        -D clippy::shadow_reuse \
        -D clippy::similar_names \
        -D clippy::string_to_string \
        -D clippy::too_many_arguments \
        -D clippy::type_complexity \
        -D clippy::unnecessary_sort_by \
        -D clippy::wildcard_enum_match_arm \
        -D clippy::wildcard_imports \
        -D clippy::use_self \
        -D unused_braces \
        \
        -D clippy::suspicious
        \
        -D clippy::complexity
        -D clippy::cognitive_complexity \
        -D clippy::cognitive_complexity \
        \
        -D clippy::correctness
        -D clippy::as_conversions \
        -D clippy::clone_on_copy \
        -D clippy::create_dir \
        -D clippy::dbg_macro \
        -D clippy::default_trait_access \
        -D clippy::else_if_without_else \
        -D clippy::enum_glob_use \
        -D clippy::expect_used \
        -D clippy::explicit_into_iter_loop \
        -D clippy::fallible_impl_from \
        -D clippy::float_arithmetic \
        -D clippy::format_push_string \
        -D clippy::if_let_mutex \
        -D clippy::imprecise_flops \
        -D clippy::indexing_slicing \
        -D clippy::integer_division \
        -D clippy::large_enum_variant \
        -D clippy::lossy_float_literal \
        -D clippy::useless_conversion \
        -D clippy::manual_strip \
        -D clippy::match_bool \
        -D clippy::mem_forget \
        -D clippy::missing_errors_doc \
        -D clippy::missing_panics_doc \
        -D clippy::missing_safety_doc \
        -D clippy::mutex_integer \
        -D clippy::needless_collect \
        -D clippy::needless_pass_by_value \
        -D clippy::non_ascii_literal \
        -D clippy::option_if_let_else \
        -D clippy::panic \
        -D clippy::print_with_newline \
        -D clippy::ptr_arg \
        -D clippy::question_mark \
        -D clippy::rc_buffer \
        -D clippy::semicolon_if_nothing_returned \
        -D clippy::single_component_path_imports \
        -D clippy::string_add_assign \
        -D clippy::string_lit_as_bytes \
        -D clippy::trait_duplication_in_bounds \
        -D clippy::trivially_copy_pass_by_ref \
        -D clippy::unwrap_used \
        -D clippy::useless_conversion \
        -D clippy::used_underscore_binding \
        -D clippy::use_debug \
        \
        -D clippy::perf\
        -D clippy::future_not_send \
        -D clippy::get_unwrap \
        -D clippy::inefficient_to_string \
        -D clippy::let_unit_value \
        -D clippy::large_types_passed_by_value \
        -D clippy::linkedlist \
        -D clippy::map_entry \
        -D clippy::match_on_vec_items \
        -D clippy::match_same_arms \
        -D enum_intrinsics_non_enums \
        -D clippy::missing_const_for_fn \
        -D clippy::missing_enforced_import_renames \
        -D clippy::missing_inline_in_public_items \
        \
        -D clippy::cargo \
        -D future_incompatible \
        \
        -D anonymous_parameters \
        -D bare_trait_objects \
        -D dead_code \
        -D keyword_idents \
        -W let_underscore \
        -D macro_use_extern_crate \
        -D missing_copy_implementations \
        -D missing_docs \
        -D mutable_transmutes \
        -D path_statements \
        -D trivial_casts \
        -D trivial_numeric_casts \
        -D unused_allocation \
        -D unused_assignments \
        -D unused_extern_crates \
        -D unused_imports \
        -D unused_macro_rules \
        -D unused_must_use \
        \
        -W clippy::nursery",
    "test -j 4 --no-fail-fast",
    "fmt --check",
    "outdated",
];
#[doc = "Balance between strictness and flexibility"]
const RUST_MEDIUM_TASKS: [&str; 8] = [
    "verify-project",
    "check --all-targets --profile=test",
    "deny check",
    "audit",
    "clippy \
        -- -W warnings
        \
        -W clippy::all \
        -W clippy::pedantic \
        \
        -D clippy::style \
        -D clippy::doc_markdown \
        -D clippy::items_after_statements \
        -D clippy::let_and_return \
        -D clippy::map_flatten \
        -D clippy::multiple_inherent_impl \
        -D clippy::needless_lifetimes \
        -D clippy::redundant_closure \
        -D clippy::shadow_reuse \
        -D clippy::similar_names \
        -D clippy::string_to_string \
        -D clippy::too_many_arguments \
        -D clippy::type_complexity \
        -D clippy::unnecessary_sort_by \
        -D clippy::wildcard_enum_match_arm \
        -D clippy::wildcard_imports \
        -D clippy::use_self \
        -D unused_braces \
        \
        -W clippy::suspicious
        \
        -W clippy::complexity
        -W clippy::cognitive_complexity \
        -W clippy::cognitive_complexity \
        \
        -D clippy::correctness \
        -D clippy::as_conversions \
        -D clippy::clone_on_copy \
        -D clippy::create_dir \
        -D clippy::dbg_macro \
        -D clippy::default_trait_access \
        -D clippy::derived_hash_with_manual_eq \
        -D clippy::else_if_without_else \
        -D clippy::enum_glob_use \
        -D clippy::expect_used \
        -D clippy::explicit_into_iter_loop \
        -D clippy::fallible_impl_from \
        -D clippy::float_arithmetic \
        -D clippy::format_push_string \
        -D clippy::if_let_mutex \
        -D clippy::imprecise_flops \
        -D clippy::indexing_slicing \
        -D clippy::integer_division \
        -D clippy::large_enum_variant \
        -D clippy::lossy_float_literal \
        -D clippy::manual_strip \
        -D clippy::match_bool \
        -D clippy::missing_errors_doc \
        -D clippy::missing_panics_doc \
        -D clippy::missing_safety_doc \
        -D clippy::mutex_integer \
        -D clippy::needless_collect \
        -D clippy::needless_pass_by_value \
        -D clippy::non_ascii_literal \
        -D clippy::option_if_let_else \
        -D clippy::panic \
        -D clippy::print_with_newline \
        -D clippy::ptr_arg \
        -D clippy::question_mark \
        -D clippy::rc_buffer \
        -D clippy::semicolon_if_nothing_returned \
        -D clippy::single_component_path_imports \
        -D clippy::string_add_assign \
        -D clippy::string_lit_as_bytes \
        -D clippy::trait_duplication_in_bounds \
        -D clippy::trivially_copy_pass_by_ref \
        -D clippy::unwrap_used \
        -D clippy::useless_conversion \
        -D clippy::used_underscore_binding \
        -D clippy::use_debug \
        \
        -D clippy::perf \
        -D clippy::future_not_send \
        -D clippy::get_unwrap \
        -D clippy::inefficient_to_string \
        -D clippy::let_unit_value \
        -D clippy::large_types_passed_by_value \
        -D clippy::linkedlist \
        -D clippy::map_entry \
        -D clippy::match_on_vec_items \
        -D clippy::match_same_arms \
        -D enum_intrinsics_non_enums \
        -D clippy::missing_const_for_fn \
        -D clippy::missing_enforced_import_renames \
        -D clippy::missing_inline_in_public_items \
        \
        -D clippy::cargo \
        -D future_incompatible \
        \
        -D anonymous_parameters \
        -D bare_trait_objects \
        -D dead_code \
        -D keyword_idents \
        -W let_underscore \
        -D macro_use_extern_crate \
        -D missing_copy_implementations \
        -D missing_docs \
        -D mutable_transmutes \
        -D path_statements \
        -D trivial_casts \
        -D trivial_numeric_casts \
        -D unused_allocation \
        -D unused_assignments \
        -D unused_extern_crates \
        -D unused_imports \
        -D unused_macro_rules \
        -D unused_must_use",
    "test -j 4 --no-fail-fast",
    "fmt --check",
    "outdated",
];

#[doc = "Configuration that focuses on essential checks and warnings"]
const RUST_LOW_TASKS: [&str; 8] = [
    "verify-project",
    "check --all-targets --profile=test",
    "deny check",
    "audit",
    "clippy \
        -- -W warnings \
        -W clippy::all \
        -W clippy::pedantic \
        -W clippy::style \
        -W clippy::doc_markdown \
        -W clippy::items_after_statements \
        -W clippy::let_and_return \
        -W clippy::map_flatten \
        -W clippy::multiple_inherent_impl \
        -W clippy::needless_lifetimes \
        -W clippy::redundant_closure \
        -W clippy::shadow_reuse \
        -W clippy::similar_names \
        -W clippy::string_to_string \
        -W clippy::too_many_arguments \
        -W clippy::type_complexity \
        -W clippy::unnecessary_sort_by \
        -W clippy::wildcard_enum_match_arm \
        -W clippy::wildcard_imports \
        -W clippy::use_self \
        -W unused_braces \
        -W clippy::suspicious
        \
        -W clippy::complexity
        -W clippy::cognitive_complexity \
        -W clippy::cognitive_complexity \
        \
        -W clippy::correctness
        -W clippy::as_conversions \
        -W clippy::clone_on_copy \
        -W clippy::create_dir \
        -W clippy::dbg_macro \
        -W clippy::default_trait_access \
        -W clippy::derived_hash_with_manual_eq \
        -W clippy::else_if_without_else \
        -W clippy::enum_glob_use \
        -W clippy::expect_used \
        -W clippy::explicit_into_iter_loop \
        -W clippy::fallible_impl_from \
        -W clippy::float_arithmetic \
        -W clippy::format_push_string \
        -W clippy::if_let_mutex \
        -W clippy::imprecise_flops \
        -W clippy::indexing_slicing \
        -W clippy::integer_division \
        -W clippy::large_enum_variant \
        -W clippy::lossy_float_literal \
        -W clippy::manual_strip \
        -W clippy::match_bool \
        -W clippy::missing_errors_doc \
        -W clippy::missing_panics_doc \
        -W clippy::missing_safety_doc \
        -W clippy::mutex_integer \
        -W clippy::needless_collect \
        -W clippy::needless_pass_by_value \
        -W clippy::non_ascii_literal \
        -W clippy::option_if_let_else \
        -W clippy::panic \
        -W clippy::print_with_newline \
        -W clippy::ptr_arg \
        -W clippy::question_mark \
        -W clippy::rc_buffer \
        -W clippy::semicolon_if_nothing_returned \
        -W clippy::single_component_path_imports \
        -W clippy::string_add_assign \
        -W clippy::string_lit_as_bytes \
        -W clippy::trait_duplication_in_bounds \
        -W clippy::trivially_copy_pass_by_ref \
        -W clippy::unwrap_used \
        -W clippy::useless_conversion \
        -W clippy::used_underscore_binding \
        -W clippy::use_debug \
        -W clippy::perf
        -W clippy::future_not_send \
        -W clippy::get_unwrap \
        -W clippy::inefficient_to_string \
        -W clippy::let_unit_value \
        -W clippy::large_types_passed_by_value \
        -W clippy::linkedlist \
        -W clippy::map_entry \
        -W clippy::match_on_vec_items \
        -W clippy::match_same_arms \
        -W enum_intrinsics_non_enums \
        -W clippy::missing_const_for_fn \
        -W clippy::missing_enforced_import_renames \
        -W clippy::missing_inline_in_public_items \
        -W clippy::cargo \
        -W future_incompatible \
        \
        -W anonymous_parameters \
        -W bare_trait_objects \
        -W dead_code \
        -W keyword_idents \
        -W let_underscore \
        -W macro_use_extern_crate \
        -W missing_copy_implementations \
        -W missing_docs \
        -W mutable_transmutes \
        -W path_statements \
        -W trivial_casts \
        -W trivial_numeric_casts \
        -W unused_allocation \
        -W unused_assignments \
        -W unused_extern_crates \
        -W unused_imports \
        -W unused_macro_rules \
        -W unused_must_use",
    "test -j 4 --no-fail-fast",
    "fmt --check",
    "outdated",
];
#[doc = "Check all tasks by mode"]
fn zuu(tasks: &[&str; 8]) -> ExitCode {
    for task in tasks {
        let x: Vec<&str> = task.split_whitespace().collect();
        if let Ok(mut child) = Command::new("cargo").args(x).current_dir(".").spawn() {
            if let Ok(code) = child.wait() {
                if code.success().eq(&false) {
                    return ExitCode::FAILURE;
                }
            }
        }
    }
    ExitCode::SUCCESS
}
#[doc = "Check source code"]
fn main() -> ExitCode {
    let app = Cmd::new("zuu")
        .arg(
            Arg::new("mode")
                .required(true)
                .short('m')
                .long("mode")
                .default_value("strict"),
        )
        .get_matches();
    app.get_one::<String>("mode")
        .map_or(ExitCode::FAILURE, |mode| match mode.as_str() {
            "ultra" => zuu(&RUST_ULTRA_TASKS),
            "high" => zuu(&RUST_HIGH_TASKS),
            "medium" => zuu(&RUST_MEDIUM_TASKS),
            "low" => zuu(&RUST_LOW_TASKS),
            "strict" => zuu(&RUST_STRICT_TASKS),
            _ => zuu(&RUST_TASKS),
        })
}
