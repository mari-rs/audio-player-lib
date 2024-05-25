

use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use anyhow::Result;

pub struct AudioPlayer {
    sink: Arc<Mutex<Option<Sink>>>,
    _stream: OutputStream,
    _stream_handle: OutputStreamHandle,
    repeat: Arc<Mutex<bool>>,
    stop_flag: Arc<Mutex<bool>>
}


impl AudioPlayer {
    pub fn new() -> Result<Self> {
        let (_stream, stream_handle) = OutputStream::try_default()?;
        Ok(AudioPlayer {
            sink: Arc::new(Mutex::new(None)),
            _stream,
            _stream_handle: stream_handle,
            repeat: Arc::new(Mutex::new(false)),
            stop_flag: Arc::new(Mutex::new(false))
        })
    }

    pub fn play(&self, file_path: &str) -> Result<()> {
        let sink = Sink::try_new(&self._stream_handle)?;
        
        let file = BufReader::new(File::open(file_path)?);
        let source = Decoder::new(file)?;
        sink.append(source);

        let mut current_sink = self.sink.lock().unwrap();
        *current_sink = Some(sink);

        let sink_arc = self.sink.clone();
        let repeat_arc = self.repeat.clone();
        let file_path = file_path.to_string();
        let stop_flag_arc = self.stop_flag.clone();

        thread::spawn(move || {
            loop {

                if *stop_flag_arc.lock().unwrap() {
                    break;
                }

                let is_empty = {
                    let sink = sink_arc.lock().unwrap();
                    if let Some(ref s) = *sink {
                        s.empty()
                    } else {
                        false
                    }
                };

                if is_empty {
                    

                    let repeat = *repeat_arc.lock().unwrap();
                    if !repeat {
                        break;
                    }

                    let file = BufReader::new(File::open(&file_path).unwrap());
                    let source = Decoder::new(file).unwrap();

                    let sink = sink_arc.lock().unwrap();
                    if let Some(ref s) = *sink {
                        s.append(source);
                        println!("playing another track");
                    }
                }
                
                thread::sleep(Duration::from_millis(100));
               
            }

        });

        Ok(())
    }

    pub fn play_repeat(&self, enabled: bool) {
        let mut repeat = self.repeat.lock().unwrap();
        *repeat = enabled;
       
    }

    pub fn pause(&self) {
        if let Some(sink) = self.sink.lock().unwrap().as_ref() {
            sink.pause();
        }
    }

    pub fn resume(&self) {
        if let Some(sink) = self.sink.lock().unwrap().as_ref() {
            sink.play();
        }
    }

    pub fn stop(&self) {

        let mut stop_flag = self.stop_flag.lock().unwrap();
        *stop_flag = true;

        if let Some(sink) = self.sink.lock().unwrap().take() {
            sink.stop();
        }
    }
}