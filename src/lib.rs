// Include new module declarations here instead of in main.rs.
// And use them in main.rs using the module root `labwhere`.
// e.g. use labwhere::models::location_type::LocationType;
//
// The project having both main.rs and lib.rs means that it has two crates - a binary and a library.
// Both of these crates have the same name as the package listed in Cargo.toml.
//
// For more info, check https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html.
pub mod db;
pub mod errors;
pub mod models;
