use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    // Put `memory.x` in our output directory and ensure it's
    // on the linker search path.
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("memory.x"))
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());

    // By default, Cargo will re-run a build script whenever
    // any file in the project changes. By specifying `memory.x`
    // here, we ensure the build script is only re-run when
    // `memory.x` is changed.
    println!("cargo:rerun-if-changed=memory.x");

    println!("cargo:rustc-link-arg-bins=--nmagic");
    println!("cargo:rustc-link-arg-bins=-Tlink.x");
    println!("cargo:rustc-link-arg-bins=-Tdefmt.x");

    #[cfg(feature = "engine-wamr")]
    {
        let wamr_dir = std::path::PathBuf::from("../third_party/wamr");
        println!("cargo:wamr_dir={}", wamr_dir.display()); // making it available to main code
        println!("cargo:rerun-if-changed={}", wamr_dir.display()); // rerun the build script if the wamr directory changes
        println!(
            "cargo:warning=WAMR build script running (path: {})",
            wamr_dir.display()
        );

        // get the header files we will need to work with bindgen
        let wamr_include_dir = wamr_dir.join("core/iwasm/include");
        let wasm_export_h = wamr_include_dir.join("wasm_export.h");
        if !wasm_export_h.exists() {
            panic!("WAMR header not found at {}", wasm_export_h.display());
        }
        let include_path = wamr_include_dir
            .to_str()
            .expect("WAMR include path invalid");
        println!("cargo:warning=WAMR include path: {}", include_path);

        // use bindgen to generate bindings from the header
        let bindings = bindgen::Builder::default()
            .header(wasm_export_h.to_str().expect("header path invalid"))
            .parse_callbacks(Box::new(bindgen::CargoCallbacks::new())) // this adds rerun hooks for cargo
            .use_core() // to make bindgen not use std types
            .ctypes_prefix("core::ffi") // to make bindgen use core::ffi::c_types instead of std::os::raw::c_types - if we are fancy, we could give it a module we implemented here
            .generate()
            .expect("unable to generate bindings");

        // write the bindings to the output directory (this is where we put generated files when working with cargo)
        let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
        bindings
            .write_to_file(out_path.join("wamr_bindings.rs"))
            .expect("could not write wamr bindings");
        println!("cargo:rerun-if-changed={}", wasm_export_h.display());
        println!(
            "cargo::warning=WAMR bindings generated and written to {}",
            out_path.join("wamr_bindings.rs").display()
        );

        // define the place where we keep the platform header for bare-metal embassy
        // Around line 68, after defining embassy_platform_dir:
        let embassy_platform_dir = std::path::PathBuf::from("wamr_specific/platform/embassy");
        let platform_internal_h = embassy_platform_dir.join("platform_internal.h");
        println!("cargo:rerun-if-changed={}", platform_internal_h.display());

        // build the WAMR static library
        let mut cc_build = cc::Build::new();
        cc_build
            .include(&wamr_include_dir)
            .include(&embassy_platform_dir)
            .include(wamr_dir.join("core/shared/utils"))
            .include(wamr_dir.join("core/shared/platform/include"))
            .include(wamr_dir.join("core/shared/mem-alloc"))
            .include(wamr_dir.join("core/iwasm/common"))
            // .include(wamr_dir.join("core/iwasm/interpreter")) // interpreter-specific includes from here on
            .include(wamr_dir.join("core/iwasm/aot")) // interpreter-specific includes from here on
            .file(wamr_dir.join("core/shared/mem-alloc/mem_alloc.c"))
            .file(wamr_dir.join("core/shared/mem-alloc/ems/ems_kfc.c"))
            .file(wamr_dir.join("core/shared/mem-alloc/ems/ems_alloc.c"))
            .file(wamr_dir.join("core/shared/utils/bh_common.c"))
            // .file(wamr_dir.join("core/shared/utils/bh_list.c"))
            .file(wamr_dir.join("core/shared/utils/bh_vector.c"))
            .file(wamr_dir.join("core/shared/utils/bh_leb128.c"))
            .file(wamr_dir.join("core/iwasm/common/arch/invokeNative_general.c")) // we apparently could use sth else here (invokeNative_thumb) to optimize
            .file(wamr_dir.join("core/iwasm/common/wasm_loader_common.c"))
            .file(wamr_dir.join("core/iwasm/common/wasm_runtime_common.c"))
            .file(wamr_dir.join("core/iwasm/common/wasm_native.c"))
            .file(wamr_dir.join("core/iwasm/common/wasm_memory.c"))
            .file(wamr_dir.join("core/iwasm/common/wasm_exec_env.c"))
            .file(wamr_dir.join("core/iwasm/common/wasm_c_api.c")) // not sure whether this is smart - checking how much we save (answr is: not much)
            .file(wamr_dir.join("core/shared/utils/bh_log.c"))
            // things needed just for interpreter
            // .file(wamr_dir.join("core/iwasm/interpreter/wasm_loader.c")) // interpreter-mode specific stuff from here on out
            // .file(wamr_dir.join("core/iwasm/interpreter/wasm_runtime.c"))
            // .file(wamr_dir.join("core/iwasm/interpreter/wasm_interp_classic.c"))
            // .define("WASM_ENABLE_INTERP", Some("1"))
            // things needed just for aot
            // .file(wamr_dir.join("core/iwasm/aot/aot_intrinsic.c"))
            .file(wamr_dir.join("core/iwasm/aot/aot_loader.c"))
            .file(wamr_dir.join("core/iwasm/aot/aot_runtime.c"))
            .file(wamr_dir.join("core/iwasm/aot/arch/aot_reloc_thumb.c"))
            .file(wamr_dir.join("core/shared/utils/bh_hashmap.c"))
            .define("WASM_ENABLE_AOT", Some("1"))
            .define("WASM_ENABLE_INTERP", Some("0"))
            // other defines
            .define("BH_MALLOC", Some("wasm_runtime_malloc"))
            .define("BH_FREE", Some("wasm_runtime_free"))
            .define("BUILD_TARGET_THUMB", None)
            .define("BUILD_TARGET", Some("\"THUMBV7\"")) // for the AOT mode: must match our target (and the info we gave wamrc when compiling the module)
            .define("WASM_ENABLE_QUICK_AOT_ENTRY", Some("0")) // Disable quick entry optimization
            .define("WASM_ENABLE_AOT_INTRINSICS", Some("0")) // Disable quick entry optimization
            .define("WASM_ENABLE_LOG", Some("0"))
            .flag("-Os")
            .flag("-ffunction-sections")
            .flag("-fdata-sections")
            .flag("-g0") // No debug info
            .compile("wamr");

        println!("cargo:rustc-link-lib=static=wamr"); // link the static library to the final binary
        println!("cargo:rustc-link-search=native={}", out_path.display()); // search for the static library in the output directory (so that we know where the file is we just mentioned)
    }
}
