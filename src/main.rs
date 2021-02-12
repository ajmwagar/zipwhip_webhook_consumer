use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use warp::{Filter, Rejection, Reply};
use zipwhip_rs::Client as ZW;

use log::info;

use std::sync::Arc;
use tokio::sync::RwLock;

type SharedState = Arc<RwLock<State>>;

struct State {

}

lazy_static! {
    pub static ref PORT: u16 = {
        std::env::var("PORT")
            .unwrap_or("3030".to_string())
            .parse()
            .unwrap_or(3030)
    };

    pub static ref SESSION_KEY: String = {
        std::env::var("SESSION_KEY").expect("Please specify SESSION_KEY.")
    };
}

#[tokio::main]
async fn main() {
    env_logger::init();

    info!("Initializing State...");

    let state = Arc::new(RwLock::new(State {}));

    info!("Building endpoints...");

    let endpoint = warp::path("receive")
        .and(with_state(state.clone()))
        .and(warp::body::json())
        .and_then(handle_webhook);
    let health_check = warp::path("health").map(|| "OK");

    info!("Binding to port {}", *PORT);

    info!("Ready to receive webhooks.");

    warp::serve(endpoint.or(health_check))
        .run(([0, 0, 0, 0], *PORT))
        .await;
}

async fn handle_webhook(state: SharedState, webhook: ReceiveWebhook) -> Result<impl Reply, Rejection> {
    info!("Recieved webhook for {} | from: {}", webhook.final_destination, webhook.final_source);

    let zw = ZW::with_key(&*SESSION_KEY);

    Ok("OK")
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Zipwhip Recieve Webhook
struct ReceiveWebhook {
    body: String,
    body_size: usize,
    address: String,
    final_source: String,
    final_destination: String,
    message_type: String,
    fingerprint: String,
    id: usize,
    cc: Option<String>,
    bcc: Option<String>,
    read: bool,
    contact_id: usize,
    scheduled_date: Option<String>,
    device_id: Option<usize>,
    date_deleted: Option<String>,
    message_transport: usize,
    has_attachment: bool,
    date_created: Option<String>,
    deleted: bool,
    date_read: Option<String>,
    status_code: usize
}

/// State filter
fn with_state(
    state: SharedState,
) -> impl Filter<Extract = (SharedState,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || state.clone())
}
