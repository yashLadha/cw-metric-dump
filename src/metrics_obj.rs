use clap::Parser;
use serde::{Deserialize, Serialize};

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

    #[clap(long, value_parser)]
    pub period: Option<i32>,

    #[clap(long, value_parser)]
    pub start_time: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct MetricOut {
    name: String,
    dimension: String,
    timestamp: String,
    extended_stat: String,
    extended_stat_value: String,
    stat: String,
    stat_value: String,
}

impl MetricOut {
    pub fn builder() -> MetricOut {
        MetricOut {
            name: String::new(),
            dimension: String::new(),
            timestamp: String::new(),
            extended_stat: String::new(),
            extended_stat_value: String::new(),
            stat: String::new(),
            stat_value: String::new(),
        }
    }

    pub fn name(&mut self, arg: String) {
        self.name = arg;
    }

    pub fn timestamp(&mut self, arg: String) {
        self.timestamp = arg;
    }

    pub fn dimension(&mut self, arg: String) {
        self.dimension = arg;
    }

    pub fn extended_stat(&mut self, arg: String) {
        self.extended_stat = arg;
    }

    pub fn extended_stat_value(&mut self, arg: String) {
        self.extended_stat_value = arg;
    }

    pub fn stat_value(&mut self, arg: String) {
        self.stat_value = arg;
    }

    pub fn stat(&mut self, arg: String) {
        self.stat = arg;
    }
}
