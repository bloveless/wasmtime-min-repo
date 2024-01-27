use std::time::Instant;
use wasmtime::{Engine, Linker, Module, Store};
use wasmtime_wasi::WasiCtxBuilder;
use anyhow::Result;

fn main() -> Result<()> {
    let start_time = Instant::now();

    // Define the WASI functions globally on the `Config`.
    let engine = Engine::default();
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |s| s)?;

    println!("1 {}", start_time.elapsed().as_millis());

    // Create a WASI context and put it in a Store; all instances in the store
    // share this context. `WasiCtxBuilder` provides a number of ways to
    // configure what the target program will have access to.
    let wasi = WasiCtxBuilder::new()
        .inherit_stdio()
        .inherit_args()?
        .build();
    let mut store = Store::new(&engine, wasi);

    println!("2 {}", start_time.elapsed().as_millis());

    // Instantiate our module with the imports we've created, and run it.
    let module = Module::from_file(&engine, "./build/examples/hello-world/go-hello-world.wasm")?;

    println!("3 {}", start_time.elapsed().as_millis());

    linker.module(&mut store, "", &module)?;

    println!("4 {}", start_time.elapsed().as_millis());

    linker
        .get_default(&mut store, "")?
        .typed::<(), ()>(&store)?
        .call(&mut store, ())?;

    println!("5 {}", start_time.elapsed().as_millis());

    Ok(())
}
