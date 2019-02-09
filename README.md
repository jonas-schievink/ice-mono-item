```
rustup install nightly-2019-02-09
rustup target add thumbv6m-none-eabi --toolchain nightly-2019-02-09
cargo +nightly-2019-02-09 check
```

```
error: internal compiler error: src/librustc_mir/monomorphize/collector.rs:745: Cannot create local mono-item for DefId(9/0:10 ~ r0[939b]::zero_bss[0])

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:595:9
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
             at src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:39
   1: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:70
   2: std::panicking::default_hook::{{closure}}
             at src/libstd/sys_common/backtrace.rs:58
             at src/libstd/panicking.rs:200
   3: std::panicking::default_hook
             at src/libstd/panicking.rs:215
   4: rustc::util::common::panic_hook
   5: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:482
   6: std::panicking::begin_panic
   7: rustc_errors::Handler::bug
   8: rustc::util::bug::opt_span_bug_fmt::{{closure}}
   9: rustc::ty::context::tls::with_opt::{{closure}}
  10: rustc::ty::context::tls::with_context_opt
  11: rustc::ty::context::tls::with_opt
  12: rustc::util::bug::opt_span_bug_fmt
  13: rustc::util::bug::bug_fmt
  14: rustc_mir::monomorphize::collector::should_monomorphize_locally
  15: rustc_mir::monomorphize::collector::visit_instance_use
  16: <rustc_mir::monomorphize::collector::MirNeighborCollector<'a, 'tcx> as rustc::mir::visit::Visitor<'tcx>>::visit_terminator_kind
  17: rustc_mir::monomorphize::collector::collect_items_rec
  18: rustc_mir::monomorphize::collector::collect_items_rec
  19: rustc_mir::monomorphize::collector::collect_crate_mono_items::{{closure}}
  20: rustc::util::common::time
  21: rustc_mir::monomorphize::collector::collect_crate_mono_items
  22: rustc::util::common::time
  23: rustc_mir::monomorphize::partitioning::collect_and_partition_mono_items
  24: rustc::ty::query::__query_compute::collect_and_partition_mono_items
  25: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::collect_and_partition_mono_items<'tcx>>::compute
  26: rustc::dep_graph::graph::DepGraph::with_task_impl
  27: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::try_get_with
  28: core::ops::function::FnOnce::call_once
  29: rustc::ty::query::__query_compute::backend_optimization_level
  30: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::backend_optimization_level<'tcx>>::compute
  31: rustc::dep_graph::graph::DepGraph::with_task_impl
  32: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::try_get_with
  33: rustc_codegen_llvm::back::write::create_target_machine
  34: rustc_codegen_llvm::context::create_module
  35: rustc_codegen_ssa::base::codegen_crate
  36: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_utils::codegen_backend::CodegenBackend>::codegen_crate
  37: rustc::util::common::time
  38: rustc_driver::driver::phase_4_codegen
  39: rustc_driver::driver::compile_input::{{closure}}
  40: <std::thread::local::LocalKey<T>>::with
  41: rustc::ty::context::TyCtxt::create_and_enter
  42: rustc_driver::driver::compile_input
  43: <scoped_tls::ScopedKey<T>>::set
  44: rustc_driver::run_compiler
  45: <scoped_tls::ScopedKey<T>>::set
query stack during panic:
#0 [collect_and_partition_mono_items] collect_and_partition_mono_items
#1 [backend_optimization_level] optimization level used by backend
end of query stack
error: aborting due to previous error


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.34.0-nightly (a2ec156a5 2019-02-08) running on x86_64-unknown-linux-gnu

note: compiler flags: -C opt-level=s -C debuginfo=2 -C debug-assertions=on -C link-arg=-Tlink.x --crate-type lib

note: some of the compiler flags provided by cargo are hidden
```

