use std::fmt::{self, Formatter, Display}

struct City {
    name: &'static str,
    lat: f32,
    lon: f32,
}

impl Display for City {
    // 'f' is abuffer, and this method must write the formatted string into it
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 {'N'} else {'S'};
        let lon_c = if self.lon >= 0.0 {'N'} else {'W'};

        write!(f, "{}: {.:3} ")
    }
}