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
            timestamp: String::new(),
            extended_stat: String::new(),
            extended_stat_value: String::new(),
            stat: String::new(),
            stat_value: String::new(),
        }
    }

    pub fn name(mut self, arg: String) -> MetricOut {
        self.name = arg;
        self
    }

    pub fn timestamp(mut self, arg: String) -> MetricOut {
        self.timestamp = arg;
        self
    }

    pub fn extended_stat(mut self, arg: String) -> MetricOut {
        self.extended_stat = arg;
        self
    }

    pub fn extended_stat_value(mut self, arg: String) -> MetricOut {
        self.extended_stat_value = arg;
        self
    }

    pub fn stat_value(mut self, arg: String) -> MetricOut {
        self.stat_value = arg;
        self
    }

    pub fn stat(mut self, arg: String) -> MetricOut {
        self.stat = arg;
        self
    }
}
