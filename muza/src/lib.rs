#![allow(unused)]
use std::io::stdin;

use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    BufferSize, SampleRate, StreamConfig,
};

pub mod aliases;
pub mod audio;
pub mod midi;
pub mod midicon;
pub mod synths;
pub mod wave;
pub mod waveforms;

pub fn run() {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("no output device available");

    let stream = device
        .build_output_stream(
            &StreamConfig {
                channels: 2,
                sample_rate: SampleRate(44_100),
                buffer_size: BufferSize::Fixed(1024),
            },
            |samples: &mut [f32], _: &cpal::OutputCallbackInfo| {
                // react to stream events and read or write stream data here.
                println!("{}", samples.len());
            },
            |err| {
                // react to errors here.
                print!("{}", err)
            },
            None, // None=blocking, Some(Duration)=timeout
        )
        .expect("could not build stream");
    stream.play().unwrap();
    println!("playing");
    stdin().read_line(&mut String::new()).unwrap();
    println!("exit");
}
