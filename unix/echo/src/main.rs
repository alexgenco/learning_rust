use std::os;

fn main() {
  let argv = os::args();
  let argc = argv.iter().count();
  let echos = argv.slice(1, argc);

  for (i, arg) in echos.iter().enumerate() {
    if i == argc - 2 {
      print!("{}\n", arg);
    } else {
      print!("{} ", arg);
    }
  }
}
