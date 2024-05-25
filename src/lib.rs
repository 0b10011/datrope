#![deny(warnings)]

#[cfg(all(
    not(feature = "private"),
    any(feature = "gateway", feature = "gateway_objects")
))]
pub mod gateway;

#[cfg(all(
    feature = "private",
    any(feature = "gateway", feature = "gateway_objects")
))]
mod gateway;

#[cfg(all(
    not(feature = "private"),
    any(feature = "api", feature = "api_objects")
))]
pub mod api;

#[cfg(all(feature = "private", any(feature = "api", feature = "api_objects")))]
mod api;
