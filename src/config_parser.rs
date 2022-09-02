use super::metrics_obj::MetricsObj;
use tokio::fs;

pub async fn parse_config(config_file: &str) -> Option<Vec<MetricsObj>> {
    match fs::read(config_file).await {
        Ok(it) => {
            let content: String = String::from_utf8_lossy(&it).parse().unwrap();
            let metrics: Vec<MetricsObj> = serde_json::from_str(&content).unwrap();
            Some(metrics)
        }
        Err(_) => panic!("Invalid config file provided"),
    }
}
