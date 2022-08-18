use substrate_wasm_builder::WasmBuilder;

fn main() {
    WasmBuilder::new()
        .with_current_project()
        .enable_feature("wasm-builder")
        .export_heap_base()
        .import_memory()
        .build();

    kumandra_wasm_tools::export_wasm_bundle_path();
}
