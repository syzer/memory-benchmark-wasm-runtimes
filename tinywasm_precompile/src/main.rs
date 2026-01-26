use tinywasm::parser::Parser;

fn main() {
    let wasm_bytes = include_bytes!(
        "../../benchmark_module/target/wasm32-unknown-unknown/release/benchmark_module.wasm"
    );

    let parser = Parser::default();
    let module = parser
        .parse_module_bytes(wasm_bytes)
        .expect("failed to parser wasm bytes while precompiling for tinywasm");
    let serialized_bytes = module
        .serialize_twasm()
        .expect("failed to serialize precompiled tinywasm module");
    std::fs::write("../benchmark_module.tw", &serialized_bytes)
        .expect("failed to write precompiled tinywasm module");
    println!("module precompiled for tinywasm; resulting file: 'benchmark_module.tw'");
}
