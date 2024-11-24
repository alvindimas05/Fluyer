#[cfg(test)]
mod tests {
    use std::{fs::File, io::BufReader, thread::sleep, time::Duration};
    use flacenc::component::{parser, Decode};
    use rodio::{buffer::SamplesBuffer, Sink};

    #[test]
    fn play_music(){
        let (_stream, stream_handle) =
            rodio::OutputStream::try_default().expect("Failed to create stream");
    
        let sink = Sink::try_new(&stream_handle).expect("Failed to create sink");
        sink.pause();
        
        let bytes = std::fs::read("/Users/alvindimas05/Music/The Meaning of Life/Gryffin - Gravity (2019)/01-02 All You Need to Know.flac")
            .expect("Failed to read FLAC file");
        let (_remaining_input, stream) = parser::stream::<()>(&bytes).expect("Failed to parse FLAC file");
        
        let frame_count = stream.frame_count();
        
        for n in 0..frame_count {
            let frame = stream.frame(n).unwrap();
            let frame_signal = frame.decode();
            
            let samples: Vec<f32> = frame_signal
                .iter()
                .map(|&sample| sample as f32 / i32::MAX as f32)
                .collect();

            sink.append(SamplesBuffer::new(1, 44100, samples));
        }
        sink.play();
        
        sleep(Duration::from_secs(60));
    }
}