extern crate wasmi;

use wasmi::{ImportsBuilder, ModuleInstance, NopExternals, RuntimeValue};

fn main() {
    // Load pre-compiled wasm bytecode.
    let wasm_binary: Vec<u8> = include_bytes!("leet.wasm").to_vec();

    // Load wasm binary and prepare it for instantiation.
    let module = wasmi::Module::from_buffer(&wasm_binary).expect("failed to load wasm");

    // Instantiate a module with empty imports and
    // assert that there is no `start` function.
    let instance = ModuleInstance::new(&module, &ImportsBuilder::default())
        .expect("failed to instantiate wasm module")
        .assert_no_start();

    // Finally, invoke the exported function "test" with no parameters
    // and empty external function executor.
    assert_eq!(
        instance
            .invoke_export("test", &[], &mut NopExternals,)
            .expect("failed to execute export"),
        Some(RuntimeValue::I32(1337)),
    );
}
