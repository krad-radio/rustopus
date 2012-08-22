#[link(name = "opus", vers = "0.1")];

use std;

extern mod opus {

    fn opus_get_version_string() -> *libc::c_char;
    fn opus_strerror(error: libc::c_int) -> *libc::c_char;

    fn opus_encoder_create(rate: libc::c_int,
                           channels: libc::c_int,
                           application: libc::c_int,
                           error: *libc::c_int) -> libc::intptr_t;
    fn opus_encoder_destroy(encoder: libc::intptr_t);
}

fn run_rust_opus_test() {

  let mut encoder: libc::intptr_t;
  let mut opuserror: libc::c_int = 0;

  io::println(~"Running Opus Test");

  io::println(~"Creating Encoder");

  encoder = opus::opus_encoder_create(48000 as libc::c_int,
                                      2 as libc::c_int,
                                      2049 as libc::c_int,
                                      ptr::addr_of(opuserror));

  print_opus_error(opuserror as int);

  io::println(~"Destroying Encoder");

  opus::opus_encoder_destroy(encoder);
  
}

fn get_rust_opus_version() -> ~str unsafe {
	return ~"Rust Opus Version 0.1 " + get_opus_version();
}



fn print_opus_error(error: int) {
  io::println(~"Opus Error: " + get_opus_error(error));
}

fn get_opus_error(error: int) -> ~str unsafe {
	return str::unsafe::from_c_str(opus::opus_strerror(error as libc::c_int));
}

fn get_opus_version() -> ~str unsafe {
	return str::unsafe::from_c_str(opus::opus_get_version_string());
}


