#![allow(unused_variables, dead_code)]

wit_bindgen_wasmtime::import!("../wits/say.wit");

use anyhow::Result;
use wit_bindgen_wasmtime::wasmtime::{self, Config, Engine, Instance, Linker, Module, Store};

fn main() {
    let path = if cfg!(not(debug_assertions)) {
        "./target/wasm32-wasi/release/guest.wasm"
    } else {
        "./target/wasm32-wasi/debug/guest.wasm"
    };
    let path2 = if cfg!(not(debug_assertions)) {
        "./target/wasm32-wasi/release/guest2.wasm"
    } else {
        "./target/wasm32-wasi/debug/guest2.wasm"
    };

    println!("\n\n\n\n\n");
    println!("Running from the guest wasm");
    run(path, "Michael");
    run(path, "Douglas");

    println!("");
    println!("Running from the unhospitable guest wasm (guest2)");
    run(path2, "Michael");
    run(path2, "Douglas");

    #[cfg(not(debug_assertions))]
    {
        println!("\n\n\n\n\n");
        println!("Lets do some poor-man's benchmarks");
        timeit(path, "Michael", 10_000_000);
    }
}

fn run(path: &str, name: &str) {
    use say::{Say, SayData};
    //type SayLinker = Linker<Context<SayData, SayData>>;
    type SayStore = Store<Context<SayData, SayData>>;

    // LEAVE THIS FOR FUTURE LEARNINGS
    //let add_to_linker = |linker: &mut SayLinker| {
    //    Say::add_to_linker(linker, |cx| -> &mut SayData { &mut cx.imports })
    //};
    //
    let funcs = instantiate(path, |store: &mut SayStore, module, linker| {
        Say::instantiate(store, module, linker, |cx| &mut cx.exports)
    });

    if let Ok((exports, mut store)) = funcs {
        match exports.hello(&mut store, name) {
            Ok(reply) => println!("{}", reply),
            Err(e) => println!("Error: {}", e),
        }
    } else {
        println!("no instantiate");
    }
}

fn timeit(path: &str, name: &str, amt: u128) {
    use say::{Say, SayData};
    type SayStore = Store<Context<SayData, SayData>>;

    // LEAVE THIS FOR FUTURE LEARNINGS
    //type SayLinker = Linker<Context<SayData, SayData>>;
    //let add_to_linker = |linker: &mut SayLinker| {
    //    Say::add_to_linker(linker, |cx| -> &mut SayData { &mut cx.imports })
    //};
    //
    if let Ok((exports, mut store)) = instantiate(path, |store: &mut SayStore, module, linker| {
        Say::instantiate(store, module, linker, |cx| &mut cx.exports)
    }) {
        let s = std::time::Instant::now();
        let mut wasm_time_count: u128 = 0;
        let mut native_time_count: u128 = 0;
        for _ in 0..amt {
            match exports.overhead(&mut store, name) {
                Ok((_reply, t)) => {
                    wasm_time_count += t as u128;
                }
                Err(_) => {}
            }

            /*match exports.hello(&mut store, name) {*/
            /*Ok(_reply) => {}*/
            /*Err(_) => {}*/
            /*}*/
        }
        let wasm_time = s.elapsed().as_nanos();

        let s1 = std::time::Instant::now();
        for _ in 0..amt {
            //let _reply = native_hello("Michael".to_string());
            let (_reply, t) = native_overhead("Michael".to_string());
            native_time_count += t as u128;
        }
        let native_time = s1.elapsed().as_nanos();

        println!("");
        println!(
            "{:>13}ns to run wasm   -- {:>6} nanos per iteration",
            wasm_time,
            wasm_time / amt
        );
        println!(
            "{:>13}ns to run native -- {:>6} nanos per iteration",
            native_time,
            native_time / amt
        );
        println!(
            "{:>13}ns inside wasm   -- {:>6} nanos per iteration",
            wasm_time_count,
            wasm_time_count / amt
        );
        println!(
            "{:>13}ns inside native -- {:>6} nanos per iteration",
            native_time_count,
            native_time_count / amt
        );
        println!("");
        println!(
            "full running of plugin is {} times slower",
            wasm_time / native_time
        );
        println!(
            "time inside of wasm is {} times slower",
            wasm_time_count / native_time_count
        );
    } else {
        println!("no instantiate");
    }
}

fn native_hello(name: String) -> String {
    format!("hello {}", name)
}

fn native_overhead(name: String) -> (String, u64) {
    let s = std::time::Instant::now();
    let rc = format!("hello {}", name);
    let ms = s.elapsed().as_nanos();

    (rc, ms as u64)
}

fn default_wasi() -> wasmtime_wasi::WasiCtx {
    wasmtime_wasi::sync::WasiCtxBuilder::new()
        .inherit_stdio()
        .build()
}

struct Context<I, E> {
    wasi: wasmtime_wasi::WasiCtx,
    imports: I,
    exports: E,
}

fn instantiate<'a, I: Default, E: Default, T>(
    wasm: &str,
    //LEAVE FOR FUTURE LEARNING add_imports: impl FnOnce(&mut Linker<Context<I, E>>) -> Result<()>,
    mk_exports: impl FnOnce(
        &mut Store<Context<I, E>>,
        &Module,
        &mut Linker<Context<I, E>>,
    ) -> Result<(T, Instance)>,
) -> Result<(T, Store<Context<I, E>>)> {
    let mut config = Config::new();
    config.cache_config_load_default()?;
    config.wasm_backtrace_details(wasmtime::WasmBacktraceDetails::Disable);

    let engine = Engine::new(&config)?;
    let module = Module::from_file(&engine, wasm)?;

    let mut linker = Linker::new(&engine);
    //add_imports(&mut linker)?;
    wasmtime_wasi::add_to_linker(&mut linker, |cx: &mut Context<I, E>| &mut cx.wasi)?;

    let mut store = Store::new(
        &engine,
        Context {
            wasi: default_wasi(),
            imports: I::default(),
            exports: E::default(),
        },
    );
    let (exports, _instance) = mk_exports(&mut store, &module, &mut linker)?;
    Ok((exports, store))
}
