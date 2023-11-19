
#[derive(Default)]
struct Format {
    percentage: bool,
    order_of_magnitude: bool,
    commas: bool,
    signed: bool,
    decimals: bool,
    prefix: Option<String>,
    suffix: Option<String>,
    nan_str: Option<String>,
    // scientific: bool,
    // sigfigs: Option<usize>,
    // leading_zeros
}

impl Format {
    fn new() -> Format {
        Format::default()
    }

    fn to_pattern(&self) -> &str {
        // [[fill]align][sign][symbol][0][width][,][.precision][type]
    }

    // Setters for each property
    fn percentage(mut self, value: bool) -> Self {
        self.percentage = value;
        self
    }

    fn order_of_magnitude(mut self, value: bool) -> Self {
        self.order_of_magnitude = value;
        self
    }

    fn commas(mut self, value: bool) -> Self {
        self.commas = value;
        self
    }

    fn signed(mut self, value: bool) -> Self {
        self.signed = value;
        self
    }

    fn decimals(mut self, value: bool) -> Self {
        self.decimals = value;
        self
    }

    // Builds the Format struct
    fn build(self) -> Format {
        Format {
            percentage: self.percentage,
            order_of_magnitude: self.order_of_magnitude,
            commas: self.commas,
            signed: self.signed,
            decimals: self.decimals,
        }
    }
}

// Implement Default for FormatBuilder
impl Default for FormatBuilder {
    fn default() -> Self {
        FormatBuilder {
            percentage: false,
            order_of_magnitude: false,
            commas: false,
            signed: false,
            decimals: false,
        }
    }
}


Format::new().oom()

