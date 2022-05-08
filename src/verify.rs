use url::Url;

use crate::Error;

#[derive(Debug, Serialize, Deserialize)]
struct LoginData {
    #[serde(rename = "openid.ns")]
    ns: String,
    #[serde(rename = "openid.mode")]
    mode: String,
    #[serde(rename = "openid.op_endpoint")]
    op_endpoint: String,
    #[serde(rename = "openid.claimed_id")]
    claimed_id: String,
    #[serde(rename = "openid.identity")]
    identity: String,
    #[serde(rename = "openid.return_to")]
    return_to: String,
    #[serde(rename = "openid.response_nonce")]
    response_nonce: String,
    #[serde(rename = "openid.invalidate_handle")]
    invalidate_handle: Option<String>,
    #[serde(rename = "openid.assoc_handle")]
    assoc_handle: String,
    #[serde(rename = "openid.signed")]
    signed: String,
    #[serde(rename = "openid.sig")]
    sig: String,
}

impl LoginData {
    pub fn claim_id(&self) -> Result<u64, Error> {
        let claimed_url =
            Url::parse(&self.claimed_id).map_err(|e| Error::ParseSteamID(e.to_string()))?;
        let mut url_segments = claimed_url
            .path_segments()
            .ok_or(Error::ParseSteamID("Invalid claimed url".to_owned()))?;
        let id_segment = url_segments
            .next_back()
            .ok_or(Error::ParseSteamID("Claim not found".to_owned()))?;

        id_segment
            .parse::<u64>()
            .map_err(|e| Error::ParseSteamID(e.to_string()))
    }
}

#[cfg(feature = "summaries")]
#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct PlayerSummaries {
    pub steamid: String,
    pub communityvisibilitystate: u32,
    pub profilestate: u32,
    pub personaname: String,
    pub commentpermission: u32,
    pub profileurl: String,
    pub avatar: String,
    pub avatarmedium: String,
    pub avatarfull: String,
    pub avatarhash: String,
    pub lastlogoff: u64,
    pub personastate: u32,
    pub primaryclanid: String,
    pub timecreated: u64,
    pub personastateflags: u32,
}

#[cfg(feature = "summaries")]
#[derive(Deserialize)]
struct SummariesPlayers {
    players: Vec<PlayerSummaries>,
}

#[cfg(feature = "summaries")]
#[derive(Deserialize)]
struct SummariesResponse {
    response: SummariesPlayers,
}

#[derive(Debug)]
pub struct Verify {
    claimed_id: u64,
}

impl Verify {
    async fn is_valid(&self, data: &LoginData) -> Result<bool, Error> {
        let form = serde_qs::to_string(&data).map_err(|e| Error::ParseSteamID(e.to_string()))?;

        let client = reqwest::Client::new();
        let response = client
            .post("https://steamcommunity.com/openid/login")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(form)
            .send()
            .await
            .map_err(|e| Error::ParseSteamID(e.to_string()))?
            .text()
            .await
            .map_err(|e| Error::ParseSteamID(e.to_string()))?;

        let is_valid = response
            .split("\n")
            .filter_map(|line| {
                let mut pair = line.splitn(2, ":");
                Some((pair.next()?, pair.next()?))
            })
            .any(|(k, v)| k == "is_valid" && v == "true");

        Ok(is_valid)
    }

    pub async fn verify_request(query_string: &str) -> Result<Self, Error> {
        let mut data = serde_qs::from_str::<LoginData>(query_string).map_err(Error::Deserialize)?;
        data.mode = "check_authentication".to_owned();

        let verify = Self {
            claimed_id: data.claim_id()?,
        };

        if !verify.is_valid(&data).await? {
            return Err(Error::ParseSteamID("Invalid data".to_string()));
        }

        Ok(verify)
    }

    #[cfg(feature = "summaries")]
    pub async fn get_summaries(&self, apikey: &str) -> Result<PlayerSummaries, Error> {
        let steamid = self.claimed_id.to_string();

        let client = reqwest::Client::new();
        let response = client
            .get("https://api.steampowered.com/ISteamUser/GetPlayerSummaries/v0002/")
            .query(&[("key", apikey), ("steamids", steamid.as_str())])
            .send()
            .await
            .map_err(|e| Error::GetSummaries(e.to_string()))?
            .json::<SummariesResponse>()
            .await
            .map_err(|e| Error::GetSummaries(e.to_string()))?;

        let player = response
            .response
            .players
            .first()
            .ok_or(Error::GetSummaries("Failed to find player".to_owned()))?;

        Ok(player.clone())
    }

    pub fn claim_id(&self) -> u64 {
        self.claimed_id
    }
}
