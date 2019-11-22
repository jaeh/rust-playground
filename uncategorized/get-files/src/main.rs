use std::{
    env,
};

const ALL_THE_FILES: &[(&str, &[u8])] = &include!(concat!(env!("OUT_DIR"), "/all_the_files.rs"));

fn main() {
	println!("ohai");
}