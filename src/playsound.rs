use rodio::{Decoder, OutputStream, source::Source};
use std::fs::File;
use std::io::BufReader;

pub fn playsound(soundtype: &str){ 
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let file = BufReader::new(File::open(soundtype).unwrap());
    // ./assets/check-on.wav
    // ./assets/check-off.wav
    let source = Decoder::new(file).unwrap();
    stream_handle.play_raw(source.convert_samples());
    std::thread::sleep(std::time::Duration::from_secs(3));
}