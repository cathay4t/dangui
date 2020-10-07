mod plot;

use hound::WavReader;
use plot::show_plot;
use rustfft::num_complex::Complex;
use rustfft::FFTplanner;
use std::env::args;

const CHUNK_SIZE: usize = 1024 * 5;
const SAMPLE_RATE: f32 = 44100.0;

fn time_based_spectral(filename: &str) {
    let mut reader =
        WavReader::open(filename).expect("Failed to open WAV file");
    let mut fft_planer = FFTplanner::new(false); // not inverse FFT;
    let fft = fft_planer.plan_fft(CHUNK_SIZE);
    let signal = reader
        .samples::<i16>()
        .map(|x| Complex::new(x.unwrap() as f32, 0f32))
        .collect::<Vec<_>>();
    let time_signal = (0..signal.len())
        .map(|i| (i as f32, signal[i].re as f32))
        .collect::<Vec<_>>();
    show_plot("signal", &time_signal);
    let mut time_spectrum = Vec::new();
    let mut i = 0;
    for signal_chunk in signal.chunks(CHUNK_SIZE) {
        i = i + 1;
        let mut new_signal_chunk = if signal_chunk.len() != CHUNK_SIZE {
            let mut tmp_vec = vec![Complex::new(0.0f32, 0.0); CHUNK_SIZE];
            for (j, d) in signal_chunk.iter().enumerate() {
                tmp_vec[j] = *d;
            }
            tmp_vec
        } else {
            signal_chunk.into()
        };
        let mut spectrum = new_signal_chunk.clone();
        fft.process(&mut new_signal_chunk[..], &mut spectrum[..]);
        let max_peak = spectrum
            .iter()
            .take(CHUNK_SIZE / 2)
            .enumerate()
            .max_by_key(|&(_, freq)| freq.norm() as u32);
        if let Some((f, _)) = max_peak {
            let bin = SAMPLE_RATE / CHUNK_SIZE as f32;
            let freq = f as f32 * bin;
            match frequency_to_key(freq) {
                Some(_) => time_spectrum.push((i as f32, freq)),
                None => continue,
            }
            println!("feq {}, key {:?}", freq, frequency_to_key(freq));
        } else {
            continue;
        }
    }
    show_plot("spectrum", &time_spectrum);
}

const DRIFT_PERCENT: f32 = 0.05;

// Refrence: http://www.contrabass.com/pages/frequency.html
static FREQENCY_MAPS: [(&str, f32); 14] = [
    ("A#3", 233.88),
    ("B3", 246.94),
    ("C4", 261.63),
    ("C#4", 277.18),
    ("D4", 293.66),
    ("D#4", 311.13),
    ("E4", 329.63),
    ("F4", 349.23),
    ("F#4", 369.99),
    ("G4", 392.00),
    ("G#4", 415.30),
    ("A4", 440.00),
    ("A#4", 466.16),
    ("B4", 493.88),
];

fn frequency_to_key(freqency: f32) -> Option<String> {
    for freq_map in &FREQENCY_MAPS {
        if ((freqency - freq_map.1).abs() / freq_map.1) < DRIFT_PERCENT {
            return Some(freq_map.0.to_string());
        }
    }
    None
}

fn main() {
    let file_path = args().nth(1).expect("Please provide a WAV file");
    println!("{}", &file_path);

    time_based_spectral(&file_path);
}
