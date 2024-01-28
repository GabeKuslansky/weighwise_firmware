use rand::Rng;
use std::thread::sleep;
use std::time::Duration;

pub struct LoadCellParser {
    is_listening: bool,
}

impl LoadCellParser {
    const DEBOUNCE_INTERVAL_MS: u64 = 100;
    const BUFFER_PERIOD_MS: u64 = 2500;
    const REQUIRED_SAME_WEIGHTS: u64 = Self::DEBOUNCE_INTERVAL_MS / Self::BUFFER_PERIOD_MS;
    const FLASH_INTERVAL: u64 = 500;
    const FLASH_CYCLES: u8 = 3;

    const PENDING_WEIGHT: f32 = -1.0;

    pub fn new() -> LoadCellParser {
        LoadCellParser {
            is_listening: false,
        }
    }

    pub fn start_listening(&mut self) {
        self.is_listening = true;
        let mut same_counter: u64 = 0;
        let mut prev_weight: f32 = Self::PENDING_WEIGHT;

        while self.is_listening {
            sleep(Duration::from_millis(Self::DEBOUNCE_INTERVAL_MS));
            let mut rng = rand::thread_rng();
            let random_num = rng.gen_range(144.22..=144.25);

            let rounded_weight: f32 = Self::round_to_two_decimals(random_num);
            println!("{}", rounded_weight);

            if prev_weight == rounded_weight {
                same_counter += 1;
                if same_counter == Self::REQUIRED_SAME_WEIGHTS {
                    self.stop_listening();
                    if prev_weight > 0.0 {
                        self.confirm_weight(prev_weight);
                    }
                }
            } else {
                same_counter = 0;
            }
            prev_weight = rounded_weight;
        }
    }

    pub fn stop_listening(&mut self) {
        self.is_listening = false;
    }

    pub fn confirm_weight(&mut self, weight: f32) {
        let mut flash_cycles = Self::FLASH_CYCLES;

        println!("Weight Confirmed: {}", weight);
        while flash_cycles > 0 {
            println!("");
            sleep(Duration::from_millis(Self::FLASH_INTERVAL));
            println!("{}", weight);
            sleep(Duration::from_millis(Self::FLASH_INTERVAL));
            flash_cycles -= 1;
        }

        sleep(Duration::from_millis(3000));
        println!("LED OFF");
    }

    fn round_to_two_decimals(val: f32) -> f32 {
        let rounded_value = (val * 100.0 + 0.5).floor() / 100.0;
        rounded_value
    }
}
