mod config_parser;
mod metrics_obj;

use aws_config::profile::ProfileFileCredentialsProvider;
use aws_sdk_cloudwatch::{model::Dimension, types::DateTime, Client, Error, Region};
use chrono::{Duration, TimeZone, Utc};
use clap::Parser;
use config_parser::parse_config;
use futures::future::join_all;
use metrics_obj::MetricsObj;

type CWMetricDataPoint = aws_sdk_cloudwatch::model::Datapoint;

async fn get_peak_time(client: &Client, args: &MetricsObj) -> Result<(), Error> {
    let config_file = args.config_file.as_ref();
    if config_file.is_none() {
        get_single_metric_info(args, client).await?;
    } else {
        let is_metrics = parse_config(config_file.unwrap().as_str()).await;
        if let Some(metrics) = is_metrics {
            let value_vec: Vec<_> = metrics
                .iter()
                .map(|metric| fetch_metric_values(client, metric))
                .collect();
            let _res = join_all(value_vec).await;
            println!("{:?}", _res);
        }
    }
    Ok(())
}

async fn get_single_metric_info(args: &MetricsObj, client: &Client) -> Result<(), Error> {
    let values = fetch_metric_values(client, args).await?;

    for value in values {
        println!("{:?}", value);
    }

    Ok(())
}

fn build_dimensions_vec(args: &MetricsObj) -> Vec<Dimension> {
    let dimensions_vec = if args.dimensions.as_ref().is_some() {
        args.dimensions
            .as_ref()
            .unwrap()
            .iter()
            .map(|value| {
                let dimension_vec = value.split('=').collect::<Vec<&str>>();
                Dimension::builder()
                    .name(dimension_vec[0])
                    .value(dimension_vec[1])
                    .build()
            })
            .collect::<Vec<Dimension>>()
    } else {
        Vec::new()
    };
    dimensions_vec
}

async fn fetch_metric_values(
    client: &Client,
    args: &MetricsObj,
) -> Result<Vec<CWMetricDataPoint>, Error> {
    let metric_name = args.name.as_ref().unwrap().as_str();
    let metric_namespace = args.namespace.as_ref().unwrap().as_str();
    let stat = args.stat.as_ref();
    let extended_stat = args.extended_stat.as_ref();
    let dimensions_vec = build_dimensions_vec(args);
    let mut cw_query_client = client
        .get_metric_statistics()
        .namespace(metric_namespace)
        .metric_name(metric_name)
        .start_time(decrease_time_by_days(20))
        .end_time(current_time())
        .set_dimensions(Some(dimensions_vec))
        .period(24 * 3600);

    if let Some(stat_str) = stat {
        cw_query_client = cw_query_client.statistics(
            aws_sdk_cloudwatch::model::Statistic::Unknown(stat_str.to_string()),
        );
    }

    if let Some(extended_stat_str) = extended_stat {
        cw_query_client = cw_query_client.extended_statistics(extended_stat_str);
    }

    let data_points = cw_query_client.send().await?;
    let mut values = data_points.datapoints.unwrap();
    values.sort_unstable_by(|a, b| {
        a.timestamp
            .unwrap()
            .secs()
            .cmp(&b.timestamp.unwrap().secs())
    });
    Ok(values)
}

fn decrease_time_by_days(days: i64) -> DateTime {
    DateTime::from_millis((Utc::now() - Duration::days(days)).timestamp_millis())
}

fn current_time() -> DateTime {
    DateTime::from_millis(Utc::now().timestamp_millis())
}

fn human_readable_time(datetime: DateTime) -> String {
    let dt = Utc.timestamp(datetime.secs(), 0);
    dt.to_rfc3339()
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = MetricsObj::parse();
    let default_region = Region::new(
        args.region
            .as_ref()
            .unwrap_or(&"ap-south-1".to_string())
            .to_string(),
    );
    let default_profile = option_env!("AWS_DEFAULT_PROFILE");
    let profile_cred_provider = match default_profile {
        Some(profile_name) => ProfileFileCredentialsProvider::builder()
            .profile_name(profile_name)
            .build(),
        None => ProfileFileCredentialsProvider::builder()
            .profile_name("DEFAULT")
            .build(),
    };
    let shared_config = aws_config::from_env()
        .region(default_region)
        .credentials_provider(profile_cred_provider)
        .load()
        .await;
    let cloudwatch_client = Client::new(&shared_config);
    get_peak_time(&cloudwatch_client, &args).await
}
