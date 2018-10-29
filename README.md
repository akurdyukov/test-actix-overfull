# test-actix-overfull

Simple demo for crashing Actix with

```
 INFO 2018-10-29T14:15:38Z: test_actix_overfull: Supervisor accepted validating command Command { user_id: 3, payload: 253 }
 INFO 2018-10-29T14:15:38Z: test_actix_overfull: Supervisor accepted validating command Command { user_id: 4, payload: 254 }
 INFO 2018-10-29T14:15:38Z: test_actix_overfull: Supervisor accepted validating command Command { user_id: 0, payload: 255 }
thread 'main' panicked at 'Use Self::Context::notify() instead of direct use of address', /Users/alik/.cargo/registry/src/github.com-1ecc6299db9ec823/actix-0.7.5/src/mailbox.rs:103:17
note: Run with `RUST_BACKTRACE=1` for a backtrace.
Panic in Arbiter thread, shutting down system.
```

Full stacktrace:
```
 INFO 2018-10-29T14:22:04Z: test_actix_overfull: Supervisor accepted validating command Command { user_id: 4, payload: 254 }
 INFO 2018-10-29T14:22:04Z: test_actix_overfull: Supervisor accepted validating command Command { user_id: 0, payload: 255 }
thread 'main' panicked at 'Use Self::Context::notify() instead of direct use of address', /Users/alik/.cargo/registry/src/github.com-1ecc6299db9ec823/actix-0.7.5/src/mailbox.rs:103:17
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
             at libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1: std::sys_common::backtrace::print
             at libstd/sys_common/backtrace.rs:71
             at libstd/sys_common/backtrace.rs:59
   2: std::panicking::default_hook::{{closure}}
             at libstd/panicking.rs:211
   3: std::panicking::default_hook
             at libstd/panicking.rs:227
   4: <std::panicking::begin_panic::PanicPayload<A> as core::panic::BoxMeUp>::get
             at libstd/panicking.rs:477
   5: core::ptr::swap_nonoverlapping_bytes
             at libstd/panicking.rs:411
   6: <actix::mailbox::Mailbox<A>>::poll
             at ./<::std::macros::panic macros>:3
   7: <actix::contextimpl::ContextFut<A, C> as futures::future::Future>::poll
             at /Users/alik/.cargo/registry/src/github.com-1ecc6299db9ec823/actix-0.7.5/src/contextimpl.rs:346
   8: core::fmt::num::<impl core::fmt::Debug for u64>::fmt
             at /Users/alik/.cargo/registry/src/github.com-1ecc6299db9ec823/futures-0.1.25/src/future/mod.rs:113
   9: core::fmt::num::<impl core::fmt::Debug for u64>::fmt
             at /Users/alik/.cargo/registry/src/github.com-1ecc6299db9ec823/futures-0.1.25/src/future/mod.rs:113
  10: core::cmp::impls::<impl core::cmp::PartialOrd for isize>::le
             at /Users/alik/.cargo/registry/src/github.com-1ecc6299db9ec823/futures-0.1.25/src/task_impl/mod.rs:326
  11: core::cmp::impls::<impl core::cmp::PartialOrd for isize>::le
             at /Users/alik/.cargo/registry/src/github.com-1ecc6299db9ec823/futures-0.1.25/src/task_impl/mod.rs:396
  12: futures::task_impl::std::BorrowedUnpark::new
             at /Users/alik/.cargo/registry/src/github.com-1ecc6299db9ec823/futures-0.1.25/src/task_impl/std/mod.rs:78
  13: core::cmp::impls::<impl core::cmp::PartialOrd for isize>::le
             at /Users/alik/.cargo/registry/src/github.com-1ecc6299db9ec823/futures-0.1.25/src/task_impl/mod.rs:396
  14: core::cmp::impls::<impl core::cmp::PartialOrd for isize>::le
             at /Users/alik/.cargo/registry/src/github.com-1ecc6299db9ec823/futures-0.1.25/src/task_impl/mod.rs:288
  15: core::cmp::impls::<impl core::cmp::PartialOrd for isize>::le
             at /Users/alik/.cargo/registry/src/github.com-1ecc6299db9ec823/futures-0.1.25/src/task_impl/mod.rs:326
  16: <core::cmp::Ordering as core::cmp::PartialEq>::eq
             at /Users/alik/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-current-thread-0.1.3/src/scheduler.rs:354
  17: <core::cmp::Ordering as core::cmp::PartialEq>::eq
             at /Users/alik/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-current-thread-0.1.3/src/scheduler.rs:333
  18: <core::cmp::Ordering as core::cmp::PartialEq>::eq
             at /Users/alik/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-current-thread-0.1.3/src/lib.rs:779
  19: tokio_current_thread::EXECUTOR_ID::__init
             at /Users/alik/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-current-thread-0.1.3/src/lib.rs:816
  20: <core::cmp::Ordering as core::cmp::PartialEq>::eq
             at /Users/alik/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-current-thread-0.1.3/src/lib.rs:778
  21: core::cmp::impls::<impl core::cmp::PartialOrd for isize>::le
             at libstd/thread/local.rs:294
  22: core::cmp::impls::<impl core::cmp::PartialOrd for isize>::le
             at libstd/thread/local.rs:248
  23: <core::cmp::Ordering as core::cmp::PartialEq>::eq
             at /Users/alik/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-current-thread-0.1.3/src/lib.rs:776
  24: <core::cmp::Ordering as core::cmp::PartialEq>::eq
             at /Users/alik/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-current-thread-0.1.3/src/scheduler.rs:333
  25: <core::cmp::Ordering as core::cmp::PartialEq>::eq
             at /Users/alik/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-current-thread-0.1.3/src/lib.rs:605
  26: alloc::alloc::realloc
             at /Users/alik/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-current-thread-0.1.3/src/lib.rs:488
  27: tokio_executor::global::EXECUTOR::__init
             at /Users/alik/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-0.1.11/src/runtime/current_thread/runtime.rs:187
  28: tokio_executor::global::EXECUTOR::__init
             at /Users/alik/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-0.1.11/src/runtime/current_thread/runtime.rs:228
  29: actix::system::CURRENT::__init
             at /Users/alik/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-executor-0.1.5/src/global.rs:192
  30: actix::arbiter::Q::__init
             at libstd/thread/local.rs:294
  31: actix::arbiter::Q::__init
             at libstd/thread/local.rs:248
  32: actix::system::CURRENT::__init
             at /Users/alik/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-executor-0.1.5/src/global.rs:162
  33: tokio_executor::global::EXECUTOR::__init
             at /Users/alik/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-0.1.11/src/runtime/current_thread/runtime.rs:226
  34: <core::cell::BorrowRefMut<'b> as core::ops::drop::Drop>::drop
             at /Users/alik/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-timer-0.2.7/src/timer/handle.rs:94
  35: actix::arbiter::Q::__init
             at libstd/thread/local.rs:294
  36: actix::arbiter::Q::__init
             at libstd/thread/local.rs:248
  37: <core::cell::BorrowRefMut<'b> as core::ops::drop::Drop>::drop
             at /Users/alik/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-timer-0.2.7/src/timer/handle.rs:81
  38: tokio_executor::global::EXECUTOR::__init
             at /Users/alik/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-0.1.11/src/runtime/current_thread/runtime.rs:219
  39: <alloc::collections::CollectionAllocErr as core::convert::From<core::alloc::LayoutErr>>::from
             at /Users/alik/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-timer-0.2.7/src/clock/clock.rs:136
  40: actix::arbiter::Q::__init
             at libstd/thread/local.rs:294
  41: actix::arbiter::Q::__init
             at libstd/thread/local.rs:248
  42: <alloc::collections::CollectionAllocErr as core::convert::From<core::alloc::LayoutErr>>::from
             at /Users/alik/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-timer-0.2.7/src/clock/clock.rs:119
  43: tokio_executor::global::EXECUTOR::__init
             at /Users/alik/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-0.1.11/src/runtime/current_thread/runtime.rs:218
  44: alloc::raw_vec::alloc_guard
             at /Users/alik/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-reactor-0.1.6/src/lib.rs:229
  45: actix::arbiter::Q::__init
             at libstd/thread/local.rs:294
  46: actix::arbiter::Q::__init
             at libstd/thread/local.rs:248
  47: alloc::raw_vec::alloc_guard
             at /Users/alik/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-reactor-0.1.6/src/lib.rs:212
  48: tokio_executor::global::EXECUTOR::__init
             at /Users/alik/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-0.1.11/src/runtime/current_thread/runtime.rs:217
  49: tokio_executor::global::EXECUTOR::__init
             at /Users/alik/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-0.1.11/src/runtime/current_thread/runtime.rs:185
  50: actix::system::System::stop_with_code
             at /Users/alik/.cargo/registry/src/github.com-1ecc6299db9ec823/actix-0.7.5/src/system.rs:174
  51: actix::system::Builder::run
             at /Users/alik/.cargo/registry/src/github.com-1ecc6299db9ec823/actix-0.7.5/src/system.rs:366
  52: actix::system::System::run
             at /Users/alik/.cargo/registry/src/github.com-1ecc6299db9ec823/actix-0.7.5/src/system.rs:151
  53: test_actix_overfull::main
             at src/main.rs:69
  54: std::rt::lang_start::{{closure}}
             at libstd/rt.rs:74
  55: std::panicking::try::do_call
             at libstd/rt.rs:59
             at libstd/panicking.rs:310
  56: macho_symbol_search
             at libpanic_unwind/lib.rs:103
  57: std::sys_common::bytestring::debug_fmt_bytestring
             at libstd/panicking.rs:289
             at libstd/panic.rs:392
             at libstd/rt.rs:58
  58: std::rt::lang_start
             at libstd/rt.rs:74
  59: <test_actix_overfull::Command as core::fmt::Debug>::fmt
Panic in Arbiter thread, shutting down system.
```