use same_file::Handle;

use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::path::Path;

use std::fs::File;

use ramhorns::{Content, Template};

#[derive(Content)]
pub struct GenerateInfo {
    pub(crate) version: String,
    pub(crate) name: String,
    pub(crate) vendor: String,
    pub(crate) downloads: String,
}

pub fn generate(generate_info: &GenerateInfo, name: &str) -> String {
    tracing::debug!(
        "Render svg for Plugin with\n name: {} \n version: {} \n vendor: {} \n downloads: {}",
        generate_info.name,
        generate_info.version,
        generate_info.vendor,
        generate_info.downloads
    );

    let mut read: String = String::new();
    match append_read(&mut read, &name) {
        Ok(_) => {
            let tpl = Template::new(read).unwrap();
            tpl.render(&generate_info)
        }
        Err(e) => {
            tracing::error!(error = ?e, "Error Render Svg as String");
            String::from("ERROR")
        }
    }
}

fn append_read(html_file: &mut String, name: &str) -> Result<(), Error> {
    let stdout_handle = Handle::stdout()?;

    let html_open_path_string = format!("rsc/{}.mustache", name);
    let html_open_path = Path::new(&html_open_path_string);

    let html_open_handle = Handle::from_path(html_open_path)?;

    if stdout_handle == html_open_handle {
        return Err(Error::new(
            ErrorKind::Other,
            "You are reading and writing to the same file",
        ));
    } else {
        let file = File::open(&html_open_path)?;
        let file = BufReader::new(file);

        for (_num, line) in file.lines().enumerate() {
            html_file.push_str(&*line?);
            html_file.push_str("\n");
        }
    }
    Ok(())
}
