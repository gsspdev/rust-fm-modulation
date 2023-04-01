#[derive(Debug)]
struct Oscillator {
    amp: f32,
    freq: f32,
    on_off: bool,
    input: Option<Box<Oscillator>>,
}

impl Oscillator {
    fn new(amp: f32, freq: f32) -> Oscillator {
        Oscillator {
            amp,
            freq,
            on_off: false,
            input: None,
        }
    }

    fn with_input(amp: f32, freq: f32, input: Oscillator) -> Oscillator {
        Oscillator {
            amp,
            freq,
            on_off: false,
            input: Some(Box::new(input)),
        }
    }

    fn frequency_modulation(&self, time: f32) -> f32 {
        let input_freq = match &self.input {
            Some(input_oscillator) => input_oscillator.frequency_modulation(time) * self.amp,
            None => 0.0,
        };

        (2.0 * std::f32::consts::PI * (self.freq + input_freq) * time).sin()
    }
}

fn main() {
    let osc1 = Oscillator::new(0.5, 440.0);
    let osc2 = Oscillator::with_input(0.25, 240.0, osc1);

    // Use frequency_modulation() method to compute the output of osc2 at a given time
    let time = 5.0; // example time value
    let output = osc2.frequency_modulation(time);
    println!("Output at time {}: {}", time, output);
}

// #[derive(Debug)]
// struct Oscillator {
//     amp: f32,
//     freq: f32,
//     on_off: bool,
//     input: Oscillator
// }

// fn main() {
//    let osc1 = Oscillator {
//         amp: 0.5,
//         freq: 440.0,
//    };

//    let osc2 = Oscillator {
//         amp: 0.25,  
//         freq: 240.0,
//    };

// }
