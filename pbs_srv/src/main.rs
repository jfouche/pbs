use tracing::error;

#[tokio::main]
async fn main() {
    if let Err(e) = pbs_srv::serve(3030).await {
        error!("{e:?}");
    }
}
