# wav_f64vec
A library of reading & writing wav files, and interconversion between wav audio data and `Vec<Vec<f64>>`. Only for 64bit OS.

## Introduction
This libary provides the following features:
* Reading & writing wav files.
* Interconversion between wav audio data and a tupple that has a wave format structure and a audio data vector(`Vec<Vec<f64>>`). The order of the audio data vector's (`Vec<Vec<f64>>`) dimensions can be specified by corresponding APIs in each.

## Wav File Format

* Format:
    * Unsigned 8bit PCM
    * Signed 16,24,32bit PCM
    * 32bit IEEE Float

* Channel:
    * Mono or Stereo

* Sampling Rate:
    * 8000Hz, 16000Hz, 32000Hz, 48000Hz, 96000Hz, 192000Hz, 22050Hz or 44100Hz

## Documents
```
cargo doc --open
```

## Usage Examples
### New & Save File
```Rust
extern crate wav_f64vec;
use wav_f64vec::*;

fn main() {
    // format id = 1(PCM), 2ch, 16000Hz, 16bit
    let wave_format = WaveFormat {
        id: 1,
        channel: 2,
        sampling_rate: 16000,
        bits: 16,
    };

    // make 2-dimensional audio_data (5sec, 440hz, -3dB, sin_wave)
    let mut channel_data_vec: Vec<Vec<f64>> = Vec::new();
    for _ in 0..wave_format.channel {
        let mut data_vec: Vec<f64> = Vec::new();
        // 5sec
        for idx in 0..wave_format.sampling_rate * 5 {
            let sec = idx as f64 / wave_format.sampling_rate as f64;
            // 440Hz -3Db
            let data = (2.0 * std::f64::consts::PI * 440.0 * sec).sin() * 0.5012;
            data_vec.push(data);
        }
        channel_data_vec.push(data_vec);
    }

    let mut wav_file = WavFile::new();
    wav_file
        .update_audio_for_channel_data_vec(&wave_format, &channel_data_vec)
        .unwrap();
    wav_file.save_as(std::path::Path::new("test.wav")).unwrap();
}

```
### Open & Save File
```Rust
extern crate wav_f64vec;
use wav_f64vec::*;

fn main() {
    // open file and make a wav file struct
    let mut wav_file = WavFile::open(std::path::Path::new(r"./test.wav")).unwrap();

    // make a WavFormat struct and aidio data vector from the wave file sturct
    let (wave_format, channel_data_vec) = wav_file.get_audio_for_channel_data_vec().unwrap();

    // any process (example: print format & convert sampling rate of wave)
    println!(
        "[opened file] format id: {}, channel: {}, sampling rate: {}, bits: {}",
        wave_format.id, wave_format.channel, wave_format.sampling_rate, wave_format.bits
    );
    println!("convert sampling rate to 44100Hz.",);
    let mut new_format = wave_format.clone();
    new_format.sampling_rate = 44100;
    let new_channel_data_vec = convert_sampling_rate_for_channel_data_vec(
        &channel_data_vec,
        wave_format.sampling_rate,
        new_format.sampling_rate,
    )
    .unwrap();
    println!(
        "[new file] format id: {}, channel: {}, sampling rate: {}, bits: {}",
        new_format.id, new_format.channel, new_format.sampling_rate, new_format.bits
    );

    // reflect changes to wav_file
    wav_file
        .update_audio_for_channel_data_vec(&new_format, &new_channel_data_vec)
        .unwrap();

    // save a wav file
    wav_file
        .save_as(std::path::Path::new("./test1.wav"))
        .unwrap();
}

```