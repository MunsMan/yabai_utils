use std::env::temp_dir;
use std::fs::File;
use std::io::Write;
use std::path::Path;

const LOG_FILENAME: &str = "yabai-utils.log";

fn load_file() -> File {
    let filepath = temp_dir().join(Path::new(LOG_FILENAME));
    File::create(filepath).unwrap()
}

pub fn log(msg: String) {
    let file = load_file();
    writeln!(&file, "{}", msg).unwrap();
}
