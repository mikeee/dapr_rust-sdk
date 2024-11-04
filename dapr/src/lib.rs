#![doc = include_str!("../README.md")]

extern crate dapr_macros;

pub use serde;
pub use serde_json;

pub use client::Client;
pub use dapr_macros::actor;

/// Module containing the Dapr Callback SDK.
pub mod appcallback;
/// Module containing the 'Client' implementation.
pub mod client;
/// Module importing the Dapr runtime implementation.
pub mod proto {
    /// Module containing the common implementations.

    pub mod common {
        /// Module containing the v1 implementations.
        pub mod v1 {
            include!("dapr/dapr.proto.common.v1.rs");
        }
    }
    /// Module containing the runtime implementations.
    pub mod runtime {
        /// Module containing the v1 implementations.
        pub mod v1 {
            include!("dapr/dapr.proto.runtime.v1.rs");
        }
    }
}
/// Module defining the error implementations.
pub mod error;
/// Module containing the 'Server' implementation.
pub mod server;
