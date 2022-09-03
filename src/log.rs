use std::fs::File;

use csv::{Error, Writer};

use super::metrics_obj::MetricOut;

pub struct Log {
    csv: bool,
    csv_writer: Option<Writer<File>>,
}

impl Log {
    pub fn new() -> Log {
        Log {
            csv: false,
            csv_writer: None,
        }
    }

    pub fn enable_csv(&mut self) {
        self.csv = true;
        self.csv_writer = if let Ok(writer) = Writer::from_path("metric_dump.csv") {
            Some(writer)
        } else {
            None
        }
    }

    pub fn write(&mut self, metric_out: &MetricOut) -> Result<(), Error> {
        if self.csv {
            if let Some(writer) = self.csv_writer.as_mut() {
                writer.serialize(metric_out)?;
            }
        } else {
            println!("{:?}", metric_out);
        }
        Ok(())
    }

    pub fn close(&mut self) -> Result<(), Error> {
        if self.csv {
            if let Some(writer) = self.csv_writer.as_mut() {
                writer.flush()?;
            }
        }
        Ok(())
    }
}
