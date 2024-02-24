use std::io;

pub fn read_non_empty_line(buf: &mut String) {
  loop {
      io::stdin().read_line(buf).expect("Failed to read line");

      *buf = buf.trim().to_string();

      if buf.trim().is_empty() {
          println!("Can't enter an empty line. Please try again");
      } else {
          break;
      }
  }
}