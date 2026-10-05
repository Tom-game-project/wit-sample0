use anyhow::Result;
use host::hello_world::host_trait::Host as HostTrait;
use tokio::time::{Duration, sleep};
use wasmtime::component::{Component, HasSelf, Linker, ResourceTable, bindgen};
use wasmtime::{Config, Engine, Store};
use wasmtime_wasi::p2::{IoView, WasiCtx, WasiCtxBuilder, WasiView};

bindgen!({
    path: "../wit/host-api.wit",
    world: "hello-world",
    async: true,
});

struct MyState {
    table: ResourceTable,
    wasi_ctx: WasiCtx,
    name: String,
}

impl HostTrait for MyState {
    async fn say_hello(&mut self, name: String) -> String {
        sleep(Duration::from_millis(150)).await;
        format!("Hello {}! (from async host import)", name)
    }

    async fn get_name(&mut self) -> String {
        sleep(Duration::from_millis(50)).await;
        self.name.clone()
    }

    async fn set_name(&mut self, name: String) {
        sleep(Duration::from_millis(80)).await;
        self.name = name;
    }
}

impl IoView for MyState {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
}

impl WasiView for MyState {
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.wasi_ctx
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut config = Config::new();
    config.wasm_component_model(true);
    config.async_support(true);
    let engine = Engine::new(&config)?;

    let mut linker: Linker<MyState> = Linker::new(&engine);
    wasmtime_wasi::p2::add_to_linker_async(&mut linker)?;
    HelloWorld::add_to_linker::<_, HasSelf<MyState>>(&mut linker, |s: &mut MyState| s)?;

    let table = ResourceTable::new();
    let wasi_ctx = WasiCtxBuilder::new()
        .inherit_stdout()
        .inherit_stderr()
        .build();
    let state = MyState {
        table,
        wasi_ctx,
        name: "Tom".to_string(),
    };
    let mut store = Store::new(&engine, state);

    let component = Component::from_file(&engine, "./plugin.wasm")?;
    let instance = linker.instantiate_async(&mut store, &component).await?;

    let interface_name = "component:tom/user-funcs";
    let func_name = "hello-world";
    let interface_idx = instance
        .get_export_index(&mut store, None, interface_name)
        .expect("Cannot get `component:tom/user-funcs` interface");
    let func_idx = instance
        .get_export_index(&mut store, Some(&interface_idx), func_name)
        .expect("Cannot get `hello-world` function in `component:tom/user-funcs`");
    let func = instance
        .get_func(&mut store, func_idx)
        .expect("function index exists");
    let typed = func.typed::<(String,), (String,)>(&store)?;
    let (result,) = typed.call_async(&mut store, ("Tomoo".to_string(),)).await?;
    typed.post_return_async(&mut store).await?;

    println!("returned from rust component: {}", result);
    println!("state.name: {}", store.data().name);
    Ok(())
}
