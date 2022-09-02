use clap::Parser;
use serde::{Serialize, Deserialize};

#[derive(Parser, Debug, Serialize, Deserialize)]
pub struct MetricsObj {
    #[clap(short, long, value_parser)]
    pub name: Option<String>,

    #[clap(long = "co", value_parser)]
    pub config_file: Option<String>,

    #[clap(short, long, value_parser)]
    pub csv: Option<bool>,

    #[clap(long = "ns", value_parser)]
    pub namespace: Option<String>,

    #[clap(short, long, value_parser)]
    pub region: Option<String>,

    #[clap(short, long, value_parser)]
    pub extended_stat: Option<String>,

    #[clap(short, long, value_parser)]
    pub stat: Option<String>,

    #[clap(short, long, value_parser)]
    pub dimensions: Option<Vec<String>>,
}
