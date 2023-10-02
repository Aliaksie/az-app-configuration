//! Azure App Configuration
extern crate azure_core;

pub(crate) mod auto_refresh;

#[cfg(feature = "configuration")]
pub mod configuration;
#[cfg(feature = "feature_manager")]
pub mod feature_manager;

pub mod prelude;
