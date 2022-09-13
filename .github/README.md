## CloudWatch Metric Dump

This is a hobby project for my own use primarily. I was facing issue with creating a csv from `CloudWatch Dashboard`
or `aws` CLI to get the metric data for a time range and statistic across multiple namespace and dimensions.

## Motivation

When I was trying to do some analysis on the traffic pattern for API's and the latency numbers, I was not able to find
a straightforward way to download the data offline and perform analysis on it. It requires downloading csv for each metric
and formatting the data, which is too much work 😅.

## Build

You can build the binary simply using `cargo build` and for release binary you can use `--release` flag.

## Usage

Binary has following options:

```
cw-metric-dump 

USAGE:
    cw-metric-dump [OPTIONS]

OPTIONS:
    -c, --csv <CSV>                        [possible values: true, false]
        --co <CONFIG_FILE>                 
    -d, --dimensions <DIMENSIONS>          
    -e, --extended-stat <EXTENDED_STAT>    
    -h, --help                             Print help information
    -n, --name <NAME>                      
        --ns <NAMESPACE>                   
        --period <PERIOD>                  
    -r, --region <REGION>                  
    -s, --stat <STAT>                      
        --start-time <START_TIME>          
```


Config file specification:

It is a valid json file which at root will have an array. Each entry inside array will be an object with following
keys:

```json
{
    name: string,
    namespace: string,
    region: string,
    extended_stat?: string,
    stat?: string,
    dimensions?: [ `key=value`, ... ],
    period?: int,
    start_time: string
}
```

## Examples

### Find the p95 of a metric for 1 week duration from now

You can use the below config file to fetch values for a metric with name `metricName` and namespace
`metricNamespace`. This will fetch the data points for past 1 week from current time and aggregate them by
1 day duration (86400 seconds).

```json
[
  {
    "name": "metricName",
    "namespace": "metricNamespace",
    "extended_stat": "p95",
    "start_time": "w:1",
    "period": 86400
  }
]
```

Corresponding CLI command is:

```sh
$ cw-metric-dump -n metricName --ns metricNamespace --period 86400 -e p95 --start-time 'w:1'
```

### Find the p95 of a metric with custom dimension from now

You can use the below config file to fetch the values with custom dimension and P95 for a metric.

```json
[
  {
    "name": "metricName",
    "namespace": "metricNamespace",
    "extended_stat": "p95",
    "start_time": "w:1",
    "period": 86400,
    "dimensions": ["EventType=SeverityHigh", "Location=USE"]
  }
]
```
Corresponding CLI command is:

```sh
$ cw-metric-dump -n metricName --ns metricNamespace --period 86400 -e p95 --start-time 'w:1' -d 'EventType=SeverityHigh' -d 'Location=USE'
```

### Find the Count for group of metrics and present on console

You can use the following config to find the count statistic for a metric

```json
[
  {
    "name": "metricName",
    "namespace": "metricNamespace",
    "stat": "SampleCount",
    "start_time": "w:6",
    "period": 86400,
    "dimensions": ["EventType=LowSeverity"]
  }
]
```
Corresponding CLI command is:

```sh
$ cw-metric-dump -n metricName --ns metricNamespace --period 86400 -s SampleCount --start-time 'w:6' -d 'EventType=LowSeverity'
```

### Find the Max for group of metrics and dump them in csv

You can use the following config to fetch the metric max statistic and dump them into csv. For writing to a csv file
you just need to enable the flag `--csv true` and it will write the data points to a csv file.

```json
[
  {
    "name": "metricName",
    "namespace": "metricNamespace",
    "stat": "Maximum",
    "start_time": "w:3",
    "period": 86400
  }
]
```

Corresponding CLI command is:

```sh
$ cw-metric-dump -n metricName --ns metricNamespace --period 86400 -s Maximum --start-time 'w:3'
```
