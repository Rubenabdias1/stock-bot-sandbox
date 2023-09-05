use rand::Rng;

fn round_to_8_decimals(value: f64) -> f64 {
    format!("{:.8}", value).parse().unwrap_or(value)
}
pub enum TimeFrame {
    OneMinute,
    FiveMinutes,
    OneHour,
    OneDay,
    OneMonth,
}

pub enum CandlestickState {
    Open,
    Closed,
}

pub struct Candlestick {
    open: f64,
    close: f64,
    high: f64,
    low: f64,
    time_frame: TimeFrame,
    timestamp: Option<i64>,
    number_of_trades: u32,
    state: CandlestickState,
}

pub struct IchimokuCloudParameters {
    short_period: usize,
    medium_period: usize,
    long_period: usize,
}

pub struct IchimokuCloudState {
    short_period_min: f64,
    short_period_max: f64,
    medium_period_min: f64,
    medium_period_max: f64,
    long_period_min: f64,
    long_period_max: f64,
    parameters: IchimokuCloudParameters,
}

pub struct IchimokuCloudResult {
    tenkan_sen: f64,
    kijun_sen: f64,
    senkou_span_a: f64,
    senkou_span_b: f64,
    chikou_span: f64,
}

pub struct IchimokuCloud {
    short_period_min: f64,
    short_period_max: f64,
    medium_period_min: f64,
    medium_period_max: f64,
    long_period_min: f64,
    long_period_max: f64,
    parameters: IchimokuCloudParameters,
    num_processed: usize, // Add this field to keep track of the number of processed candlesticks
}

impl IchimokuCloud {
    pub fn new(params: IchimokuCloudParameters) -> Self {
        Self {
            short_period_min: f64::MAX,
            short_period_max: f64::MIN,
            medium_period_min: f64::MAX,
            medium_period_max: f64::MIN,
            long_period_min: f64::MAX,
            long_period_max: f64::MIN,
            parameters: params,
            num_processed: 0,
        }
    }

