use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    let content = web_scraper::get_all_content().await.unwrap();
    let yaml = serde_yaml::to_string(&content).unwrap();
    let mut file = File::create("content.yaml").await.unwrap();
    file.write_all(yaml.as_bytes()).await.unwrap();
}
