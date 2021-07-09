extern crate ffmpeg_next as ffmpeg;
mod io_utils;
use ffmpeg::util::frame::video::Video;

pub fn run() {
  io_utils::video_util::read_video(&process_frame);
}

pub fn process_frame(frame: &Video, frame_index: i32) {
  println!("Process frame {}", frame_index);
  println!("Process frame width: {}, height: {}", frame.width(), frame.height());
  //println!("Process frame {:?}", format!("P6\n{} {}\n255\n", frame.width(), frame.height()).as_bytes());
  //println!("Process frame {:?}", frame.data(0));  
}
