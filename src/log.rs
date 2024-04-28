use std::env::temp_dir;
use std::fs::File;
use std::io::Write;
use std::path::Path;

const LOG_FILENAME: &str = "yabai-utils.log";

fn load_file() -> Result<File, std::io::Error> {
    let filepath = temp_dir().join(Path::new(LOG_FILENAME));
    let file = File::options().create(true).append(true).open(filepath)?;
    Ok(file)
}

pub fn log(msg: String) {
    let file = load_file();
    match file {
        Ok(file) => writeln!(&file, "{}", msg).unwrap(),
        Err(e) => println!("{}", e),
    }
}
