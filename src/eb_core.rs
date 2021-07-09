mod io_util;
mod ui_util;

extern crate ffmpeg_next as ffmpeg;
use ffmpeg::util::frame::video::Video;

pub fn run() {
  //ui_util::win_util::create_window();
  io_util::video_util::read_video(&process_frame).unwrap();  
}

pub fn process_frame(frame: &Video, frame_index: i32) {
  println!("Process frame {}", frame_index);
  println!("Process frame width: {}, height: {}", frame.width(), frame.height());
  //println!("Process frame {:?}", format!("P6\n{} {}\n255\n", frame.width(), frame.height()).as_bytes());
  //println!("Process frame {:?}", frame.data(0));  
}
