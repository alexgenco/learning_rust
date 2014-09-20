use std::os;
use std::io::File;
use std::io::BufferedReader;

fn main() {
  let args = os::args();
  let paths = args.tail();

  if paths.len() > 0 {
    for p in paths.iter() {
      cat_file_at(p);
    }
  } else {
  }
}

fn cat_file_at(pstr: &String) {
  let path = Path::new(pstr.to_string());
  let file = File::open(&path);
  let mut reader = BufferedReader::new(file);

  for line in reader.lines() {
    match line {
      Ok(content) => print!("{}", content),
      Err(err)    => print!("error: {}", err)
    }
  }
}
