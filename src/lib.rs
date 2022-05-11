//! # steam-connect
//!
//! Implementation Steam web authorization for simple use in projects with or without actix_web
//!
//! ### Usage
//!
//! Example:
//! ```
//! // Getting the authorization link. Requires a link to redirect
//! // the user after authorization. If used in a project with
//! // actix_web, you can use the redirect function defined in Redirect
//! let url = Redirect::new("http://127.0.0.1:8080/auth/callback").unwrap();
//!
//! // Performs data validation when returning to the callback page
//! let verify = Verify::verify_request(req.query_string()).await.unwrap();
//!
//! verify.claim_id(); // Get SteamID64 of an authorized user
//!
//! // Only available in summaries feature.
//! // Queries the steam api for more information about the profile.
//! verify.get_summaries();
//! ```
//!
//! You can study an [example project](https://github.com/AspectUnk/steam-connect-rs/blob/main/examples/actix.rs) using actix_web

#[macro_use]
extern crate serde;
#[macro_use]
extern crate lazy_static;

mod redirect;
mod verify;

pub use redirect::Redirect;
pub use verify::Verify;

#[derive(Debug)]
pub enum Error {
    /// Often occurs due to incorrect callback link specified when creating a redirect link
    ParseUrl(url::ParseError),
    ParseQuery(serde_qs::Error),
    /// Query string conversion error when checking for validity
    Deserialize(serde_qs::Error),
    /// Error getting a SteamID64
    ParseSteamID(String),
    /// Error getting a player profile
    GetSummaries(String),
}
