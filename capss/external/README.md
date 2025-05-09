# External Dependencies

This folder contains all the external dependencies of the CAPSS framework:

 * The folder `anemoi-rust` contains a copy of the git repository `anemoi-rust` at the commit `647c921` (December 2023, 14th), available at https://github.com/anemoi-hash/anemoi-rust. Here is the list of the modified files:
    * The files `src/bn254/anemoi_2_1/wrapper.rs` and `src/bn254/anemoi_4_3/wrapper.rs` have been added to provide a binding Rust-C of some functions.
    * The line `mod wrapper;` has been added in the files `src/bn254/anemoi_2_1/mod.rs` and `src/bn254/anemoi_4_3/mod.rs` to include the new wrapper files.
    * The file `Cargo.toml` by adding the following lines to enable the user to compile the release as a static library:
        ```bash
        [lib]
        crate-type = ["staticlib"]
        ```
 * The folder `hash_f64_benchmarks` contains a copy of the git repository `hash_f64_benchmarks` at the commit `53b8225` (May 2023, 11th), available at https://github.com/anemoi-hash/hash_f64_benchmarks. Here is the list of the modified files:
    * The file `src/anemoi/anemoi_64_8_4/wrapper.rs` has been added to provide a binding Rust-C of some functions.
    * The line `mod wrapper;` has been added in the file `src/anemoi/anemoi_64_8_4/mod.rs` to include the new wrapper file.
    * The file `Cargo.toml` by adding the following lines to enable the user to compile the release as a static library:
        ```bash
        [lib]
        crate-type = ["staticlib"]
        ```
