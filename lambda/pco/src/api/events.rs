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
use std::marker::PhantomData;
use serde::de;
use std::fmt;
use std::collections::HashMap;
use serde::Deserialize;
use std::result;
use poem_openapi::Object;
use serde::Serialize;

// TODO: Check for 429 error (rate limit).
// Response is {"errors":[{"code": "429", "detail": "Rate limit exceeded: 118 of 100 requests per 20 seconds"}]}
// Error hints are here: https://developer.planning.center/docs/#/overview/errors
#[derive(Deserialize, Object, Serialize, Debug, Clone, PartialEq, Eq)]
struct PCOMeta {
    pub count: i32,
    pub total_count: i32,
}

#[derive(Deserialize, Object, Serialize, Debug, Clone, PartialEq, Eq)]
struct PCOLink {
    #[serde(rename = "self")]
    pub myself: String,
    pub html: Option<String>,
}

#[derive(Deserialize, Object, Serialize, Debug, Clone, PartialEq, Eq)]
struct PCORef {
    pub id: String,
    #[serde(rename = "type")]
    pub mytype: String,
}

#[derive(Deserialize, Object, Serialize, Debug, Clone, PartialEq, Eq)]
struct PCORelated {
    pub related: String,
}

#[derive(Deserialize, Object, Serialize, Debug, Clone, PartialEq, Eq)]
struct PCORelationship {
    #[serde(deserialize_with = "value_or_array")]
    pub data: Vec<PCORef>, // could be not wrapped in array...
    pub links: Option<PCORelated>,
}

#[derive(Deserialize, Object, Serialize, Debug, Clone, PartialEq, Eq)]
struct PCOObject {
    pub attributes: HashMap<String, Value>,
    pub id: String,
    pub links: PCOLink,
    pub relationships: HashMap<String, PCORelationship>,
    #[serde(rename = "type")]
    pub mytype: String,
}

#[derive(Deserialize, Object, Serialize, Debug, Clone, PartialEq, Eq)]
struct PCOPagination {
    #[serde(rename = "self")]
    pub myself: String,
    pub next: Option<String>,
}

#[derive(Deserialize, Object, Serialize, Debug, Clone, PartialEq, Eq)]
struct PCOResponse {
    pub data: Vec<PCOObject>,
    pub included: Vec<PCOObject>,
    pub links: PCOPagination,
    pub meta: PCOMeta,
}

#[OpenApi]
impl Api {
    #[oai(path = "/events", method = "get")]
    async fn events(&self) -> Result<Json<PCOResponse>> {
        let last_week = DateTime::<Utc>::from(SystemTime::now()) - Duration::from_secs(60 * 60 * 24 * 7);
        let mut url = self.url.join("calendar/v2/event_instances").wrap_err()?;
        url.query_pairs_mut()
            .append_pair("order", "starts_at")
            .append_pair("where[ends_at][gt]", &last_week.to_rfc3339_opts(SecondsFormat::Secs, true))
            .append_pair("include", "event,tags")
            .append_pair("per_page", "100");
        let response = self.client.get(url)
            .header("accept", "application/vnd.api+json")
            .basic_auth(&self.pat.app_id, Some(self.pat.secret.as_ref()))
            .send()
            .await.wrap_err()?
            .json::<PCOResponse>()
            .await.wrap_err()?;
        Ok(Json(response))
    }
}

fn value_or_array<'de, T, D>(deserializer: D) -> result::Result<Vec<T>, D::Error>
where
    T: Deserialize<'de>,
    D: serde::Deserializer<'de>,
{
    struct ValueOrArray<T>(PhantomData<fn() -> T>);
    impl<'de, T> de::Visitor<'de> for ValueOrArray<T>
    where
        T: Deserialize<'de>,
    {
        type Value = Vec<T>;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("value or array")
        }
        fn visit_seq<S>(self, seq: S) -> result::Result<Self::Value, S::Error>
        where
            S: de::SeqAccess<'de>,
        {
            Deserialize::deserialize(de::value::SeqAccessDeserializer::new(seq))
        }
        fn visit_map<M>(self, map: M) -> result::Result<Self::Value, M::Error>
        where
            M: de::MapAccess<'de>
        {
            Ok(vec![Deserialize::deserialize(de::value::MapAccessDeserializer::new(map))?])
        }
    }
    deserializer.deserialize_any(ValueOrArray(PhantomData))
}