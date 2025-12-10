use log::{error, info};
use std::fs::File;
use std::io::Write;

pub fn save_svg(content_as_string: String, output_file_path: &str) {
    let file = File::create(output_file_path);

    let fix_file = match file {
        Err(e) => {
            error!("couldn't create {}: {}", output_file_path, e);
            return;
        }
        Ok(mut file) => file.write_all(content_as_string.as_bytes()),
    };

    match fix_file {
        Err(e) => error!("couldn't write to {}: {}", output_file_path, e),
        Ok(_) => info!("wrote to {}", output_file_path),
    }
}
