//! Host-side CLI for `build.rs` scripts (avoids listing `valence-codegen` as a build-dependency
//! when cross-compiling the frontend to wasm32).

use std::path::PathBuf;

fn main() {
    let mut args = std::env::args();
    let _program = args.next();
    let Some(schemas_dir) = args.next() else {
        usage();
    };
    let Some(out_dir) = args.next() else {
        usage();
    };
    let file_suffix = args
        .next()
        .unwrap_or_else(|| valence_codegen::DEFAULT_SCHEMA_SUFFIX.to_string());
    let trait_file_suffix = args
        .next()
        .unwrap_or_else(|| valence_codegen::DEFAULT_TRAIT_SUFFIX.to_string());

    if args.next().is_some() {
        usage();
    }

    valence_codegen::generate_models(valence_codegen::CodegenConfig {
        schemas_dir: PathBuf::from(schemas_dir),
        out_dir: PathBuf::from(out_dir),
        file_suffix: &file_suffix,
        trait_file_suffix: &trait_file_suffix,
    })
    .expect("valence codegen failed");
}

fn usage() -> ! {
    eprintln!(
        "usage: valence-generate <schemas_dir> <out_dir> [<file_suffix> <trait_file_suffix>]"
    );
    eprintln!(
        "defaults: file_suffix={} trait_file_suffix={}",
        valence_codegen::DEFAULT_SCHEMA_SUFFIX,
        valence_codegen::DEFAULT_TRAIT_SUFFIX
    );
    std::process::exit(1);
}
