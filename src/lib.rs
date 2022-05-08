#[macro_use]
extern crate serde;

mod redirect;
mod verify;

pub use redirect::Redirect;
pub use verify::Verify;

#[derive(Debug)]
pub enum Error {
    ParseUrl(url::ParseError),
    ParseQuery(serde_qs::Error),
    Deserialize(serde_qs::Error),
    ParseSteamID(String),
    #[cfg(feature = "summaries")]
    GetSummaries(String),
}
