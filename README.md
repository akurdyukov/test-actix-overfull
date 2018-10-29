# test-actix-overfull

Simple demo for crashing Actix with

```
 INFO 2018-10-29T13:55:50Z: test_actix_overfull: Supervisor accepted validating command Command { user_id: 3, payload: 253 }
 INFO 2018-10-29T13:55:50Z: test_actix_overfull: Supervisor accepted validating command Command { user_id: 4, payload: 254 }
 INFO 2018-10-29T13:55:50Z: test_actix_overfull: Supervisor accepted validating command Command { user_id: 0, payload: 255 }
thread 'main' panicked at 'Use Self::Context::notify() instead of direct use of address', /Users/alik/.cargo/registry/src/github.com-1ecc6299db9ec823/actix-0.7.5/src/mailbox.rs:103:17
note: Run with `RUST_BACKTRACE=1` for a backtrace.
Panic in Arbiter thread, shutting down system.
```
