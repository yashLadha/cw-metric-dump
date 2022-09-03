mod config_parser;
mod log;
mod metrics_obj;
mod start_time_parser;

use aws_config::profile::ProfileFileCredentialsProvider;
use aws_sdk_cloudwatch::{model::Dimension, types::DateTime, Client, Error, Region};
use chrono::{TimeZone, Utc};
use clap::Parser;
use config_parser::parse_config;
use futures::future::join_all;
use metrics_obj::MetricsObj;
use start_time_parser::start_time_parse;

use crate::metrics_obj::MetricOut;

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
            let res: Vec<Vec<CWMetricDataPoint>> = join_all(value_vec)
                .await
                .into_iter()
                .filter(|item| item.is_ok())
                .map(|item| item.unwrap())
                .collect();
            (0..metrics.len()).for_each(|index| {
                res.get(index).into_iter().for_each(|values| {
                    for item in values {
                        build_metric_output(&metrics[index], item);
                    }
                });
            });
        }
    }
    Ok(())
}

async fn get_single_metric_info(args: &MetricsObj, client: &Client) -> Result<(), Error> {
    let values = fetch_metric_values(client, args).await?;

    for value in values {
        build_metric_output(args, &value);
    }

    Ok(())
}

fn build_metric_output(metrics_arg: &MetricsObj, value: &CWMetricDataPoint) -> MetricOut {
    let mut metric_out = MetricOut::builder();
    metric_out = metric_out.name(metrics_arg.name.as_ref().unwrap().to_string());
    metric_out = metric_out.timestamp(human_readable_time(value.timestamp.unwrap()));
    if let Some(extended_stat_str) = metrics_arg.extended_stat.as_ref() {
        if let Some(extended_stat_map) = value.extended_statistics.as_ref() {
            if let Some(extended_stat_val) = extended_stat_map.get(extended_stat_str) {
                metric_out = metric_out.extended_stat(extended_stat_str.to_string());
                metric_out =
                    metric_out.extended_stat_value(extended_stat_val.to_owned().to_string());
            }
        }
    }
    if let Some(stat) = metrics_arg.stat.as_ref() {
        metric_out = metric_out.stat(stat.to_string());
        match stat.as_str() {
            "SampleCount" => {
                if let Some(sample_cnt_val) = value.sample_count().as_ref() {
                    metric_out = metric_out.stat_value(sample_cnt_val.to_string());
                }
            }
            "Sum" => {
                if let Some(sum_val) = value.sum().as_ref() {
                    metric_out = metric_out.stat_value(sum_val.to_string());
                }
            }
            "Maximum" => {
                if let Some(max_val) = value.maximum().as_ref() {
                    metric_out = metric_out.stat_value(max_val.to_string());
                }
            }
            "Average" => {
                if let Some(avg_val) = value.average().as_ref() {
                    metric_out = metric_out.stat_value(avg_val.to_string());
                }
            }
            "Minimum" => {
                if let Some(min_val) = value.minimum().as_ref() {
                    metric_out = metric_out.stat_value(min_val.to_string());
                }
            }
            _ => panic!("Unsupported stat {} passed in metric", stat),
        }
    }
    metric_out
}

fn build_dimensions_vec(args: &MetricsObj) -> Option<Vec<Dimension>> {
    if args.dimensions.as_ref().is_some() {
        return Some(
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
                .collect::<Vec<Dimension>>(),
        );
    }
    None
}

async fn fetch_metric_values(
    client: &Client,
    args: &MetricsObj,
) -> Result<Vec<CWMetricDataPoint>, Error> {
    let dimensions_vec = build_dimensions_vec(args);
    let mut cw_query_client = client.get_metric_statistics().end_time(current_time());

    if args.period.as_ref().is_some() {
        cw_query_client = cw_query_client.set_period(args.period);
    }

    if dimensions_vec.is_some() {
        cw_query_client = cw_query_client.set_dimensions(dimensions_vec);
    }

    if args.name.as_ref().is_some() {
        cw_query_client = cw_query_client.metric_name(args.name.as_ref().unwrap().as_str());
    }

    if args.namespace.as_ref().is_some() {
        cw_query_client = cw_query_client.namespace(args.namespace.as_ref().unwrap().as_str());
    }

    if args.period.as_ref().is_some() {
        cw_query_client = cw_query_client.set_period(args.period);
    }

    if args.stat.as_ref().is_some() {
        cw_query_client = cw_query_client.statistics(
            aws_sdk_cloudwatch::model::Statistic::Unknown(args.stat.as_ref().unwrap().to_string()),
        );
    }

    if args.start_time.as_ref().is_some() {
        cw_query_client =
            cw_query_client.set_start_time(start_time_parse(args.start_time.as_ref()));
    }

    if args.extended_stat.as_ref().is_some() {
        cw_query_client =
            cw_query_client.extended_statistics(args.extended_stat.as_ref().unwrap().as_str());
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
