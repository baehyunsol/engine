use tera::Context;
use mdxt::{render_to_html, RenderOption, RenderResult};
use yaml_rust::Yaml;
use crate::file_io::*;
use std::collections::HashMap;

enum EngineType {
    Tera, Sass, MDxt
}

struct Log {
    file_from: String,
    file_to: String,
    metadata: Yaml
}

fn render_directory(
    dir_from: &str, ext_from: &str,
    engine: EngineType,
    dir_to: &str, ext_to: &str,
    tera_context: &Option<Context>,
    mdxt_option: &Option<RenderOption>
) -> Result<Vec<Log>, ()> {

    let mut files = read_dir(dir_from)?;
    files = files.into_iter().filter(
        |f| match extension(f) {
            Ok(ext) if ext == ext_from => true,
            _ => false
        }
    ).collect();

    let mut logs = Vec::with_capacity(files.len());

    match engine {

        EngineType::Tera => {
            let context = match tera_context {
                Some(c) => c.clone()
                None => {return Err(());}
            };

            for file in files.iter() {}

        }

        EngineType::Sass => {}

        EngineType::MDxt => {
            let options = match mdxt_option {
                Some(option) => option.clone(),
                None => RenderOption::default()
            };

            for file in files.iter() {
                let file_name = file_name(file)?;
                let dest = set_ext(&join(dir_to, &file_name)?, ext_to)?;

                match read_string(file) {
                    Ok(mdxt) => {
                        let rendered = render_to_html(&mdxt, options.clone());
                        write_to_file(&dest, rendered.content.as_bytes())?;
                    },
                    _ => { return Err(()); }
                }

            }

        }

    }

    Ok(logs)
}