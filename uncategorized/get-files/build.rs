use std::{
    env, error::Error, fs::{self, File}, io::Write, path::Path,
};

const SOURCE_DIR: &str = "www";

fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = env::var("OUT_DIR")?;
    let dest_path = Path::new(&out_dir).join("all_the_files.rs");
    let mut all_the_files = File::create(&dest_path)?;

    let path = env::current_dir()?;
		let cwd = path.display();

		let source_dir = format!("{}/{}", cwd, SOURCE_DIR);
    writeln!(&mut all_the_files, r#"["#,)?;

    for f in fs::read_dir(SOURCE_DIR)? {
        let f = f?;

        if !f.file_type()?.is_file() {
            continue;
        }

				let path = format!("{}/{}", cwd, f.path().display());

        writeln!(
            &mut all_the_files,
            r#"("{name}", include_bytes!("{path}")),"#,
            name = f.path().display(),
						path = path,
        )?;
    }

    writeln!(&mut all_the_files, r#"];"#,)?;

    Ok(())
}