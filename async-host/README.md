# async-host sample (WASI 0.3 style async host calls)

このサンプルは以下を同時に示します。

- **提供**: ホスト側が `host:hello-world/host-trait` を `async` 実装で提供する
- **利用**: ホスト側がコンポーネントの `component:tom/user-funcs#hello-world` を `async` 呼び出しで利用する

`bindgen!` は `async: true` を有効化しているため、
コンポーネント export への呼び出し (`typed.call_async`) と host import 実装の両方を非同期で扱えます。

## 実行例

```sh
# 1) component をビルドして plugin.wasm を配置
cargo component build --manifest-path /home/runner/work/wit-sample0/wit-sample0/component_example/Cargo.toml
cp /home/runner/work/wit-sample0/wit-sample0/component_example/target/wasm32-wasip1/debug/component_example.wasm \
  /home/runner/work/wit-sample0/wit-sample0/plugin.wasm

# 2) async host を実行
cargo run --manifest-path /home/runner/work/wit-sample0/wit-sample0/async-host/Cargo.toml
```
