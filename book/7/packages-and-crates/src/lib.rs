// this file gets compiled to target/{target}/libpackages_and_crates

mod lib_export {
	pub fn testing(string: String) {
		println!("string {}", string);
	}
}

pub fn run_testing(string: String) {
  lib_export::testing(string)
}