use std::{env, path::PathBuf};

use bindgen::{self, MacroTypeVariation};

fn main() {
    // path to this crate
    let _crate_dir: PathBuf = env::var("CARGO_MANIFEST_DIR").unwrap().into();

    // path to DEOS desk
    let deos_desk_dir: PathBuf = env::var("DEOS_DESK").unwrap_or("/desk".to_string()).into();

    // Modifiy C_INCLUDE_PATH so that the DEOS headers are found.
    // Append, in order to allow overriding by the user (user modification gets precedence when
    // appended).
    let include_path_var_key = "C_INCLUDE_PATH";
    let mut deos_include_paths = Vec::new();
    deos_include_paths.push(deos_desk_dir.join("include"));

    match std::env::var("CARGO_CFG_TARGET_ARCH").unwrap().as_str() {
        "arm" => deos_include_paths.push(deos_desk_dir.join("arm").join("include")),
        "x86" | "x86_64" | "mips" | "powerpc" | "powerpc64" | "aarch64" => {
            unimplemented!()
        }
        _ => {
            panic!("unknown architecture")
        }
    }

    // concatenate the include path into a long string
    let include_path_prefix: Vec<_> = deos_include_paths
        .into_iter()
        .map(|x| x.to_string_lossy().to_string())
        .collect();

    let include_path_prefix = include_path_prefix.join(":");

    // prepend C_INCLUDE_PATH with our include path (if it exists)
    let new_include_path = match std::env::var(include_path_var_key) {
        Ok(include_path) => {
            format!("{include_path_prefix}:{include_path}")
        }
        Err(_) => include_path_prefix,
    };

    std::env::set_var(include_path_var_key, new_include_path);

    // import C functions and types to a Rust file, for consumption by the Rust code
    let bindings = bindgen::Builder::default()
        // block unwanted symbols
        .blocklist_function(".*_$")
        // retain comments
        .clang_args([
            "-fretain-comments-from-system-headers",
            "-fparse-all-comments",
        ])
        // Generate default initializers
        .derive_default(true)
        // default to u32/u64
        .default_macro_constant_type(MacroTypeVariation::Unsigned)
        // Re-export the documentation
        .generate_comments(true)
        // Generate inline functions, otherwise half the API is missing
        .generate_inline_functions(true)
        // The input header we would like to generate
        // bindings for.
        .header("deos_api.h")
        // layout tests fail anyhow, since test is not no_std compatible
        .layout_tests(false)
        // Ugly enums, that actually are treated as integers with unknown possible values
         .newtype_enum("resourceStatus")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))

        // Nice enums can become Rust enums
        .rustified_enum("(CGA.*|execution_history_t|(event|interrupt|ioi|ipc|kernelFile|library|mutex|process|resource|scheduler|semaphore|system|thread)Status|kernelAttributes)")
        // default to libcore
        .use_core()
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
