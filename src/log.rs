use csv::Writer;

pub struct Log {
    csv: bool,
}

impl Log {
    pub fn write(self, message: &str) {
        if self.csv {
        }
        println!("{}", message);
    }
}
