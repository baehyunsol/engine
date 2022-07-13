use tera::{Context, Tera};
use mdxt::{render_to_html, RenderOption, RenderResult};
use yaml_rust::Yaml;
use crate::file_io::*;
use crate::error::Error;
use crate::log::Log;
use crate::yaml_hash;

#[derive(Debug)]
pub enum EngineType {
    Tera, Scss, MDxt
}

pub fn render_directory(
    dir_from: &str, ext_from: &str,
    engine: EngineType,
    dir_to: &str, ext_to: &str,
    tera_context: &Option<Context>,
    mdxt_option: &Option<RenderOption>
) -> Result<Vec<Log>, Error> {

    let mut files = match read_dir(dir_from) {
        Ok(f) => f,
        Err(_) => {
            return Err(Error::PathError(format!("error at `render_directory({:?}, {:?}, ...)`\n`read_dir({:?})` failed", dir_from, ext_from, dir_from)));
        }
    };

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
                Some(c) => c.clone(),
                None => Context::new()
            };

            let mut tera = Tera::default();

            for file in files.iter() {
                let dest = get_dest_path(&file, dir_to, ext_to)?;

                match tera.add_template_file(file, None) {
                    Err(e) => {
                        return Err(Error::PathError(format!(
                            "error at `render_directory({:?}, {:?}, ...)`\n`tera.add_template_file({:?})` failed\nerror message: {:?}",
                            dir_from, ext_from, file, e
                        )));
                    }
                    _ => {}
                }

                match tera.render(file, &context) {
                    Ok(result) => {
                        match write_to_file(&dest, result.as_bytes()) {
                            Err(_) => {return Err(Error::PathError(format!("error at `render_directory({:?}, {:?}, ...)`\n`write_to_file({:?})` failed", dir_from, ext_from, dest)));}
                            _ => { logs.push(Log::new(file, &dest, None)); }
                        }
                    },
                    Err(e) => {
                        return Err(Error::RenderError(
                            EngineType::Tera,
                            format!("tera render error: failed to render {:?}, with error: {:?}", file, e)
                        ));
                    }
                }

            }

        }

        EngineType::Scss => {
            let sass_option = grass::Options::default();

            for file in files.iter() {
                let dest = get_dest_path(&file, dir_to, ext_to)?;

                match grass::from_path(file, &sass_option) {
                    Ok(result) => {
                        match write_to_file(&dest, result.as_bytes()) {
                            Err(_) => {return Err(Error::PathError(format!("error at `render_directory({:?}, {:?}, ...)`\n`write_to_file({:?})` failed", dir_from, ext_from, dest)));}
                            _ => { logs.push(Log::new(file, &dest, None)); }
                        }
                    },
                    Err(e) => {
                        return Err(Error::RenderError(
                            EngineType::Scss,
                            format!("sass render error: failed to render {:?}, with error: {:?}", file, e)
                        ));
                    }
                }

            }

        }

        EngineType::MDxt => {
            let options = match mdxt_option {
                Some(option) => option.clone(),
                None => RenderOption::default()
            };

            for file in files.iter() {
                let dest = get_dest_path(&file, dir_to, ext_to)?;

                match read_string(file) {
                    Ok(mdxt) => {
                        let RenderResult {
                            content,
                            has_math,
                            has_collapsible_table,
                            metadata
                        } = render_to_html(&mdxt, options.clone());

                        let mut metadata = match metadata {
                            None => yaml_hash::new(),
                            Some(m) => yaml_hash::from_yaml(m)
                        };

                        metadata = yaml_hash::insert(metadata, Yaml::from_str("has_math"), Yaml::Boolean(has_math));
                        metadata = yaml_hash::insert(metadata, Yaml::from_str("has_collapsible_table"), Yaml::Boolean(has_collapsible_table));

                        match write_to_file(&dest, content.as_bytes()) {
                            Err(_) => {return Err(Error::PathError(format!("error at `render_directory({:?}, {:?}, ...)`\n`write_to_file({:?})` failed", dir_from, ext_from, dest)));}
                            _ => { logs.push(Log::new(file, &dest, Some(metadata))); }
                        }
                    },
                    _ => {
                        return Err(Error::PathError(format!("error at `render_directory({:?}, {:?}, ...)`\n`read_string({:?})` failed", dir_from, ext_from, file)));
                    }
                }

            }

        }

    }

    Ok(logs)
}

fn get_dest_path(curr_path: &str, dir_to: &str, ext_to: &str) -> Result<String, Error> {
    let file_name = match file_name(curr_path) {
        Ok(f) => f,
        Err(_) => {
            return Err(Error::PathError(format!("error at `get_dest_path({:?}, {:?}, {:?})`\n`file_name({:?})` failed", curr_path, dir_to, ext_to, curr_path)))
        }
    };

    let joined = match join(dir_to, &file_name) {
        Ok(j) => j,
        Err(_) => {
            return Err(Error::PathError(format!("error at `get_dest_path({:?}, {:?}, {:?})`\n`join({:?}, {:?})` failed", curr_path, dir_to, ext_to, dir_to, file_name)));
        }
    };

    match set_ext(&joined, ext_to) {
        Ok(s) => Ok(s),
        Err(_) => Err(Error::PathError(format!("error at `get_dest_path({:?}, {:?}, {:?})`\n`set_ext({:?}, {:?})` failed", curr_path, dir_to, ext_to, joined, ext_to)))
    }
}