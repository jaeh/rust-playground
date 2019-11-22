use std::{
    collections::HashMap,
    env,
    error::Error,
    fs::{self, File},
    io::{Write, Read},
};

const SOURCE_DIR: &str = "www";

fn find_files(dir: &str) -> Result<(), Box<dyn Error>> {
    let mut files: HashMap<String, String> = HashMap::new();

    files.insert("testing".to_string(), "hashmap".to_string());

    let key = "testing";

    let content = match files.get(key) {
        Some(content) => content,
        None => panic!("{} does not exist.", key),
    };

    println!("{}", content);

    // for f in fs::read_dir(dir)? {
    //     let f = f?;

    //     if !f.file_type()?.is_file() {
    //         let path = format!("{}", f.path().display());
            
    //         let files = match find_files(SOURCE_DIR) {
    //             Ok(files) => files,
    //             Err(e) => println!("{}", e),
    //         };

    //         continue;
    //     }

    //     let path = format!("{}", f.path().display());

    //     let html_path = "www/index.html";
    //     let path404 = format!("{}/404.html", SOURCE_DIR);

    //     println!("{} {}", path, path404);

    //     let mut file = match File::open(html_path) {
    //         Ok(file) => file,
    //         Err(_) => File::open(path404).expect("404.html file missing!"),
    //     };

    //     let mut contents = String::new();
    //     file.read_to_string(&mut contents).unwrap();
    //     println!("{}", contents);

    //     files.insert(String::from(&path), String::from("stesting"));
    //     println!("file {}", &path);
    // }

    Ok(())
}

fn main() {
    let files = match find_files(SOURCE_DIR) {
        Ok(files) => files,
        Err(e) => println!("{}", e),
    };

    // println!("ohai");
}
