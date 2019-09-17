#![deny(missing_docs)]
#![deny(missing_doc_code_examples)]
#![deny(unsafe_code)]
//! ElimuNasi client program
//! # Example
//! First, clone this repo using git
//!
//! `cargo build --release`
//!
//! Run the project in production
//!
//! env ROCKET=prod ./target/release/en_client
//!
//! ```no_run
//!
//! # use en_client;
//! ```
    /// Global user defined types
    #[allow(missing_doc_code_examples)]
pub mod global;
    /// ## `global` module for global re-exports
    /// #### Example
    /// ```no_run
    /// use en_client::{User, stringify_user, enumify_user};
    /// ```
pub use global::{User, stringify_user, enumify_user};
