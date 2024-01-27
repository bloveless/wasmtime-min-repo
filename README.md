# Minimum Reproduction of failure to include go module in wasmtime

The only thing that is requred to reproduce the error is to run make.

```
make
```

Current versions

```
cargo --version
cargo 1.77.0-nightly (7bb7b5395 2024-01-20)

rustc --version
rustc 1.77.0-nightly (5bd5d214e 2024-01-25)

go version
go version go1.21.4 darwin/arm64
```

And for quick reference this is the error that I'm seeing

```
rm -rf ./build/examples/hello-world/go-hello-world.wasm
GOOS=wasip1 GOARCH=wasm go build -o ./build/examples/hello-world/go-hello-world.wasm ./examples/hello-world/main.go
RUST_BACKTRACE=full cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.18s
     Running `target/debug/wasmtime-min-repo`
1 7
2 9
3 4161
4 4163
Hello world!
Error: error while executing at wasm backtrace:
    0: 0x79824 - <unknown>!runtime.exit
    1: 0x86e48 - <unknown>!runtime.main
    2: 0xe735c - <unknown>!wasm_pc_f_loop
    3: 0xe73f4 - <unknown>!_rt0_wasm_wasip1

Caused by:
    Exited with i32 exit status 0

Stack backtrace:
   0: std::backtrace_rs::backtrace::libunwind::trace
             at /rustc/5bd5d214effd494f4bafb29b3a7a2f6c2070ca5c/library/std/src/../../backtrace/src/backtrace/libunwind.rs:104:5
   1: std::backtrace_rs::backtrace::trace_unsynchronized
             at /rustc/5bd5d214effd494f4bafb29b3a7a2f6c2070ca5c/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2: std::backtrace::Backtrace::create
             at /rustc/5bd5d214effd494f4bafb29b3a7a2f6c2070ca5c/library/std/src/backtrace.rs:331:13
   3: anyhow::error::<impl core::convert::From<E> for anyhow::Error>::from
             at /Users/brennon/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.79/src/error.rs:565:25
   4: <T as core::convert::Into<U>>::into
             at /rustc/5bd5d214effd494f4bafb29b3a7a2f6c2070ca5c/library/core/src/convert/mod.rs:758:9
   5: wasi_common::snapshots::preview_1::<impl wasi_common::snapshots::preview_1::wasi_snapshot_preview1::WasiSnapshotPreview1 for wasi_common::ctx::WasiCtx>::proc_exit::{{closure}}
             at /Users/brennon/.cargo/registry/src/index.crates.io-6f17d22bba15001f/wasi-common-17.0.0/src/snapshots/preview_1.rs:1064:13
   6: <core::pin::Pin<P> as core::future::future::Future>::poll
             at /rustc/5bd5d214effd494f4bafb29b3a7a2f6c2070ca5c/library/core/src/future/future.rs:124:9
   7: wasi_common::snapshots::preview_1::wasi_snapshot_preview1::proc_exit::{{closure}}
             at /Users/brennon/.cargo/registry/src/index.crates.io-6f17d22bba15001f/wasi-common-17.0.0/src/snapshots/preview_1.rs:27:1
   8: <tracing::instrument::Instrumented<T> as core::future::future::Future>::poll
             at /Users/brennon/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tracing-0.1.40/src/instrument.rs:321:9
   9: wasmtime_wasi::sync::snapshots::preview_1::add_wasi_snapshot_preview1_to_linker::{{closure}}::{{closure}}
             at /Users/brennon/.cargo/registry/src/index.crates.io-6f17d22bba15001f/wasmtime-wasi-17.0.0/src/lib.rs:66:9
  10: wiggle::run_in_dummy_executor
             at /Users/brennon/.cargo/registry/src/index.crates.io-6f17d22bba15001f/wiggle-17.0.0/src/lib.rs:1171:11
  11: wasmtime_wasi::sync::snapshots::preview_1::add_wasi_snapshot_preview1_to_linker::{{closure}}
             at /Users/brennon/.cargo/registry/src/index.crates.io-6f17d22bba15001f/wasmtime-wasi-17.0.0/src/lib.rs:66:9
  12: <F as wasmtime::func::IntoFunc<T,(wasmtime::func::Caller<T>,A1),R>>::into_func::native_call_shim::{{closure}}::{{closure}}
             at /Users/brennon/.cargo/registry/src/index.crates.io-6f17d22bba15001f/wasmtime-17.0.0/src/func.rs:1974:41
  13: core::ops::function::FnOnce::call_once
             at /rustc/5bd5d214effd494f4bafb29b3a7a2f6c2070ca5c/library/core/src/ops/function.rs:250:5
  14: <core::panic::unwind_safe::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
             at /rustc/5bd5d214effd494f4bafb29b3a7a2f6c2070ca5c/library/core/src/panic/unwind_safe.rs:272:9
  15: std::panicking::try::do_call
             at /rustc/5bd5d214effd494f4bafb29b3a7a2f6c2070ca5c/library/std/src/panicking.rs:554:40
  16: ___rust_try
  17: std::panicking::try
             at /rustc/5bd5d214effd494f4bafb29b3a7a2f6c2070ca5c/library/std/src/panicking.rs:518:19
  18: std::panic::catch_unwind
             at /rustc/5bd5d214effd494f4bafb29b3a7a2f6c2070ca5c/library/std/src/panic.rs:142:14
  19: <F as wasmtime::func::IntoFunc<T,(wasmtime::func::Caller<T>,A1),R>>::into_func::native_call_shim::{{closure}}
             at /Users/brennon/.cargo/registry/src/index.crates.io-6f17d22bba15001f/wasmtime-17.0.0/src/func.rs:1969:29
  20: wasmtime::func::Caller<T>::with::{{closure}}
             at /Users/brennon/.cargo/registry/src/index.crates.io-6f17d22bba15001f/wasmtime-17.0.0/src/func.rs:1786:13
  21: wasmtime_runtime::instance::Instance::from_vmctx
             at /Users/brennon/.cargo/registry/src/index.crates.io-6f17d22bba15001f/wasmtime-runtime-17.0.0/src/instance.rs:240:9
  22: wasmtime::func::Caller<T>::with
             at /Users/brennon/.cargo/registry/src/index.crates.io-6f17d22bba15001f/wasmtime-17.0.0/src/func.rs:1784:9
  23: <F as wasmtime::func::IntoFunc<T,(wasmtime::func::Caller<T>,A1),R>>::into_func::native_call_shim
             at /Users/brennon/.cargo/registry/src/index.crates.io-6f17d22bba15001f/wasmtime-17.0.0/src/func.rs:1958:34
  24: <unknown>
make: *** [runhost] Error 1
```
