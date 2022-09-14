use std::{
    env, fs,
    path::{Path, PathBuf},
};

fn main() {
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    fs::remove_dir_all(&out).unwrap();
    fs::create_dir(&out).unwrap();
    let files = [
        "src/nvpair.c",
        "src/nvpair_alloc_system.c",
        "src/libnvpair.c",
    ];
    let includes = [Path::new("/usr/include"), Path::new("/usr/local/include")];

    println!("cargo:rustc-link-search=native={}", out.display());
    println!("cargo:rustc-link-lib=static=libnvpair");

    cc::Build::new()
        .files(files)
        .includes(includes)
        .flag_if_supported("-g")
        .flag_if_supported("-Wall")
        .flag("-fPIC")
        .flag("-O2")
        .flag("-std=c99")
        .flag("-D_GNU_SOURCE")
        .flag("-D__EXTENSION__")
        .flag("-D_LARGEFILE_SOURCE")
        .flag("-D_FILE_OFFSET_BITS=64")
        .flag("-DPIC")
        .flag("-Wunused-parameter")
        .flag("-Wsign-compare")
        .flag("-Wformat-security")
        // .flag("-shared")
        // .out_dir("tmp")
        .compile("libnvpair");
    println!("cargo:rerun-if-changed=src/nvpair.c");
    println!("cargo:rerun-if-changed=src/nvpair_alloc_system.c");
    println!("cargo:rerun-if-changed=src/libnvpair.c");

    let default_enum_style = bindgen::EnumVariation::Rust {
        non_exhaustive: true,
    };

    println!("cargo:rustc-link-lib=nvpair");
    println!("cargo:rerun-if-changed=nvpair.h");

    let bindings = bindgen::Builder::default()
        .header("nvpair.h")
        // .clang_args(["-I".to_string() + out.as_os_str().to_str().unwrap()])
        .size_t_is_usize(true)
        .ctypes_prefix("libc")
        .allowlist_var(r#"(^NV_\w*)"#)
        .allowlist_type(r#"(\w*nvpair\w*)"#)
        .allowlist_type(r#"(\w*nvlist\w*)"#)
        .allowlist_function(r#"(\w*nvpair\w*)"#)
        .allowlist_function(r#"(\w*nvlist\w*)"#)
        .rustified_enum("boolean_t")
        .default_enum_style(default_enum_style)
        .blocklist_function(r#"nvpair_value_match\w*"#)
        .blocklist_function(r#"nvlist_print\w*"#)
        .blocklist_function("dump_nvlist")
        .blocklist_item(r#"nvlist_prt\w*"#)
        .blocklist_type("regex_t")
        .blocklist_type("reg_syntax_t")
        .blocklist_type("re_pattern_buffer")
        .blocklist_type("FILE")
        .blocklist_item(r#"_IO_\w*"#)
        .generate()
        .expect("Unable to generate bindings");

    let nvpair = env::var("OUT_DIR")
        .map(PathBuf::from)
        .expect("OUT_DIR environment")
        .join("nvpair.rs");

    bindings
        .write_to_file(nvpair)
        .expect("Couldn't write bindings!");
}
