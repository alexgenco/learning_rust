use std::os;
use std::io;
use std::io::{File, BufferedReader, IoResult};

fn main() {
  let args = os::args();
  let paths = args.tail();

  if paths.len() > 0 {
    for p in paths.iter() {
      cat_file_at(p);
    }
  } else {
    cat_stdin();
  }
}

fn cat_file_at(pstr: &String) {
  let path = Path::new(pstr.to_string());
  let file = File::open(&path);
  let mut reader = BufferedReader::new(file);

  for line in reader.lines() {
    print_io_result(line);
  }
}

fn cat_stdin() {
  for line in io::stdin().lines() {
    print_io_result(line);
  }
}

fn print_io_result(result: IoResult<String>) {
  match result {
    Ok(content) => print!("{}", content),
    Err(err)    => print!("error: {}", err)
  }
}
