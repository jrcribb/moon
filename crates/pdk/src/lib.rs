mod args;
mod extension;
mod toolchain;

pub use args::*;
pub use extension::*;
pub use moon_pdk_api::*;
pub use toolchain::*;
pub use warpgate_pdk::*;

/// Map a `miette` (or similar error) to an `extism` Error.
pub fn map_miette_error(error: impl std::fmt::Display) -> extism_pdk::Error {
    anyhow!("{error}")
}
