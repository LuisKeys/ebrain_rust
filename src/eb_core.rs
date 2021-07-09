extern crate ffmpeg_next as ffmpeg;

mod io_util;
mod ui_util;

use ffmpeg::util::frame::video::Video;
use std::thread;

pub fn run() {
  

  thread::spawn(|| {
    io_util::video_util::read_video(&process_frame).unwrap();    
  });
  
  ui_util::win_util::create_window();
}

pub fn process_frame(frame: &Video, frame_index: i32) {
  println!("Process frame {}", frame_index);
  println!("Process frame width: {}, height: {}", frame.width(), frame.height());
  //println!("Process frame {:?}", format!("P6\n{} {}\n255\n", frame.width(), frame.height()).as_bytes());
  //println!("Process frame {:?}", frame.data(0));  
}
