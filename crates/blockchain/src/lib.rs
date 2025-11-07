#![cfg_attr(not(feature = "std"), no_std)]

// Simplified blockchain module for prototype
// In production, these would be full Substrate pallets

pub mod types;

pub use types::*;
pub use sp_core;
pub use sp_runtime;
