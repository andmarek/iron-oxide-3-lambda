use aws_config::meta::region::RegionProviderChain;
use lambda_runtime::{handler_fn, Context};
use tracing::log::{error, info};
use aws_sdk_sesv2::{Region, Client};
use serde::{Deserialize, Serialize};
use std::env;
mod ses;
use structopt::StructOpt;
use ses::{Opt, send_message};



#[derive(Deserialize)]
struct Request {
    pub body: String,
}

#[derive(Debug, Serialize)]
struct SuccessResponse {
    pub body: String,
}

#[derive(Debug, Serialize)]
struct FailureResponse {
    pub body: String,
}

impl std::fmt::Display for FailureResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.body)
    }
}

impl std::error::Error for FailureResponse {}

type Response = Result<SuccessResponse, FailureResponse>;

#[tokio::main]
async fn main() -> Result<(), lambda_runtime::Error> {
    let func = handler_fn(handler);
    lambda_runtime::run(func).await?;

    Ok(())
}


async fn handler(req: Request, _ctx: lambda_runtime::Context) -> Response {
    info!("handling a request...");
    let Opt {
        contact_list,
        region,
        from_address,
        message,
        subject,
        verbose,
    } = Opt::from_args();

    /* gonna try to hardcode this because it's not working
    let region_provider = RegionProviderChain::first_try(region.map(Region::new))
        .or_default_provider()
        .or_else(Region::new("us-west-2"));
    */

    let provider = RegionProviderChain::first_try("us-east-1")
        .or_default_provider()
        .or_else("us-east-2");

    let shared_config = aws_config::from_env().region(provider).load().await;

    // TODO: try to figure out how to change types ? 
    let client = Client::new(&shared_config);

    send_message(&client, &contact_list, &from_address, &subject, &message).await;


    let filename = format!("{}.txt", time::OffsetDateTime::now_utc().unix_timestamp());
    info!(
        "YOOOOOOOOOOOOO we ran bro '{}'",
        &filename
    );

    Ok(SuccessResponse {
        body: format!(
            "wtf {}",
            filename
        ),
    })
}
