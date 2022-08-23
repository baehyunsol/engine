use tera::{Context, Tera};
use mdxt::{render_to_html, RenderOption, RenderResult};
use yaml_rust::Yaml;
use crate::file_io::*;
use crate::error::Error;
use crate::log::Log;
use crate::yaml_hash;
use crate::MONTHS;

#[derive(Copy, Clone, Debug)]
pub enum EngineType {
    Tera, Scss, MDxt, XML
}

/*
Read all the files in the given directory with the given extension.
Convert the files with the given engine, then save the converted result to the given directory with the given extension.
*/
pub fn render_directory(
    dir_from: &str, ext_from: &str,
    engine: EngineType,
    dir_to: &str, ext_to: &str,
    tera_context: &Option<Context>,
    mdxt_option: &Option<RenderOption>,
    recursive: bool
) -> Result<Vec<Log>, Error> {

    let mut files = match read_dir(dir_from) {
        Ok(f) => f,
        Err(_) => {
            return Err(Error::PathError(format!("error at `render_directory({:?}, {:?}, ...)`\n`read_dir({:?})` failed", dir_from, ext_from, dir_from)));
        }
    };

    let mut logs = vec![];

    if recursive {
        let sub_dirs = files.iter().filter(|f| is_dir(f)).filter_map(
            |f| basename(f).ok()
        ).collect::<Vec<String>>();

        let mut recursive_logs = Vec::with_capacity(sub_dirs.len());

        for sub_dir in sub_dirs.iter() {
            let new_dir_from = match join(dir_from, sub_dir) {
                Ok(d) => d,
                _ => {
                    return Err(Error::PathError(format!("error at `render_directory({:?}, {:?}, ...)`\n`join({:?}, {:?})` failed", dir_from, ext_from, dir_from, sub_dir)))
                }
            };

            let new_dir_to = match join(dir_to, sub_dir) {
                Ok(d) => d,
                _ => {
                    return Err(Error::PathError(format!("error at `render_directory({:?}, {:?}, ...)`\n`join({:?}, {:?})` failed", dir_from, ext_from, dir_to, sub_dir)))
                }
            };

            match render_directory(
                &new_dir_from, ext_from,
                engine,
                &new_dir_to, ext_to,
                tera_context, mdxt_option,
                recursive
            ) {
                Ok(logs) => { recursive_logs.push(logs); }
                Err(e) => { return Err(e); }
            }
        }

        logs = recursive_logs.concat();
    }

    files = files.into_iter().filter(
        |f| match extension(f) {
            Ok(ext) if ext == ext_from => true,
            _ => false
        }
    ).collect();

    mkdir(dir_to);    // don't unwrap this. If the path already exists, it'd raise an error, but that's fine.

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
                            Err(_) => { return Err(Error::PathError(format!("error at `render_directory({:?}, {:?}, ...)`\n`write_to_file({:?})` failed", dir_from, ext_from, dest))); }
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
            let scss_option = grass::Options::default();

            for file in files.iter() {
                let dest = get_dest_path(&file, dir_to, ext_to)?;

                match grass::from_path(file, &scss_option) {
                    Ok(result) => {
                        match write_to_file(&dest, result.as_bytes()) {
                            Err(_) => {return Err(Error::PathError(format!("error at `render_directory({:?}, {:?}, ...)`\n`write_to_file({:?})` failed", dir_from, ext_from, dest)));}
                            _ => { logs.push(Log::new(file, &dest, None)); }
                        }
                    },
                    Err(e) => {
                        return Err(Error::RenderError(
                            EngineType::Scss,
                            format!("scss render error: failed to render {:?}, with error: {:?}", file, e)
                        ));
                    }
                }

            }

        }

        EngineType::MDxt => {
            let mut options = match mdxt_option {
                Some(option) => option.clone(),
                None => RenderOption::default()
            };
            options.xml = true;

            let mut article_info = Tera::default();

            match article_info.add_template_file("./templates/pages/article_info.tera", Some("article_info")) {
                Err(e) => {
                    return Err(Error::PathError(format!(
                        "error at `render_directory({:?}, {:?}, ...)`\n`tera.add_template_file(./templates/pages/article_info.tera, ..)` failed\nerror message: {:?}",
                        dir_from, ext_from, e
                    )));
                },
                _ => {}
            }

            for file in files.iter() {
                let dest = get_dest_path(&file, dir_to, ext_to)?;

                match read_string(file) {
                    Ok(mdxt) => {
                        let RenderResult {
                            mut content,
                            has_collapsible_table,
                            metadata,
                            fenced_code_contents: _
                        } = render_to_html(&mdxt, options.clone());

                        let mut metadata = match metadata {
                            None => yaml_hash::new(),
                            Some(m) => yaml_hash::from_yaml(m)
                        };

                        metadata = yaml_hash::insert(metadata, Yaml::from_str("has_collapsible_table"), Yaml::Boolean(has_collapsible_table));

                        // it renders article_info if the metadata has `date` or `tags`.
                        match render_article_info(&metadata, &article_info) {
                            Some(info) => {
                                content = vec![render_to_html(&info, options.clone()).content, content].concat();
                            }
                            _ => {}
                        }

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

        EngineType::XML => {
            let image_box = match read_string("./templates/xml/img_box.html") {
                Ok(s) => s,
                _ => {
                    return Err(Error::PathError(format!("error at `render_directory({:?}, {:?}, ...)`\n`read_string('./templates/xml/img_box.html')` failed", dir_from, ext_from)));
                }
            };

            let image_script = match read_string("./templates/xml/img_script.html") {
                Ok(s) => s,
                _ => {
                    return Err(Error::PathError(format!("error at `render_directory({:?}, {:?}, ...)`\n`read_string('./templates/xml/img_script.html')` failed", dir_from, ext_from)));
                }
            };

            for file in files.iter() {
                let dest = get_dest_path(&file, dir_to, ext_to)?;

                match read_string(file) {
                    Ok(html) => match hxml::into_dom(html) {
                        Ok(_) => {
                            let mut images = hxml::dom::get_elements_by_tag_name(None, "img".to_string());

                            if images.len() > 0 {

                                for img in images.iter_mut() {
                    
                                    match img.get_attribute("src".to_string()) {
                                        Some(src) => {
                                            img.set_attribute("onclick".to_string(), format!("open_modal_img('{}');", src));
                                        },
                                        _ => {}
                                    }
                    
                                }

                            }

                            let mut head = &mut hxml::dom::get_elements_by_tag_name(None, "head".to_string())[0];
                            head.add_element_ptr(hxml::Element::from_string("<link href=\"image.css\" rel=\"stylesheet\"/>".to_string()).unwrap());

                            let mut body = &mut hxml::dom::get_elements_by_tag_name(None, "body".to_string())[0];
                            let modal_box = hxml::Element::from_string(image_box.clone()).unwrap();
                            let script = hxml::Element::from_string(image_script.clone()).unwrap();

                            body.add_element_ptr(modal_box);
                            body.add_element_ptr(script);

                            let result = hxml::dom::get_root().to_string();

                            // HXML should handle this..!!
                            let result = format!("<!DOCTYPE html>{}", result);

                            match write_to_file(&dest, result.as_bytes()) {
                                Ok(_) => {
                                    logs.push(Log::new(file, &dest, None));
                                },
                                Err(error) => {
                                    return Err(Error::PathError(format!("error at `render_directory({:?}, {:?}, ...)`\n`write_to_file({:?})` failed", dir_from, ext_from, dest)));
                                }
                            }

                        },
                        Err(errors) => {
                            return Err(Error::RenderError(EngineType::XML, format!("{:?} is not a valid xml!\nerrors: {:?}", file, errors)));
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

/*
Read all the files in `article path` with the given extension.
Render the given template with the given context and the articles.
The articles are inserted to the context with `article` keyword.
The rendered results are written to `output_path`, with `output_ext`.
*/
pub fn render_templates(
    template_path: &str,  // file
    article_path: &str,   // path
    article_ext: &str,
    output_path: &str,
    output_ext: &str,
    mut tera_instance: Option<Tera>,
    mut context: Option<Context>,
    recursive: bool
) -> Result<Vec<Log>, Error> {

    if tera_instance.is_none() {
        let mut tera = Tera::default();

        match tera.add_template_file(template_path, None) {
            Err(e) => {
                return Err(Error::PathError(format!(
                    "error at `render_templates({:?}, {:?}, ...)`\n`tera.add_template_file({:?})` failed\nerror message: {:?}",
                    template_path, article_path, template_path, e
                )));
            },
            _ => {}
        }

        tera_instance = Some(tera);
    }

    if context.is_none() {
        let context_ = Context::new();
        context = Some(context_);
    }

    let mut articles = match read_dir(article_path) {
        Ok(f) => f,
        Err(_) => {
            return Err(Error::PathError(format!("error at `render_templates({:?}, {:?}, ...)`\n`read_dir({:?})` failed", template_path, article_path, article_path)));
        }
    };

    let mut logs = vec![];

    if recursive {
        let sub_dirs = articles.iter().filter(|f| is_dir(f)).filter_map(
            |f| basename(f).ok()
        ).collect::<Vec<String>>();

        let mut recursive_logs = Vec::with_capacity(sub_dirs.len());

        for sub_dir in sub_dirs.iter() {
            let new_article_path = match join(article_path, sub_dir) {
                Ok(d) => d,
                _ => {
                    return Err(Error::PathError(format!("error at `render_templates({:?}, {:?}, ...)`\n`join({:?}, {:?})` failed", template_path, article_path, article_path, sub_dir)))
                }
            };

            let new_output_path = match join(output_path, sub_dir) {
                Ok(d) => d,
                _ => {
                    return Err(Error::PathError(format!("error at `render_templates({:?}, {:?}, ...)`\n`join({:?}, {:?})` failed", template_path, output_path, output_path, sub_dir)))
                }
            };

            match render_templates(
                template_path,
                &new_article_path,
                article_ext,
                &new_output_path,
                output_ext,
                tera_instance.clone(),
                context.clone(),
                recursive
            ) {
                Ok(logs) => { recursive_logs.push(logs); }
                Err(e) => { return Err(e); }
            }
        }

        logs = recursive_logs.concat();
    }

    articles = articles.into_iter().filter(
        |f| match extension(f) {
            Ok(ext) if ext == article_ext => true,
            _ => false
        }
    ).collect();
    mkdir(output_path);    // don't unwrap this. If the path already exists, it'd raise an error, but that's fine.

    let mut context = context.unwrap();
    let tera_instance = tera_instance.unwrap();

    let has_title = context.get("title").is_some();

    for article in articles.iter() {
        let dest = get_dest_path(&article, output_path, output_ext)?;
        let article_data = match read_string(article) {
            Ok(s) => s,
            _ => {
                return Err(Error::PathError(format!("error at `render_templates({:?}, {:?}, ...)`\n`read_string({:?})` failed", template_path, article_path, article)));
            }
        };

        if !has_title {
            let mut title = match file_name(article) {
                Ok(t) => t,
                _ => {
                    return Err(Error::PathError(format!("error at `render_templates({:?}, {:?}, ...)`\n`file_name({:?})` failed", template_path, article_path, article)));
                }
            };

            if title == "index".to_string() {
                title = "Blog".to_string();
            }

            context.insert("title", &title);
        }

        context.insert("article", &article_data);

        match tera_instance.render(template_path, &context) {
            Ok(result) => {
                match write_to_file(&dest, result.as_bytes()) {
                    Err(_) => {return Err(Error::PathError(format!("error at `render_templates({:?}, {:?}, ...)`\n`write_to_file({:?})` failed", template_path, article_path, dest)));}
                    _ => { logs.push(Log::new(article, &dest, None)); }
                }
            },
            Err(e) => {
                return Err(Error::RenderError(
                    EngineType::Tera,
                    format!("tera render error: failed to render {:?}, with error: {:?}", article, e)
                ));
            }
        }
    }

    Ok(logs)
}

pub fn copy_all(
    dir_from: &str,
    ext_from: &str,
    dir_to: &str,
    ext_to: &str,
    recursive: bool
) -> Result<Vec<Log>, Error> {

    let mut files = match read_dir(dir_from) {
        Ok(f) => f,
        Err(_) => {
            return Err(Error::PathError(format!("error at `copy_all({:?}, {:?}, ...)`\n`read_dir({:?})` failed", dir_from, ext_from, dir_from)));
        }
    };

    let mut logs = vec![];

    if recursive {
        let sub_dirs = files.iter().filter(|f| is_dir(f)).filter_map(
            |f| basename(f).ok()
        ).collect::<Vec<String>>();

        let mut recursive_logs = Vec::with_capacity(sub_dirs.len());

        for sub_dir in sub_dirs.iter() {
            let new_dir_from = match join(dir_from, sub_dir) {
                Ok(d) => d,
                _ => {
                    return Err(Error::PathError(format!("error at `copy_all({:?}, {:?}, ...)`\n`join({:?}, {:?})` failed", dir_from, ext_from, dir_from, sub_dir)))
                }
            };

            let new_dir_to = match join(dir_to, sub_dir) {
                Ok(d) => d,
                _ => {
                    return Err(Error::PathError(format!("error at `copy_all({:?}, {:?}, ...)`\n`join({:?}, {:?})` failed", dir_from, ext_from, dir_to, sub_dir)))
                }
            };

            match copy_all(
                &new_dir_from, ext_from,
                &new_dir_to, ext_to,
                recursive
            ) {
                Ok(logs) => { recursive_logs.push(logs); }
                Err(e) => { return Err(e); }
            }
        }

        logs = recursive_logs.concat();
    }

    files = files.into_iter().filter(
        |f| match extension(f) {
            Ok(ext) if ext == ext_from => true,
            _ => false
        }
    ).collect();

    mkdir(dir_to);

    for file in files.iter() {
        let dest = get_dest_path(&file, dir_to, ext_to)?;

        match read_bytes(file) {
            Ok(data) => {
                match write_to_file(&dest, &data) {
                    Err(_) => {
                        return Err(Error::PathError(format!("error at `copy_all({:?}, {:?}, ...)`\n`write_to_file({:?}, ...)` failed", dir_from, ext_from, dest)));
                    }
                    _ => {
                        logs.push(Log::new(file, &dest, None));
                    }
                }
            },
            _ => {
                return Err(Error::PathError(format!("error at `copy_all({:?}, {:?}, ...)`\n`read_bytes({:?})` failed", dir_from, ext_from, file)));
            }
        }

    }

    Ok(logs)
}

fn render_article_info(metadata: &Yaml, tera: &Tera) -> Option<String> {

    let mut tera_context = Context::new();
    let mut has_nothing = true;

    match yaml_hash::get(metadata, &Yaml::from_str("date")) {
        None => {}
        Some(d) => {

            match d.as_vec() {
                Some(date) if date.len() == 3 => {
                    let year = date[0].clone().into_i64();
                    let month = date[1].clone().into_i64();
                    let day = date[2].clone().into_i64();

                    if year.is_some() && month.is_some() && day.is_some() {
                        let year = year.unwrap();
                        let month = month.unwrap();
                        let day = day.unwrap();

                        if day > 0 && month > 0 && day < 32 && month < 13 {
                            tera_context.insert("date", &format!("{}.{}.{}", day, MONTHS[month as usize], year));
                            has_nothing = false;
                        }

                    }

                }
                _ => {},
            }

        }
    };

    match yaml_hash::get(metadata, &Yaml::from_str("tags")) {
        None => {}
        Some(t) => {
            let tags = t.clone();

            match tags.as_vec() {
                None => {}
                Some(t) => {
                    tera_context.insert("tags", &t.iter().filter_map(
                        |tag| match tag.clone().into_string() {
                            Some(tag) => Some(format!("[#{}](tag-{}.html)", tag, tag_page(&tag))),
                            None => None
                        }
                    ).collect::<Vec<String>>());
                    has_nothing = false;
                }
            }
        }
    };

    if has_nothing {
        None
    }

    else {
        tera.render("article_info", &tera_context).ok()
    }

}

fn tag_page(tag_name: &str) -> String {
    tag_name.to_string()  // not implemented yet
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