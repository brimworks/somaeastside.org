use poem_openapi::OpenApi;
use poem_openapi::payload::Json;
use serde_json::Value;
use crate::api::Api;
use crate::error::*;
use std::time::SystemTime;
use chrono::DateTime;
use chrono::offset::Utc;
use chrono::SecondsFormat;
use std::time::Duration;

#[OpenApi]
impl Api {
    #[oai(path = "/events", method = "get")]
    async fn events(&self) -> Result<Json<Value>> {
        let last_week = DateTime::<Utc>::from(SystemTime::now()) - Duration::from_secs(60 * 60 * 24 * 7);
        let mut url = self.url.join("calendar/v2/event_instances")?;
        url.query_pairs_mut()
            .append_pair("order", "starts_at")
            .append_pair("where[ends_at][gt]", &last_week.to_rfc3339_opts(SecondsFormat::Secs, true))
            .append_pair("include", "event,tags");
        let response = self.client.get(url)
            .header("accept", "application/vnd.api+json")
            .basic_auth(&self.pat.app_id, Some(self.pat.secret.as_ref()))
            .send()
            .await?
            .json::<Value>()
            .await?;
        Ok(Json(response))
    }
}
