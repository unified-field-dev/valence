//! Build-time codegen for parameterized cross-backend hop pairs.

fn main() {
    valence_codegen::build().expect("valence codegen failed");
}
