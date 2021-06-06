use rodio::source::{SineWave, Source};
use rodio::{OutputStream, Sink};
use std::time::Duration;

pub fn beep() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let source = SineWave::new(440)
        .take_duration(Duration::from_secs_f32(0.15))
        .amplify(0.10);
    sink.append(source);

    sink.sleep_until_end();
}
