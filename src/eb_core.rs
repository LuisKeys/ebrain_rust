mod io_utils;

pub fn run() {
  println!("Core");
  io_utils::video_util::read_video();
}
