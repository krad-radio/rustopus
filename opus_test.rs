use std;
use opus;


fn main() {
  io::println(opus::get_rust_opus_version());
  
  opus::run_rust_opus_test();
  
}
