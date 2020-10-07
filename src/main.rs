use hound::WavReader;
use plotters;
use rustfft::num_complex::Complex;
use rustfft::FFTplanner;
use std::env::args;

fn find_spectral_peak(filename: &str) -> Option<f32> {
    let mut reader = WavReader::open(filename).expect("Failed to open WAV file");
    let num_samples = reader.len() as usize;
    let mut fft_planer = FFTplanner::new(false); // not inverse FFT;
    let fft = fft_planer.plan_fft(num_samples);
    let mut signal = reader
        .samples::<i16>()
        .map(|x| Complex::new(x.unwrap() as f32, 0f32))
        .collect::<Vec<_>>();
    let mut spectrum = signal.clone();
    fft.process(&mut signal[..], &mut spectrum[..]);
    let max_peak = spectrum
        .iter()
        .take(num_samples / 2)
        .enumerate()
        .max_by_key(|&(_, freq)| freq.norm() as u32);
    if let Some((i, _)) = max_peak {
        let bin = 44100f32 / num_samples as f32;
        Some(i as f32 * bin)
    } else {
        None
    }
}

const DRIFT_PERCENT: u32 = 5;
struct FreqencyMap {
    key: &'static str,
    freq: f32,
}

// Refrence: http://www.contrabass.com/pages/frequency.html
static FREQENCY_MAPS: [FreqencyMap; 12] = [
    FreqencyMap {
        key: "C4",
        freq: 261.63,
    },
    FreqencyMap {
        key: "C#4",
        freq: 277.18,
    },
    FreqencyMap {
        key: "D4",
        freq: 293.66,
    },
    FreqencyMap {
        key: "D#4",
        freq: 311.13,
    },
    FreqencyMap {
        key: "E4",
        freq: 329.63,
    },
    FreqencyMap {
        key: "F4",
        freq: 349.23,
    },
    FreqencyMap {
        key: "F#4",
        freq: 369.99,
    },
    FreqencyMap {
        key: "G4",
        freq: 392.00,
    },
    FreqencyMap {
        key: "G#4",
        freq: 415.30,
    },
    FreqencyMap {
        key: "A4",
        freq: 440.0,
    },
    FreqencyMap {
        key: "A#4",
        freq: 466.16,
    },
    FreqencyMap {
        key: "B4",
        freq: 493.88,
    },
];

fn frequency_to_key(freqency: f32) -> Option<String> {
    for freq_map in &FREQENCY_MAPS {
        if ((freqency - freq_map.freq).abs() / freq_map.freq) < DRIFT_PERCENT as f32 {
            return Some(freq_map.key.to_string());
        }
    }
    None
}

//fn to_png(

fn main() {
    let file_path = args().nth(1).expect("Please provide a WAV file");
    println!("{}", &file_path);

    let freq = find_spectral_peak(&file_path).unwrap();
    println!("spectral_peak {:?}", freq);
    println!("key {}", frequency_to_key(freq).unwrap());
}
