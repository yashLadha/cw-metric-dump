## CW Metric Dump

This is a hobby project for my own use primarily. I was facing issue with creating a csv from `CW Dashboard`
or `aws` cli to get the metric data for a time range and statistic across mutiple namespace and dimensions.

## Motivation

When I was trying to do some analysis on the traffic pattern for API's and the latency numbers, I was not able to find
a straightforward way to download the data offline and perform analysis on it. It requires downloading csv for each metric
and formatting the data, which is too much work 😅.

## Build

You can build the binary simply using `cargo build` and for release binary you can use `--release` flag.

## TODO
* [ ] Add examples and documentation
* [ ] Add Unit tests 😅
