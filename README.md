# Rustc ICE Reproduction: Unsize Coercion with `!Sync` Futures

This repository contains a minimal reproduction code that triggers an Internal Compiler Error (ICE) in `rustc`.

**Backtrace:**
```text
canmi@xyy ~/C/P/future_sync_ice (main)> RUST_BACKTRACE=full cargo check
    Checking vane v0.6.7 (/Users/canmi/Canmi/Project/future_sync_ice)
error: internal compiler error: compiler/rustc_mir_transform/src/validate.rs:81:25: broken MIR in Item(DefId(0:29 ~ vane[a94f]::repro::serve_request::{closure#0})) (after phase change to runtime-optimized) at bb13[0]:
                                Unsize coercion, but `std::pin::Pin<std::boxed::Box<{async block@src/main.rs:58:38: 58:48}>>` isn't coercible to `std::pin::Pin<std::boxed::Box<dyn std::future::Future<Output = ()> + std::marker::Send + std::marker::Sync>>`
  --> src/main.rs:67:72
   |
67 |         let _target: Pin<Box<dyn Future<Output = ()> + Send + Sync>> = tunnel_future;
   |                                                                        ^^^^^^^^^^^^^


thread 'rustc' (1554380) panicked at compiler/rustc_mir_transform/src/validate.rs:81:25:
Box<dyn Any>
stack backtrace:
   0:        0x11124d384 - <std::sys::backtrace::BacktraceLock::print::DisplayBacktrace as core::fmt::Display>::fmt::h2808c764b780ef77
   1:        0x10e68f6d8 - core::fmt::write::h164e2e78980a3344
   2:        0x111206504 - std::io::Write::write_fmt::hc5b2a4d160c11c86
   3:        0x1112185a8 - std::sys::backtrace::BacktraceLock::print::h765d16e49e83c7b3
   4:        0x11121ee2c - std::panicking::default_hook::{{closure}}::hf5d7ef6cefa10586
   5:        0x11121eacc - std::panicking::default_hook::h8781bd6c5c54e4cc
   6:        0x10f23dae0 - std[dc0ae436bb89db06]::panicking::update_hook::<alloc[bc4fa8be2c1bb627]::boxed::Box<rustc_driver_impl[50cf2a748b5c59a1]::install_ice_hook::{closure#1}>>::{closure#0}
   7:        0x11121f354 - std::panicking::panic_with_hook::h93c775fc227522dd
   8:        0x10f2bee04 - std[dc0ae436bb89db06]::panicking::begin_panic::<rustc_errors[49e2b406ffe14c44]::ExplicitBug>::{closure#0}
   9:        0x10f2ac59c - std[dc0ae436bb89db06]::sys::backtrace::__rust_end_short_backtrace::<std[dc0ae436bb89db06]::panicking::begin_panic<rustc_errors[49e2b406ffe14c44]::ExplicitBug>::{closure#0}, !>
  10:        0x113feb218 - std[dc0ae436bb89db06]::panicking::begin_panic::<rustc_errors[49e2b406ffe14c44]::ExplicitBug>
  11:        0x113febdc8 - <rustc_errors[49e2b406ffe14c44]::diagnostic::BugAbort as rustc_errors[49e2b406ffe14c44]::diagnostic::EmissionGuarantee>::emit_producing_guarantee
  12:        0x11405793c - <rustc_errors[49e2b406ffe14c44]::DiagCtxtHandle>::span_bug::<rustc_span[b61ca60f5275f53a]::span_encoding::Span, alloc[bc4fa8be2c1bb627]::string::String>
  13:        0x1140589a8 - rustc_middle[3debd01596c306bb]::util::bug::opt_span_bug_fmt::<rustc_span[b61ca60f5275f53a]::span_encoding::Span>::{closure#0}
  14:        0x10fe81c04 - rustc_middle[3debd01596c306bb]::ty::context::tls::with_opt::<rustc_middle[3debd01596c306bb]::util::bug::opt_span_bug_fmt<rustc_span[b61ca60f5275f53a]::span_encoding::Span>::{closure#0}, !>::{closure#0}
  15:        0x10fe5aa68 - rustc_middle[3debd01596c306bb]::ty::context::tls::with_context_opt::<rustc_middle[3debd01596c306bb]::ty::context::tls::with_opt<rustc_middle[3debd01596c306bb]::util::bug::opt_span_bug_fmt<rustc_span[b61ca60f5275f53a]::span_encoding::Span>::{closure#0}, !>::{closure#0}, !>
  16:        0x114057c64 - rustc_middle[3debd01596c306bb]::util::bug::span_bug_fmt::<rustc_span[b61ca60f5275f53a]::span_encoding::Span>
  17:        0x11013ed7c - <rustc_mir_transform[30feda94a763ca72]::validate::CfgChecker>::fail::<alloc[bc4fa8be2c1bb627]::string::String>
  18:        0x110267240 - <rustc_mir_transform[30feda94a763ca72]::validate::Validator as rustc_mir_transform[30feda94a763ca72]::pass_manager::MirPass>::run_pass
  19:        0x1101f8a3c - rustc_mir_transform[30feda94a763ca72]::pass_manager::run_passes_inner
  20:        0x1101b6928 - rustc_mir_transform[30feda94a763ca72]::run_optimization_passes
  21:        0x1101b5cb8 - rustc_mir_transform[30feda94a763ca72]::optimized_mir
  22:        0x1106cff30 - rustc_query_impl[1913c4dc82019177]::plumbing::__rust_begin_short_backtrace::<rustc_query_impl[1913c4dc82019177]::query_impl::optimized_mir::dynamic_query::{closure#2}::{closure#0}, rustc_middle[3debd01596c306bb]::query::erase::Erased<[u8; 8usize]>>
  23:        0x110718e54 - rustc_query_system[55b68c9b31defbbe]::query::plumbing::try_execute_query::<rustc_query_impl[1913c4dc82019177]::DynamicConfig<rustc_query_system[55b68c9b31defbbe]::query::caches::DefIdCache<rustc_middle[3debd01596c306bb]::query::erase::Erased<[u8; 8usize]>>, false, false, false>, rustc_query_impl[1913c4dc82019177]::plumbing::QueryCtxt, true>
  24:        0x11085e308 - rustc_query_impl[1913c4dc82019177]::query_impl::optimized_mir::get_query_incr::__rust_end_short_backtrace
  25:        0x10fece630 - <rustc_middle[3debd01596c306bb]::ty::context::TyCtxt>::coroutine_layout
  26:        0x111163e48 - rustc_ty_utils[f20e79acf91c0c4d]::layout::layout_of
  27:        0x1106d5b74 - rustc_query_impl[1913c4dc82019177]::plumbing::__rust_begin_short_backtrace::<rustc_query_impl[1913c4dc82019177]::query_impl::layout_of::dynamic_query::{closure#2}::{closure#0}, rustc_middle[3debd01596c306bb]::query::erase::Erased<[u8; 16usize]>>
  28:        0x1109e7f8c - <rustc_query_impl[1913c4dc82019177]::query_impl::layout_of::dynamic_query::{closure#2} as core[efaf33ed1a5ac28d]::ops::function::FnOnce<(rustc_middle[3debd01596c306bb]::ty::context::TyCtxt, rustc_middle[3debd01596c306bb]::ty::PseudoCanonicalInput<rustc_middle[3debd01596c306bb]::ty::Ty>)>>::call_once
  29:        0x11072b63c - rustc_query_system[55b68c9b31defbbe]::query::plumbing::try_execute_query::<rustc_query_impl[1913c4dc82019177]::DynamicConfig<rustc_query_system[55b68c9b31defbbe]::query::caches::DefaultCache<rustc_middle[3debd01596c306bb]::ty::PseudoCanonicalInput<rustc_middle[3debd01596c306bb]::ty::Ty>, rustc_middle[3debd01596c306bb]::query::erase::Erased<[u8; 16usize]>>, false, true, false>, rustc_query_impl[1913c4dc82019177]::plumbing::QueryCtxt, true>
  30:        0x11088eda8 - rustc_query_impl[1913c4dc82019177]::query_impl::layout_of::get_query_incr::__rust_end_short_backtrace
  31:        0x1111a9b5c - <rustc_middle[3debd01596c306bb]::ty::layout::LayoutCx as rustc_middle[3debd01596c306bb]::ty::layout::LayoutOf>::spanned_layout_of
  32:        0x11116d364 - <core[efaf33ed1a5ac28d]::iter::adapters::GenericShunt<core[efaf33ed1a5ac28d]::iter::adapters::by_ref_sized::ByRefSized<core[efaf33ed1a5ac28d]::iter::adapters::map::Map<core[efaf33ed1a5ac28d]::slice::iter::Iter<rustc_middle[3debd01596c306bb]::ty::VariantDef>, rustc_ty_utils[f20e79acf91c0c4d]::layout::layout_of_uncached::{closure#20}>>, core[efaf33ed1a5ac28d]::result::Result<core[efaf33ed1a5ac28d]::convert::Infallible, &rustc_middle[3debd01596c306bb]::ty::layout::LayoutError>> as core[efaf33ed1a5ac28d]::iter::traits::iterator::Iterator>::next
  33:        0x11112887c - core[efaf33ed1a5ac28d]::iter::adapters::try_process::<core[efaf33ed1a5ac28d]::iter::adapters::by_ref_sized::ByRefSized<core[efaf33ed1a5ac28d]::iter::adapters::map::Map<core[efaf33ed1a5ac28d]::slice::iter::Iter<rustc_middle[3debd01596c306bb]::ty::VariantDef>, rustc_ty_utils[f20e79acf91c0c4d]::layout::layout_of_uncached::{closure#20}>>, rustc_index[3b6d27c33e9a86b4]::vec::IndexVec<rustc_abi[b9b81dfdd0cd95d4]::layout::ty::FieldIdx, rustc_abi[b9b81dfdd0cd95d4]::layout::ty::TyAndLayout<rustc_middle[3debd01596c306bb]::ty::Ty>>, core[efaf33ed1a5ac28d]::result::Result<core[efaf33ed1a5ac28d]::convert::Infallible, &rustc_middle[3debd01596c306bb]::ty::layout::LayoutError>, <core[efaf33ed1a5ac28d]::iter::adapters::map::Map<core[efaf33ed1a5ac28d]::slice::iter::Iter<rustc_middle[3debd01596c306bb]::ty::VariantDef>, rustc_ty_utils[f20e79acf91c0c4d]::layout::layout_of_uncached::{closure#20}> as core[efaf33ed1a5ac28d]::iter::traits::iterator::Iterator>::try_collect<rustc_index[3b6d27c33e9a86b4]::vec::IndexVec<rustc_abi[b9b81dfdd0cd95d4]::layout::ty::VariantIdx, rustc_index[3b6d27c33e9a86b4]::vec::IndexVec<rustc_abi[b9b81dfdd0cd95d4]::layout::ty::FieldIdx, rustc_abi[b9b81dfdd0cd95d4]::layout::ty::TyAndLayout<rustc_middle[3debd01596c306bb]::ty::Ty>>>>::{closure#0}, rustc_index[3b6d27c33e9a86b4]::vec::IndexVec<rustc_abi[b9b81dfdd0cd95d4]::layout::ty::VariantIdx, rustc_index[3b6d27c33e9a86b4]::vec::IndexVec<rustc_abi[b9b81dfdd0cd95d4]::layout::ty::FieldIdx, rustc_abi[b9b81dfdd0cd95d4]::layout::ty::TyAndLayout<rustc_middle[3debd01596c306bb]::ty::Ty>>>>
  34:        0x111163ccc - rustc_ty_utils[f20e79acf91c0c4d]::layout::layout_of
  35:        0x1106d5b74 - rustc_query_impl[1913c4dc82019177]::plumbing::__rust_begin_short_backtrace::<rustc_query_impl[1913c4dc82019177]::query_impl::layout_of::dynamic_query::{closure#2}::{closure#0}, rustc_middle[3debd01596c306bb]::query::erase::Erased<[u8; 16usize]>>
  36:        0x1109e7f8c - <rustc_query_impl[1913c4dc82019177]::query_impl::layout_of::dynamic_query::{closure#2} as core[efaf33ed1a5ac28d]::ops::function::FnOnce<(rustc_middle[3debd01596c306bb]::ty::context::TyCtxt, rustc_middle[3debd01596c306bb]::ty::PseudoCanonicalInput<rustc_middle[3debd01596c306bb]::ty::Ty>)>>::call_once
  37:        0x11072b63c - rustc_query_system[55b68c9b31defbbe]::query::plumbing::try_execute_query::<rustc_query_impl[1913c4dc82019177]::DynamicConfig<rustc_query_system[55b68c9b31defbbe]::query::caches::DefaultCache<rustc_middle[3debd01596c306bb]::ty::PseudoCanonicalInput<rustc_middle[3debd01596c306bb]::ty::Ty>, rustc_middle[3debd01596c306bb]::query::erase::Erased<[u8; 16usize]>>, false, true, false>, rustc_query_impl[1913c4dc82019177]::plumbing::QueryCtxt, true>
  38:        0x11088eda8 - rustc_query_impl[1913c4dc82019177]::query_impl::layout_of::get_query_incr::__rust_end_short_backtrace
  39:        0x1111a9b5c - <rustc_middle[3debd01596c306bb]::ty::layout::LayoutCx as rustc_middle[3debd01596c306bb]::ty::layout::LayoutOf>::spanned_layout_of
  40:        0x11116d3f4 - <core[efaf33ed1a5ac28d]::iter::adapters::GenericShunt<core[efaf33ed1a5ac28d]::iter::adapters::by_ref_sized::ByRefSized<core[efaf33ed1a5ac28d]::iter::adapters::map::Map<core[efaf33ed1a5ac28d]::slice::iter::Iter<rustc_middle[3debd01596c306bb]::ty::VariantDef>, rustc_ty_utils[f20e79acf91c0c4d]::layout::layout_of_uncached::{closure#20}>>, core[efaf33ed1a5ac28d]::result::Result<core[efaf33ed1a5ac28d]::convert::Infallible, &rustc_middle[3debd01596c306bb]::ty::layout::LayoutError>> as core[efaf33ed1a5ac28d]::iter::traits::iterator::Iterator>::next
  41:        0x11112887c - core[efaf33ed1a5ac28d]::iter::adapters::try_process::<core[efaf33ed1a5ac28d]::iter::adapters::by_ref_sized::ByRefSized<core[efaf33ed1a5ac28d]::iter::adapters::map::Map<core[efaf33ed1a5ac28d]::slice::iter::Iter<rustc_middle[3debd01596c306bb]::ty::VariantDef>, rustc_ty_utils[f20e79acf91c0c4d]::layout::layout_of_uncached::{closure#20}>>, rustc_index[3b6d27c33e9a86b4]::vec::IndexVec<rustc_abi[b9b81dfdd0cd95d4]::layout::ty::FieldIdx, rustc_abi[b9b81dfdd0cd95d4]::layout::ty::TyAndLayout<rustc_middle[3debd01596c306bb]::ty::Ty>>, core[efaf33ed1a5ac28d]::result::Result<core[efaf33ed1a5ac28d]::convert::Infallible, &rustc_middle[3debd01596c306bb]::ty::layout::LayoutError>, <core[efaf33ed1a5ac28d]::iter::adapters::map::Map<core[efaf33ed1a5ac28d]::slice::iter::Iter<rustc_middle[3debd01596c306bb]::ty::VariantDef>, rustc_ty_utils[f20e79acf91c0c4d]::layout::layout_of_uncached::{closure#20}> as core[efaf33ed1a5ac28d]::iter::traits::iterator::Iterator>::try_collect<rustc_index[3b6d27c33e9a86b4]::vec::IndexVec<rustc_abi[b9b81dfdd0cd95d4]::layout::ty::VariantIdx, rustc_index[3b6d27c33e9a86b4]::vec::IndexVec<rustc_abi[b9b81dfdd0cd95d4]::layout::ty::FieldIdx, rustc_abi[b9b81dfdd0cd95d4]::layout::ty::TyAndLayout<rustc_middle[3debd01596c306bb]::ty::Ty>>>>::{closure#0}, rustc_index[3b6d27c33e9a86b4]::vec::IndexVec<rustc_abi[b9b81dfdd0cd95d4]::layout::ty::VariantIdx, rustc_index[3b6d27c33e9a86b4]::vec::IndexVec<rustc_abi[b9b81dfdd0cd95d4]::layout::ty::FieldIdx, rustc_abi[b9b81dfdd0cd95d4]::layout::ty::TyAndLayout<rustc_middle[3debd01596c306bb]::ty::Ty>>>>
  42:        0x111163ccc - rustc_ty_utils[f20e79acf91c0c4d]::layout::layout_of
  43:        0x1106d5b74 - rustc_query_impl[1913c4dc82019177]::plumbing::__rust_begin_short_backtrace::<rustc_query_impl[1913c4dc82019177]::query_impl::layout_of::dynamic_query::{closure#2}::{closure#0}, rustc_middle[3debd01596c306bb]::query::erase::Erased<[u8; 16usize]>>
  44:        0x1109e7f8c - <rustc_query_impl[1913c4dc82019177]::query_impl::layout_of::dynamic_query::{closure#2} as core[efaf33ed1a5ac28d]::ops::function::FnOnce<(rustc_middle[3debd01596c306bb]::ty::context::TyCtxt, rustc_middle[3debd01596c306bb]::ty::PseudoCanonicalInput<rustc_middle[3debd01596c306bb]::ty::Ty>)>>::call_once
  45:        0x11072b63c - rustc_query_system[55b68c9b31defbbe]::query::plumbing::try_execute_query::<rustc_query_impl[1913c4dc82019177]::DynamicConfig<rustc_query_system[55b68c9b31defbbe]::query::caches::DefaultCache<rustc_middle[3debd01596c306bb]::ty::PseudoCanonicalInput<rustc_middle[3debd01596c306bb]::ty::Ty>, rustc_middle[3debd01596c306bb]::query::erase::Erased<[u8; 16usize]>>, false, true, false>, rustc_query_impl[1913c4dc82019177]::plumbing::QueryCtxt, true>
  46:        0x11088eda8 - rustc_query_impl[1913c4dc82019177]::query_impl::layout_of::get_query_incr::__rust_end_short_backtrace
  47:        0x1111a9b5c - <rustc_middle[3debd01596c306bb]::ty::layout::LayoutCx as rustc_middle[3debd01596c306bb]::ty::layout::LayoutOf>::spanned_layout_of
  48:        0x11116d5bc - <core[efaf33ed1a5ac28d]::iter::adapters::GenericShunt<core[efaf33ed1a5ac28d]::iter::adapters::by_ref_sized::ByRefSized<core[efaf33ed1a5ac28d]::iter::adapters::map::Map<core[efaf33ed1a5ac28d]::slice::iter::Iter<rustc_middle[3debd01596c306bb]::mir::query::CoroutineSavedTy>, rustc_ty_utils[f20e79acf91c0c4d]::layout::layout_of_uncached::{closure#12}>>, core[efaf33ed1a5ac28d]::result::Result<core[efaf33ed1a5ac28d]::convert::Infallible, &rustc_middle[3debd01596c306bb]::ty::layout::LayoutError>> as core[efaf33ed1a5ac28d]::iter::traits::iterator::Iterator>::next
  49:        0x111128a58 - core[efaf33ed1a5ac28d]::iter::adapters::try_process::<core[efaf33ed1a5ac28d]::iter::adapters::by_ref_sized::ByRefSized<core[efaf33ed1a5ac28d]::iter::adapters::map::Map<core[efaf33ed1a5ac28d]::slice::iter::Iter<rustc_middle[3debd01596c306bb]::mir::query::CoroutineSavedTy>, rustc_ty_utils[f20e79acf91c0c4d]::layout::layout_of_uncached::{closure#12}>>, rustc_abi[b9b81dfdd0cd95d4]::layout::ty::TyAndLayout<rustc_middle[3debd01596c306bb]::ty::Ty>, core[efaf33ed1a5ac28d]::result::Result<core[efaf33ed1a5ac28d]::convert::Infallible, &rustc_middle[3debd01596c306bb]::ty::layout::LayoutError>, <core[efaf33ed1a5ac28d]::iter::adapters::map::Map<core[efaf33ed1a5ac28d]::slice::iter::Iter<rustc_middle[3debd01596c306bb]::mir::query::CoroutineSavedTy>, rustc_ty_utils[f20e79acf91c0c4d]::layout::layout_of_uncached::{closure#12}> as core[efaf33ed1a5ac28d]::iter::traits::iterator::Iterator>::try_collect<rustc_index[3b6d27c33e9a86b4]::vec::IndexVec<rustc_middle[3debd01596c306bb]::mir::query::CoroutineSavedLocal, rustc_abi[b9b81dfdd0cd95d4]::layout::ty::TyAndLayout<rustc_middle[3debd01596c306bb]::ty::Ty>>>::{closure#0}, rustc_index[3b6d27c33e9a86b4]::vec::IndexVec<rustc_middle[3debd01596c306bb]::mir::query::CoroutineSavedLocal, rustc_abi[b9b81dfdd0cd95d4]::layout::ty::TyAndLayout<rustc_middle[3debd01596c306bb]::ty::Ty>>>
  50:        0x111164088 - rustc_ty_utils[f20e79acf91c0c4d]::layout::layout_of
  51:        0x1106d5b74 - rustc_query_impl[1913c4dc82019177]::plumbing::__rust_begin_short_backtrace::<rustc_query_impl[1913c4dc82019177]::query_impl::layout_of::dynamic_query::{closure#2}::{closure#0}, rustc_middle[3debd01596c306bb]::query::erase::Erased<[u8; 16usize]>>
  52:        0x1109e7f8c - <rustc_query_impl[1913c4dc82019177]::query_impl::layout_of::dynamic_query::{closure#2} as core[efaf33ed1a5ac28d]::ops::function::FnOnce<(rustc_middle[3debd01596c306bb]::ty::context::TyCtxt, rustc_middle[3debd01596c306bb]::ty::PseudoCanonicalInput<rustc_middle[3debd01596c306bb]::ty::Ty>)>>::call_once
  53:        0x11072b63c - rustc_query_system[55b68c9b31defbbe]::query::plumbing::try_execute_query::<rustc_query_impl[1913c4dc82019177]::DynamicConfig<rustc_query_system[55b68c9b31defbbe]::query::caches::DefaultCache<rustc_middle[3debd01596c306bb]::ty::PseudoCanonicalInput<rustc_middle[3debd01596c306bb]::ty::Ty>, rustc_middle[3debd01596c306bb]::query::erase::Erased<[u8; 16usize]>>, false, true, false>, rustc_query_impl[1913c4dc82019177]::plumbing::QueryCtxt, true>
  54:        0x11088eda8 - rustc_query_impl[1913c4dc82019177]::query_impl::layout_of::get_query_incr::__rust_end_short_backtrace
  55:        0x1111a9b5c - <rustc_middle[3debd01596c306bb]::ty::layout::LayoutCx as rustc_middle[3debd01596c306bb]::ty::layout::LayoutOf>::spanned_layout_of
  56:        0x11116d364 - <core[efaf33ed1a5ac28d]::iter::adapters::GenericShunt<core[efaf33ed1a5ac28d]::iter::adapters::by_ref_sized::ByRefSized<core[efaf33ed1a5ac28d]::iter::adapters::map::Map<core[efaf33ed1a5ac28d]::slice::iter::Iter<rustc_middle[3debd01596c306bb]::ty::VariantDef>, rustc_ty_utils[f20e79acf91c0c4d]::layout::layout_of_uncached::{closure#20}>>, core[efaf33ed1a5ac28d]::result::Result<core[efaf33ed1a5ac28d]::convert::Infallible, &rustc_middle[3debd01596c306bb]::ty::layout::LayoutError>> as core[efaf33ed1a5ac28d]::iter::traits::iterator::Iterator>::next
  57:        0x11112887c - core[efaf33ed1a5ac28d]::iter::adapters::try_process::<core[efaf33ed1a5ac28d]::iter::adapters::by_ref_sized::ByRefSized<core[efaf33ed1a5ac28d]::iter::adapters::map::Map<core[efaf33ed1a5ac28d]::slice::iter::Iter<rustc_middle[3debd01596c306bb]::ty::VariantDef>, rustc_ty_utils[f20e79acf91c0c4d]::layout::layout_of_uncached::{closure#20}>>, rustc_index[3b6d27c33e9a86b4]::vec::IndexVec<rustc_abi[b9b81dfdd0cd95d4]::layout::ty::FieldIdx, rustc_abi[b9b81dfdd0cd95d4]::layout::ty::TyAndLayout<rustc_middle[3debd01596c306bb]::ty::Ty>>, core[efaf33ed1a5ac28d]::result::Result<core[efaf33ed1a5ac28d]::convert::Infallible, &rustc_middle[3debd01596c306bb]::ty::layout::LayoutError>, <core[efaf33ed1a5ac28d]::iter::adapters::map::Map<core[efaf33ed1a5ac28d]::slice::iter::Iter<rustc_middle[3debd01596c306bb]::ty::VariantDef>, rustc_ty_utils[f20e79acf91c0c4d]::layout::layout_of_uncached::{closure#20}> as core[efaf33ed1a5ac28d]::iter::traits::iterator::Iterator>::try_collect<rustc_index[3b6d27c33e9a86b4]::vec::IndexVec<rustc_abi[b9b81dfdd0cd95d4]::layout::ty::VariantIdx, rustc_index[3b6d27c33e9a86b4]::vec::IndexVec<rustc_abi[b9b81dfdd0cd95d4]::layout::ty::FieldIdx, rustc_abi[b9b81dfdd0cd95d4]::layout::ty::TyAndLayout<rustc_middle[3debd01596c306bb]::ty::Ty>>>>::{closure#0}, rustc_index[3b6d27c33e9a86b4]::vec::IndexVec<rustc_abi[b9b81dfdd0cd95d4]::layout::ty::VariantIdx, rustc_index[3b6d27c33e9a86b4]::vec::IndexVec<rustc_abi[b9b81dfdd0cd95d4]::layout::ty::FieldIdx, rustc_abi[b9b81dfdd0cd95d4]::layout::ty::TyAndLayout<rustc_middle[3debd01596c306bb]::ty::Ty>>>>
  58:        0x111163ccc - rustc_ty_utils[f20e79acf91c0c4d]::layout::layout_of
  59:        0x1106d5b74 - rustc_query_impl[1913c4dc82019177]::plumbing::__rust_begin_short_backtrace::<rustc_query_impl[1913c4dc82019177]::query_impl::layout_of::dynamic_query::{closure#2}::{closure#0}, rustc_middle[3debd01596c306bb]::query::erase::Erased<[u8; 16usize]>>
  60:        0x1109e7f8c - <rustc_query_impl[1913c4dc82019177]::query_impl::layout_of::dynamic_query::{closure#2} as core[efaf33ed1a5ac28d]::ops::function::FnOnce<(rustc_middle[3debd01596c306bb]::ty::context::TyCtxt, rustc_middle[3debd01596c306bb]::ty::PseudoCanonicalInput<rustc_middle[3debd01596c306bb]::ty::Ty>)>>::call_once
  61:        0x11072b63c - rustc_query_system[55b68c9b31defbbe]::query::plumbing::try_execute_query::<rustc_query_impl[1913c4dc82019177]::DynamicConfig<rustc_query_system[55b68c9b31defbbe]::query::caches::DefaultCache<rustc_middle[3debd01596c306bb]::ty::PseudoCanonicalInput<rustc_middle[3debd01596c306bb]::ty::Ty>, rustc_middle[3debd01596c306bb]::query::erase::Erased<[u8; 16usize]>>, false, true, false>, rustc_query_impl[1913c4dc82019177]::plumbing::QueryCtxt, true>
  62:        0x11088eda8 - rustc_query_impl[1913c4dc82019177]::query_impl::layout_of::get_query_incr::__rust_end_short_backtrace
  63:        0x1111a9b5c - <rustc_middle[3debd01596c306bb]::ty::layout::LayoutCx as rustc_middle[3debd01596c306bb]::ty::layout::LayoutOf>::spanned_layout_of
  64:        0x11116d3f4 - <core[efaf33ed1a5ac28d]::iter::adapters::GenericShunt<core[efaf33ed1a5ac28d]::iter::adapters::by_ref_sized::ByRefSized<core[efaf33ed1a5ac28d]::iter::adapters::map::Map<core[efaf33ed1a5ac28d]::slice::iter::Iter<rustc_middle[3debd01596c306bb]::ty::VariantDef>, rustc_ty_utils[f20e79acf91c0c4d]::layout::layout_of_uncached::{closure#20}>>, core[efaf33ed1a5ac28d]::result::Result<core[efaf33ed1a5ac28d]::convert::Infallible, &rustc_middle[3debd01596c306bb]::ty::layout::LayoutError>> as core[efaf33ed1a5ac28d]::iter::traits::iterator::Iterator>::next
  65:        0x11112887c - core[efaf33ed1a5ac28d]::iter::adapters::try_process::<core[efaf33ed1a5ac28d]::iter::adapters::by_ref_sized::ByRefSized<core[efaf33ed1a5ac28d]::iter::adapters::map::Map<core[efaf33ed1a5ac28d]::slice::iter::Iter<rustc_middle[3debd01596c306bb]::ty::VariantDef>, rustc_ty_utils[f20e79acf91c0c4d]::layout::layout_of_uncached::{closure#20}>>, rustc_index[3b6d27c33e9a86b4]::vec::IndexVec<rustc_abi[b9b81dfdd0cd95d4]::layout::ty::FieldIdx, rustc_abi[b9b81dfdd0cd95d4]::layout::ty::TyAndLayout<rustc_middle[3debd01596c306bb]::ty::Ty>>, core[efaf33ed1a5ac28d]::result::Result<core[efaf33ed1a5ac28d]::convert::Infallible, &rustc_middle[3debd01596c306bb]::ty::layout::LayoutError>, <core[efaf33ed1a5ac28d]::iter::adapters::map::Map<core[efaf33ed1a5ac28d]::slice::iter::Iter<rustc_middle[3debd01596c306bb]::ty::VariantDef>, rustc_ty_utils[f20e79acf91c0c4d]::layout::layout_of_uncached::{closure#20}> as core[efaf33ed1a5ac28d]::iter::traits::iterator::Iterator>::try_collect<rustc_index[3b6d27c33e9a86b4]::vec::IndexVec<rustc_abi[b9b81dfdd0cd95d4]::layout::ty::VariantIdx, rustc_index[3b6d27c33e9a86b4]::vec::IndexVec<rustc_abi[b9b81dfdd0cd95d4]::layout::ty::FieldIdx, rustc_abi[b9b81dfdd0cd95d4]::layout::ty::TyAndLayout<rustc_middle[3debd01596c306bb]::ty::Ty>>>>::{closure#0}, rustc_index[3b6d27c33e9a86b4]::vec::IndexVec<rustc_abi[b9b81dfdd0cd95d4]::layout::ty::VariantIdx, rustc_index[3b6d27c33e9a86b4]::vec::IndexVec<rustc_abi[b9b81dfdd0cd95d4]::layout::ty::FieldIdx, rustc_abi[b9b81dfdd0cd95d4]::layout::ty::TyAndLayout<rustc_middle[3debd01596c306bb]::ty::Ty>>>>
  66:        0x111163ccc - rustc_ty_utils[f20e79acf91c0c4d]::layout::layout_of
  67:        0x1106d5b74 - rustc_query_impl[1913c4dc82019177]::plumbing::__rust_begin_short_backtrace::<rustc_query_impl[1913c4dc82019177]::query_impl::layout_of::dynamic_query::{closure#2}::{closure#0}, rustc_middle[3debd01596c306bb]::query::erase::Erased<[u8; 16usize]>>
  68:        0x1109e7f8c - <rustc_query_impl[1913c4dc82019177]::query_impl::layout_of::dynamic_query::{closure#2} as core[efaf33ed1a5ac28d]::ops::function::FnOnce<(rustc_middle[3debd01596c306bb]::ty::context::TyCtxt, rustc_middle[3debd01596c306bb]::ty::PseudoCanonicalInput<rustc_middle[3debd01596c306bb]::ty::Ty>)>>::call_once
  69:        0x11072b63c - rustc_query_system[55b68c9b31defbbe]::query::plumbing::try_execute_query::<rustc_query_impl[1913c4dc82019177]::DynamicConfig<rustc_query_system[55b68c9b31defbbe]::query::caches::DefaultCache<rustc_middle[3debd01596c306bb]::ty::PseudoCanonicalInput<rustc_middle[3debd01596c306bb]::ty::Ty>, rustc_middle[3debd01596c306bb]::query::erase::Erased<[u8; 16usize]>>, false, true, false>, rustc_query_impl[1913c4dc82019177]::plumbing::QueryCtxt, true>
  70:        0x11088eda8 - rustc_query_impl[1913c4dc82019177]::query_impl::layout_of::get_query_incr::__rust_end_short_backtrace
  71:        0x1111a9b5c - <rustc_middle[3debd01596c306bb]::ty::layout::LayoutCx as rustc_middle[3debd01596c306bb]::ty::layout::LayoutOf>::spanned_layout_of
  72:        0x11116d5bc - <core[efaf33ed1a5ac28d]::iter::adapters::GenericShunt<core[efaf33ed1a5ac28d]::iter::adapters::by_ref_sized::ByRefSized<core[efaf33ed1a5ac28d]::iter::adapters::map::Map<core[efaf33ed1a5ac28d]::slice::iter::Iter<rustc_middle[3debd01596c306bb]::mir::query::CoroutineSavedTy>, rustc_ty_utils[f20e79acf91c0c4d]::layout::layout_of_uncached::{closure#12}>>, core[efaf33ed1a5ac28d]::result::Result<core[efaf33ed1a5ac28d]::convert::Infallible, &rustc_middle[3debd01596c306bb]::ty::layout::LayoutError>> as core[efaf33ed1a5ac28d]::iter::traits::iterator::Iterator>::next
  73:        0x111128ab0 - core[efaf33ed1a5ac28d]::iter::adapters::try_process::<core[efaf33ed1a5ac28d]::iter::adapters::by_ref_sized::ByRefSized<core[efaf33ed1a5ac28d]::iter::adapters::map::Map<core[efaf33ed1a5ac28d]::slice::iter::Iter<rustc_middle[3debd01596c306bb]::mir::query::CoroutineSavedTy>, rustc_ty_utils[f20e79acf91c0c4d]::layout::layout_of_uncached::{closure#12}>>, rustc_abi[b9b81dfdd0cd95d4]::layout::ty::TyAndLayout<rustc_middle[3debd01596c306bb]::ty::Ty>, core[efaf33ed1a5ac28d]::result::Result<core[efaf33ed1a5ac28d]::convert::Infallible, &rustc_middle[3debd01596c306bb]::ty::layout::LayoutError>, <core[efaf33ed1a5ac28d]::iter::adapters::map::Map<core[efaf33ed1a5ac28d]::slice::iter::Iter<rustc_middle[3debd01596c306bb]::mir::query::CoroutineSavedTy>, rustc_ty_utils[f20e79acf91c0c4d]::layout::layout_of_uncached::{closure#12}> as core[efaf33ed1a5ac28d]::iter::traits::iterator::Iterator>::try_collect<rustc_index[3b6d27c33e9a86b4]::vec::IndexVec<rustc_middle[3debd01596c306bb]::mir::query::CoroutineSavedLocal, rustc_abi[b9b81dfdd0cd95d4]::layout::ty::TyAndLayout<rustc_middle[3debd01596c306bb]::ty::Ty>>>::{closure#0}, rustc_index[3b6d27c33e9a86b4]::vec::IndexVec<rustc_middle[3debd01596c306bb]::mir::query::CoroutineSavedLocal, rustc_abi[b9b81dfdd0cd95d4]::layout::ty::TyAndLayout<rustc_middle[3debd01596c306bb]::ty::Ty>>>
  74:        0x111164088 - rustc_ty_utils[f20e79acf91c0c4d]::layout::layout_of
  75:        0x1106d5b74 - rustc_query_impl[1913c4dc82019177]::plumbing::__rust_begin_short_backtrace::<rustc_query_impl[1913c4dc82019177]::query_impl::layout_of::dynamic_query::{closure#2}::{closure#0}, rustc_middle[3debd01596c306bb]::query::erase::Erased<[u8; 16usize]>>
  76:        0x1109e7f8c - <rustc_query_impl[1913c4dc82019177]::query_impl::layout_of::dynamic_query::{closure#2} as core[efaf33ed1a5ac28d]::ops::function::FnOnce<(rustc_middle[3debd01596c306bb]::ty::context::TyCtxt, rustc_middle[3debd01596c306bb]::ty::PseudoCanonicalInput<rustc_middle[3debd01596c306bb]::ty::Ty>)>>::call_once
  77:        0x11072b63c - rustc_query_system[55b68c9b31defbbe]::query::plumbing::try_execute_query::<rustc_query_impl[1913c4dc82019177]::DynamicConfig<rustc_query_system[55b68c9b31defbbe]::query::caches::DefaultCache<rustc_middle[3debd01596c306bb]::ty::PseudoCanonicalInput<rustc_middle[3debd01596c306bb]::ty::Ty>, rustc_middle[3debd01596c306bb]::query::erase::Erased<[u8; 16usize]>>, false, true, false>, rustc_query_impl[1913c4dc82019177]::plumbing::QueryCtxt, true>
  78:        0x11088eda8 - rustc_query_impl[1913c4dc82019177]::query_impl::layout_of::get_query_incr::__rust_end_short_backtrace
  79:        0x10faeb4bc - <rustc_middle[3debd01596c306bb]::ty::context::TyCtxt>::par_hir_body_owners::<rustc_interface[4523f22467e79f56]::passes::run_required_analyses::{closure#1}::{closure#0}>::{closure#0}
  80:        0x10fb33fec - rustc_interface[4523f22467e79f56]::passes::analysis
  81:        0x1106d5820 - rustc_query_impl[1913c4dc82019177]::plumbing::__rust_begin_short_backtrace::<rustc_query_impl[1913c4dc82019177]::query_impl::analysis::dynamic_query::{closure#2}::{closure#0}, rustc_middle[3debd01596c306bb]::query::erase::Erased<[u8; 0usize]>>
  82:        0x11071c708 - rustc_query_system[55b68c9b31defbbe]::query::plumbing::try_execute_query::<rustc_query_impl[1913c4dc82019177]::DynamicConfig<rustc_query_system[55b68c9b31defbbe]::query::caches::SingleCache<rustc_middle[3debd01596c306bb]::query::erase::Erased<[u8; 0usize]>>, false, false, false>, rustc_query_impl[1913c4dc82019177]::plumbing::QueryCtxt, true>
  83:        0x11088d204 - rustc_query_impl[1913c4dc82019177]::query_impl::analysis::get_query_incr::__rust_end_short_backtrace
  84:        0x10f1f1560 - rustc_interface[4523f22467e79f56]::passes::create_and_enter_global_ctxt::<core[efaf33ed1a5ac28d]::option::Option<rustc_interface[4523f22467e79f56]::queries::Linker>, rustc_driver_impl[50cf2a748b5c59a1]::run_compiler::{closure#0}::{closure#2}>
  85:        0x10f23c174 - rustc_interface[4523f22467e79f56]::interface::run_compiler::<(), rustc_driver_impl[50cf2a748b5c59a1]::run_compiler::{closure#0}>::{closure#1}
  86:        0x10f230734 - std[dc0ae436bb89db06]::sys::backtrace::__rust_begin_short_backtrace::<rustc_interface[4523f22467e79f56]::util::run_in_thread_with_globals<rustc_interface[4523f22467e79f56]::util::run_in_thread_pool_with_globals<rustc_interface[4523f22467e79f56]::interface::run_compiler<(), rustc_driver_impl[50cf2a748b5c59a1]::run_compiler::{closure#0}>::{closure#1}, ()>::{closure#0}, ()>::{closure#0}::{closure#0}, ()>
  87:        0x10f242970 - <<std[dc0ae436bb89db06]::thread::Builder>::spawn_unchecked_<rustc_interface[4523f22467e79f56]::util::run_in_thread_with_globals<rustc_interface[4523f22467e79f56]::util::run_in_thread_pool_with_globals<rustc_interface[4523f22467e79f56]::interface::run_compiler<(), rustc_driver_impl[50cf2a748b5c59a1]::run_compiler::{closure#0}>::{closure#1}, ()>::{closure#0}, ()>::{closure#0}::{closure#0}, ()>::{closure#1} as core[efaf33ed1a5ac28d]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  88:        0x111215190 - std::sys::thread::unix::Thread::new::thread_start::h98270432b6aefc44
  89:        0x18a0efbc8 - __pthread_cond_wait

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.92.0 (ded5c06cf 2025-12-08) running on aarch64-apple-darwin

note: compiler flags: --crate-type bin -C embed-bitcode=no -C debuginfo=2 -C split-debuginfo=unpacked -C incremental=[REDACTED]

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [optimized_mir] optimizing MIR for `repro::serve_request::{closure#0}`
#1 [layout_of] computing layout of `{async fn body of repro::serve_request()}`
#2 [layout_of] computing layout of `core::mem::manually_drop::ManuallyDrop<{async fn body of repro::serve_request()}>`
#3 [layout_of] computing layout of `core::mem::maybe_uninit::MaybeUninit<{async fn body of repro::serve_request()}>`
#4 [layout_of] computing layout of `{async block@src/main.rs:44:26: 44:36}`
#5 [layout_of] computing layout of `core::mem::manually_drop::ManuallyDrop<{async block@src/main.rs:44:26: 44:36}>`
#6 [layout_of] computing layout of `core::mem::maybe_uninit::MaybeUninit<{async block@src/main.rs:44:26: 44:36}>`
#7 [layout_of] computing layout of `{async fn body of repro::handle_connection()}`
#8 [analysis] running analysis passes on this crate
end of query stack
error: future cannot be shared between threads safely
  --> src/main.rs:67:72
   |
67 |         let _target: Pin<Box<dyn Future<Output = ()> + Send + Sync>> = tunnel_future;
   |                                                                        ^^^^^^^^^^^^^ future created by async block is not `Sync`
   |
   = help: the trait `Sync` is not implemented for `dyn Future<Output = Result<Upgraded, ()>> + Send`
note: captured value is not `Sync`
  --> src/main.rs:60:36
   |
60 |             match tokio::try_join!(client, upstream) {
   |                                    ^^^^^^ has type `Pin<Box<dyn Future<Output = Result<Upgraded, ()>> + Send>>` which is not `Sync`
   = note: required for the cast from `Pin<Box<{async block@src/main.rs:58:38: 58:48}>>` to `Pin<Box<dyn Future<Output = ()> + Send + Sync>>`

error: future cannot be shared between threads safely
  --> src/main.rs:67:72
   |
67 |         let _target: Pin<Box<dyn Future<Output = ()> + Send + Sync>> = tunnel_future;
   |                                                                        ^^^^^^^^^^^^^ future created by async block is not `Sync`
   |
   = help: the trait `Sync` is not implemented for `(dyn AsyncReadWrite + Send + 'static)`
note: future is not `Sync` as this value is used across an await
  --> src/main.rs:62:81
   |
61 |                 Ok((mut c_io, mut u_io)) => {
   |                     -------- has type `Upgraded` which is not `Sync`
62 |                     let _ = tokio::io::copy_bidirectional(&mut c_io, &mut u_io).await;
   |                                                                                 ^^^^^ await occurs here, with `mut c_io` maybe used later
   = note: required for the cast from `Pin<Box<{async block@src/main.rs:58:38: 58:48}>>` to `Pin<Box<dyn Future<Output = ()> + Send + Sync>>`

error: could not compile `vane` (bin "vane") due to 2 previous errors
canmi@xyy ~/C/P/future_sync_ice (main) [101]>
```

## Location

The ICE is triggered at the very end of `serve_request` where an `unsize coercion` is attempted:

```rust
// The `tunnel_future` captures variables that are `!Sync`.
// We attempt to coerce it into a trait object that requires `Sync`.
// Instead of a standard compile error, this causes the compiler to crash during MIR validation.
let _target: Pin<Box<dyn Future<Output = ()> + Send + Sync>> = tunnel_future;
```

## Notes

During code-minimize process, I identified specific conditions required to sustain the ICE.

### 1. The Core Logic
The root cause involves type inference regarding thread safety (`Sync`):
*   **The Type:** We define `Upgraded`, which contains `Box<dyn AsyncReadWrite + Send>`. This makes `Upgraded` **`Send` but `!Sync`**.
*   **The Capture:** The `tunnel_future` async block captures two instances of `Upgraded`.
*   **The Await:** Inside `tunnel_future`, we use `tokio::try_join!` and `tokio::io::copy_bidirectional`. These hold the `!Sync` `Upgraded` instances across an `.await` point.
*   **The Consequence:** This makes the resulting `tunnel_future` itself `!Sync`.
*   **The Trigger:** Coercing this `!Sync` future to `Pin<Box<dyn Future + ... + Sync>>`.

### 2. Load-Bearing Structures
Surprisingly, the ICE is fragile regarding the code structure. I attempted to simplify the following but found them **necessary** to trigger the crash:

*   **Tokio Dependency:** I attempted to replace `tokio` with minimal local mocks of `AsyncRead`/`AsyncWrite` and `try_join`. However, the zero-dependency version **failed** to reproduce the ICE. The specific implementation of `tokio`'s IO traits or macros seems relevant to the generated MIR.
*   **Module Nesting (`mod repro`):** The code **must** be wrapped in at least one module layer (`pub(crate) mod repro`). Moving the logic to the crate root (`src/main.rs` top level) causes the ICE to disappear.
*   **Async Closure Wrapper:** The `handle_connection` function is required. Specifically, wrapping the call to `serve_request` inside an async closure:
    ```rust
    // Removing this wrapper and calling serve_request directly prevents the ICE.
    let service = || async move { serve_request().await };
    let _ = service().await;
    ```
