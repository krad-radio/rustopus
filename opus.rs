#[link(name = "opus", vers = "0.1")];

extern mod opus {
    fn opus_get_version_string() -> *libc::c_char;
}

fn get_rust_opus_version() -> ~str unsafe {
	return ~"Rust Opus Version 0.1 " + get_opus_version();
}

fn get_opus_version() -> ~str unsafe {
	return str::unsafe::from_c_str(opus::opus_get_version_string());
}


