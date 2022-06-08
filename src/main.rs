mod config_parser;
mod metric_obj;
mod args_obj;

use clap::Parser;
use futures::future::join_all;
use args_obj::Args;
use aws_config::profile::ProfileFileCredentialsProvider;
use aws_sdk_cloudwatch::{model::Dimension, types::DateTime, Client, Error, Region};
use chrono::{Duration, TimeZone, Utc};
use config_parser::parse_config;

async fn get_peak_time(client: &Client, args: &Args) -> Result<(), Error> {
    let config_file = args.config_file.as_ref();
    if config_file.is_none() {
        get_single_metric_info(args, client).await?;
    } else {
        let is_metrics = parse_config(config_file.unwrap().as_str()).await;
        if let Some(metrics) = is_metrics {
            let value_vec: Vec<_> = metrics
                .iter()
                .map(|metric| {
                    let dimensions_vec: Vec<Dimension> = if metric.dimensions.as_ref().is_some() {
                        metric
                            .dimensions
                            .as_ref()
                            .unwrap()
                            .iter()
                            .map(|(k, v)| Dimension::builder().name(k).value(v).build())
                            .collect()
                    } else {
                        Vec::new()
                    };
                    fetch_metric_values(
                        client,
                        &metric.namespace,
                        &metric.name,
                        metric.stat.as_ref(),
                        metric.extended_stat.as_ref(),
                        dimensions_vec,
                    )
                })
                .collect();
            let _res = join_all(value_vec).await;
            println!("{:?}", _res);
        }
    }
    Ok(())
}

async fn get_single_metric_info(args: &Args, client: &Client) -> Result<(), Error> {
    let metric_name = args.metric_name.as_ref().unwrap().as_str();
    let metric_namespace = args.namespace.as_ref().unwrap().as_str();
    let stat = args.stat.as_ref();
    let extended_stat = args.extended_stat.as_ref();
    let dimensions_vec = args
        .dimensions
        .iter()
        .map(|value| {
            let dimension_vec = value.split('=').collect::<Vec<&str>>();
            Dimension::builder()
                .name(dimension_vec[0])
                .value(dimension_vec[1])
                .build()
        })
        .collect::<Vec<Dimension>>();
    let values = fetch_metric_values(
        client,
        metric_namespace,
        metric_name,
        stat,
        extended_stat,
        dimensions_vec,
    )
    .await?;

    for value in values {
        println!("{:?}", value);
    }

    Ok(())
}

async fn fetch_metric_values(
    client: &Client,
    metric_namespace: &str,
    metric_name: &str,
    stat: Option<&String>,
    extended_stat: Option<&String>,
    dimensions_vec: Vec<Dimension>,
) -> Result<Vec<aws_sdk_cloudwatch::model::Datapoint>, Error> {
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
    let args = Args::parse();
    let default_region = Region::new(args.region.to_string());
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
