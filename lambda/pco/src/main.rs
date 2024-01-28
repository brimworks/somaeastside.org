use poem::Route;
use poem_openapi::OpenApiService;
use clap::Command;
use poem::listener::TcpListener;
use clap::arg;
use std::env;
use crate::api::PersonalAccessToken;
use url::Url;

use crate::error::Result;
use crate::error::Trace;
use crate::error::WrapErr;
use crate::error::OrNotFound;

pub(crate) mod api;
pub(crate) mod secret;
pub(crate) mod error;

struct Config {
    pat: PersonalAccessToken,
    self_url: &'static str,
    pco_url: Url,
}

fn env_var<K: AsRef<str> + AsRef<std::ffi::OsStr>>(key: K) -> Result<String> {
    env::var(&key).wrap_err_msg(|_| format!(
        "Missing environment variable '{}'",
        <K as AsRef<str>>::as_ref(&key)))
}

// FIXME: hard-coded URLs:
fn config_local() -> Result<Config> {
    Ok(Config {
        pat: PersonalAccessToken {
            app_id: env_var("PCO_PAT_APP_ID").trace()?,
            secret: env_var("PCO_PAT_SECRET").trace()?.into(),
        },
        self_url: "http://localhost:3001/api",
        pco_url: "https://api.planningcenteronline.com/".try_into().wrap_err()?
    })
}

async fn get_parameter(client: &aws_sdk_ssm::Client, parameter: impl Into<String>) -> Result<String> {
    let parameter: String = parameter.into();
    let value = client.get_parameter()
        .name(parameter.clone())
        .with_decryption(true)
        .send()
        .await
        .wrap_err_msg(|_| format!("getting parameter={parameter}"))?
        .parameter
        .or_not_found(|| format!("parameter={parameter} not found"))?
        .value
        .or_not_found(|| format!("parameter={parameter} has no value"))?;
    Ok(value)
}

async fn config_lambda() -> Result<Config> {
    let config = aws_config::load_from_env().await;
    let client = aws_sdk_ssm::Client::new(&config);
    Ok(Config {
        pat: PersonalAccessToken {
            app_id: get_parameter(&client, "/prod/pco/pat/app_id").await.trace()?,
            secret: get_parameter(&client, "/prod/pco/pat/secret").await.trace()?.into(),
        },
        self_url: "https://soma.brimworks.com/api",
        pco_url: "https://api.planningcenteronline.com/".try_into().wrap_err()?
    })
}

#[tokio::main]
async fn main() -> Result<()> {
    let matches = Command::new("Soma API")
        .author("Brian Maher")
        .version("1.0")
        .about("Serve any dynamic content for the Soma website")
        .args(&[
            arg!(--local "Bind to local TCP socket, without this the Lambda runtime is assumed")
            .action(clap::ArgAction::SetTrue),
        ])
        .get_matches();
    let config = if matches.get_flag("local") {
        config_local()
    } else {
        config_lambda().await
    }.trace()?;
    let api_service =
        OpenApiService::new(api::Api::new(config.pco_url, config.pat).trace()?, "Soma API", "1.0").server(config.self_url);
    let ui = api_service.swagger_ui();
    let app = Route::new().nest("/api", api_service).nest("/api/docs", ui);

    if matches.get_flag("local") {
        println!("\x1b[1;94mServing API here\x1b[0m -> http://localhost:3001/api/docs");
        poem::Server::new(TcpListener::bind("127.0.0.1:3001"))
            .run(app)
            .await
            .wrap_err()?;
    } else {
        poem_lambda::run(app).await.wrap_err()?;
    }
    Ok(())
}

/*
use lambda_runtime::{service_fn, LambdaEvent, Error};
use serde_json::{json, Value};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = service_fn(func);
    lambda_runtime::run(func).await?;
    Ok(())
}

/*

event structure:

"headers": {
    // Cloudfront enhanced location headers:
    "cloudfront-viewer-latitude": "47.55590",
    "cloudfront-viewer-longitude": "-121.99870",
    "cloudfront-viewer-time-zone": "America/Los_Angeles",
    "cloudfront-viewer-country-region": "WA",
    "cloudfront-viewer-city": "Issaquah",
    "cloudfront-viewer-country-name": "United States",
    "cloudfront-viewer-postal-code": "98029",
    "cloudfront-viewer-country": "US",
    "x-forwarded-for": "216.9.0.224",

    // Standard headers
    "content-type": "application/json",
},
"requestContext": {
    "http": {
        "protocol": "HTTP/1.1"
        "method": "POST",
    }
}
"rawPath": "/api/path",
"rawQueryString": "query=val",

*/
async fn func(event: LambdaEvent<Value>) -> Result<Value, Error> {
    let (event, _context) = event.into_parts();

    Ok(json!({ "echo": event }))
}
*/