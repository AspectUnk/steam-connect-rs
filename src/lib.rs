#[doc = include_str!("../README.md")]

#[macro_use]
extern crate serde;

mod redirect;
mod verify;

pub use redirect::Redirect;
pub use verify::Verify;

#[derive(Debug)]
pub enum Error {
    // Often occurs due to incorrect callback link specified when creating a redirect link
    ParseUrl(url::ParseError),
    ParseQuery(serde_qs::Error),
    // Query string conversion error when checking for validity
    Deserialize(serde_qs::Error),
    // Error getting a SteamID64
    ParseSteamID(String),
    #[cfg(feature = "summaries")]
    // Error getting a player profile
    GetSummaries(String),
}
