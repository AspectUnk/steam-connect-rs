use url::Url;

use crate::Error;

#[cfg(feature = "actix")]
use actix_web::{http, HttpResponse};

#[derive(Serialize)]
struct OpenIDRequest<'a> {
    #[serde(rename = "openid.mode")]
    mode: &'a str,
    #[serde(rename = "openid.ns")]
    ns: &'a str,
    #[serde(rename = "openid.identity")]
    identity: &'a str,
    #[serde(rename = "openid.claimed_id")]
    claimed_id: &'a str,
    #[serde(rename = "openid.return_to")]
    return_to: &'a str,
    #[serde(rename = "openid.realm")]
    realm: &'a str,
}

pub struct Redirect {
    url: Url,
}

impl Redirect {
    pub fn new(callback: &str) -> Result<Self, Error> {
        let url = Url::parse(callback).map_err(Error::ParseUrl)?;

        let request = OpenIDRequest {
            mode: "checkid_setup",
            ns: "http://specs.openid.net/auth/2.0",
            identity: "http://specs.openid.net/auth/2.0/identifier_select",
            claimed_id: "http://specs.openid.net/auth/2.0/identifier_select",
            return_to: &url.to_string(),
            realm: &url.origin().ascii_serialization(),
        };

        let params = serde_qs::to_string(&request).map_err(Error::ParseQuery)?;

        let mut auth_url =
            Url::parse("https://steamcommunity.com/openid/login").map_err(Error::ParseUrl)?;
        auth_url.set_query(Some(&params));

        Ok(Self { url: auth_url })
    }

    #[cfg(feature = "actix")]
    pub fn redirect(&self) -> HttpResponse {
        HttpResponse::TemporaryRedirect()
            .append_header((http::header::LOCATION, self.url.to_string()))
            .finish()
    }

    pub fn url(&self) -> &Url {
        &self.url
    }
}
