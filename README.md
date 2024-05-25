# audio-player-lib
a simple audio player lib for rust.

## features
- playing audio
- pausing/resuming audio
- stoping audio
- repeating tracks

## How to use this library

### how to add the library to your cargo

(expecting your directory tree looks like this or similar)
```
├───libtester
│   └───src
├───audio-player-lib
    ├───src
        
```
add this to your Cargo.toml 
```toml
[dependencies]
audio_player_lib = { path = "../audio-player-lib" }
```

### prerequisites 
- [rust language](https://www.rust-lang.org/tools/install)
- a few mp3 or any other supported audio format by rodio to test this library

### example 
```rs
use tokio::signal;
use audio_player_lib::AudioPlayer;

#[tokio::main]
async fn main() {
    let audio_file = dirs::home_dir().unwrap().join("spotify-dl-data").join("cy4ne - keep yourself safe.flac");

    let audio_file = dirs::home_dir().unwrap().join("spotify-dl-data").join("cy4ne - keep yourself safe.flac");

    let engine = AudioPlayer::new();
 
    engine.play(audio_file.as_os_str().to_str().unwrap());


    tokio::time::sleep(Duration::from_secs(4)).await;

    engine.pause();

    tokio::time::sleep(Duration::from_secs(4)).await;

    engine.resume();

    engine.play_repeat(true);

    tokio::time::sleep(Duration::from_secs(110)).await;

    engine.pause();

    tokio::time::sleep(Duration::from_secs(6)).await;

    engine.play_repeat(false);

    engine.resume();

    tokio::time::sleep(Duration::from_secs(5)).await;

    engine.stop();    
   
    signal::ctrl_c().await.expect("failed to listen for Ctrl+C"); //keep the thread alive
}
```