    pub fn initialize<'a>(
        &mut self,
        candlesticks: &'a [Candlestick],
    ) -> Vec<(&'a Candlestick, Option<IchimokuCloudResult>)> {
        let mut results: Vec<(&'a Candlestick, Option<IchimokuCloudResult>)> = Vec::new();

        for candle in candlesticks.iter() {
            self.num_processed += 1;
            // Update min and max values for all periods.
            // This is a simplified example; you might have different logic to update these based on the actual candlestick data.
            self.short_period_min = self.short_period_min.min(candle.low);
            self.short_period_max = self.short_period_max.max(candle.high);
            self.medium_period_min = self.medium_period_min.min(candle.low);
            self.medium_period_max = self.medium_period_max.max(candle.high);
            self.long_period_min = self.long_period_min.min(candle.low);
            self.long_period_max = self.long_period_max.max(candle.high);

            // Calculate Ichimoku Cloud values
            // This is a simplified example; your actual calculations may differ.
            let tenkan_sen = (self.short_period_max + self.short_period_min) / 2.0;
            let kijun_sen = (self.medium_period_max + self.medium_period_min) / 2.0;
            let senkou_span_a = (tenkan_sen + kijun_sen) / 2.0;
            let senkou_span_b = (self.long_period_max + self.long_period_min) / 2.0;
            let chikou_span = candle.close; // This is just a placeholder; real calculation might differ

            let ichimoku_result = if self.num_processed >= self.parameters.long_period {
                Some(IchimokuCloudResult {
                    tenkan_sen: round_to_8_decimals(tenkan_sen),
                    kijun_sen: round_to_8_decimals(kijun_sen),
                    senkou_span_a: round_to_8_decimals(senkou_span_a),
                    senkou_span_b: round_to_8_decimals(senkou_span_b),
                    chikou_span: round_to_8_decimals(chikou_span),
                })
            } else {
                None
            };

            // Store the result
            results.push((candle, ichimoku_result));
        }

        results
    }

    // Calculate the Ichimoku Cloud values for a given candlestick.
    // If the candlestick is closed, also update the state.
    pub fn calculate(&mut self, candle: &Candlestick) -> Option<IchimokuCloudResult> {
        // Temporary variables to hold min/max values
        let mut temp_short_min = self.short_period_min;
        let mut temp_short_max = self.short_period_max;
        let mut temp_medium_min = self.medium_period_min;
        let mut temp_medium_max = self.medium_period_max;
        let mut temp_long_min = self.long_period_min;
        let mut temp_long_max = self.long_period_max;

        // Update temporary min/max values
        temp_short_min = temp_short_min.min(candle.low);
        temp_short_max = temp_short_max.max(candle.high);
        temp_medium_min = temp_medium_min.min(candle.low);
        temp_medium_max = temp_medium_max.max(candle.high);
        temp_long_min = temp_long_min.min(candle.low);
        temp_long_max = temp_long_max.max(candle.high);

        // Calculate Ichimoku Cloud values based on the temporary state
        let tenkan_sen = (temp_short_max + temp_short_min) / 2.0;
        let kijun_sen = (temp_medium_max + temp_medium_min) / 2.0;
        let senkou_span_a = (tenkan_sen + kijun_sen) / 2.0;
        let senkou_span_b = (temp_long_max + temp_long_min) / 2.0;
        let chikou_span = candle.close; // Placeholder, real calculation may differ

        // If the candlestick is closed, update the state
        if let CandlestickState::Closed = candle.state {
            self.short_period_min = temp_short_min;
            self.short_period_max = temp_short_max;
            self.medium_period_min = temp_medium_min;
            self.medium_period_max = temp_medium_max;
            self.long_period_min = temp_long_min;
            self.long_period_max = temp_long_max;
            self.num_processed += 1;
        }

        // Return the calculated values
        if self.num_processed >= self.parameters.long_period {
            Some(IchimokuCloudResult {
                tenkan_sen: round_to_8_decimals(tenkan_sen),
                kijun_sen: round_to_8_decimals(kijun_sen),
                senkou_span_a: round_to_8_decimals(senkou_span_a),
                senkou_span_b: round_to_8_decimals(senkou_span_b),
                chikou_span: round_to_8_decimals(chikou_span),
            })
        } else {
            None
        }
    }
}

fn main() {
    // Your existing structs, enums, and impl blocks go here

    // Create an empty vector to store candlesticks
    let mut candlesticks = Vec::new();

    // Generate 60 random candlesticks
    let mut rng = rand::thread_rng();
    for i in 0..256 {
        let open: f64 = rng.gen_range(90.0..130.0);
        let close: f64 = rng.gen_range(90.0..130.0);
        let high: f64 = f64::max(open, close) + rng.gen_range(1.0..5.0);
        let low: f64 = f64::min(open, close) - rng.gen_range(1.0..5.0);

        let candle = Candlestick {
            open,
            close,
            high,
            low,
            time_frame: TimeFrame::OneMinute,
            timestamp: Some(1632405600 + i * 60),
            number_of_trades: rng.gen_range(80..120),
            state: CandlestickState::Closed,
        };

        candlesticks.push(candle);
    }

    // Initialize Ichimoku Cloud parameters
    let ichimoku_parameters = IchimokuCloudParameters {
        short_period: 9,
        medium_period: 26,
        long_period: 52,
    };

    // Initialize Ichimoku Cloud object
    let mut ichimoku = IchimokuCloud::new(ichimoku_parameters);

    // Initialize Ichimoku Cloud with existing candlestick data
    let initial_results = ichimoku.initialize(&candlesticks);

    // Display the initial results
    for (_, result) in initial_results.iter() {
        // println!("Candle close price: {}", candle.close);
        if let Some(result) = result {
            println!(
                "Ichimoku: Tenkan Sen: {}, Kijun Sen: {}, Senkou Span A: {}, Senkou Span B: {}, Chikou Span: {}",
                result.tenkan_sen, result.kijun_sen, result.senkou_span_a, result.senkou_span_b, result.chikou_span
            );
        }
    }
    print!("{}", ichimoku.num_processed);
}
