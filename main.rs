use chrono::{DateTime, Local};
use std::{thread, time, io};
//use std::time::Duration;
use std::io::BufReader;
use std::fs::File;
use rodio::{Decoder, OutputStream, Sink};
use rodio::source::{Source};

fn main() {
    let mut wakey = String::new();
    println!("Enter the time you would like to wake up.");
    println!("Please use 24hrformat (ex. 06:15):");
    io::stdin()
        .read_line(&mut wakey)
        .expect("Failed to read line");

    'outer: loop {
        let now: DateTime<Local> = Local::now();
        let one_secs = time::Duration::from_millis(1000);
        let current_time: String = now.format("%R").to_string();
        std::process::Command::new("clear").status().unwrap();
        println!("Current Time: {}", current_time);
        println!("Alarm set for: {}", wakey);

        if wakey.trim().eq(&current_time) {
            break 'outer;
        } else {
            thread::sleep(one_secs);
        }
    }
    println!{"Wakey wakey hands off snakey!!!"}
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

// Add a dummy source of the sake of the example.
    //let source = SineWave::new(440.0).take_duration(Duration::from_secs_f32(5.0)).amplify(0.20);
    //sink.append(source);
    let file = BufReader::new(File::open("./audio.mp3").unwrap());
    let source = Decoder::new(file).unwrap().repeat_infinite();
    sink.append(source);
// The sound plays in a separate thread. This call will block the current thread until the sink
// has finished playing all its queued sounds.
    sink.sleep_until_end();
}

