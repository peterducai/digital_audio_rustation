use std::time::{Duration, SystemTime};
use std::thread::sleep;
enum ClockType {
    GLOBAL,
    LOCAL,
    EXTERNAL
}

// enum Content {
//     MIDI,
//     WAV,
//     FLAC,
//     DEVICE
// }

//Track can be MIDI, Audio, Device
struct Clock {
    kind: ClockType,
    start: SystemTime(time::SystemTime::now()), //wrong, should not be in compile time
    address: String,
    buffer: Content
}



fn compute_elapsed() {
   let now = SystemTime::now();

   // we sleep for 2 seconds
   sleep(Duration::new(2, 0));
   match now.elapsed() {
       Ok(elapsed) => {
           // it prints '2'
           println!("{}", elapsed.as_secs());
       }
       Err(e) => {
           // an error occurred!
           println!("Error: {e:?}");
       }
   }
}