use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[path = "src/lookup_build.rs"]
mod lookup_build;

fn write(filename: &str, content: &str) {
	let out_dir = env::var("OUT_DIR").unwrap();
	let dest_path = Path::new(&out_dir).join(filename);
	let mut f = File::create(dest_path).unwrap();
	writeln!(&mut f, "{}", content).unwrap();
}

fn main() {
	write(
		"lookup_table.rs",
		&format!(
			"pub const THREAT_LOOKUP_TABLE: [LookupTableValue; STONES_PERMU_COUNT] = {:?};",
			lookup_build::generate_lookup_table()
		)
	);
}
