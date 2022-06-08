use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    #[clap(short, long, value_parser)]
    pub metric_name: Option<String>,

    #[clap(long = "co", value_parser)]
    pub config_file: Option<String>,

    #[clap(short, long, value_parser)]
    pub csv: bool,

    #[clap(short, long, value_parser)]
    pub namespace: Option<String>,

    #[clap(short, long, value_parser, default_value = "ap-south-1")]
    pub region: String,

    #[clap(short, long, value_parser)]
    pub extended_stat: Option<String>,

    #[clap(short, long, value_parser)]
    pub stat: Option<String>,

    #[clap(long = "du", value_parser, default_value = "300")]
    pub duration: i32,

    #[clap(short, long, value_parser)]
    pub dimensions: Vec<String>,

    #[clap(long = "st", value_parser, default_value = "10")]
    pub start_time: String,
}
