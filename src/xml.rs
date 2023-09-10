use crate::engine::EngineType;
use crate::error::Error;
use crate::file_io::*;
use hxml::Content;

pub fn render_clickable_image(image_box: String, article_title: String) -> Result<(), Error> {
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

        let head = match hxml::dom::get_element_by_tag_name(None, "head".to_string()) {
            Some(head) => head,
            None => {
                return Err(Error::RenderError(EngineType::XML, format!("error at `render_clickable_image()`\n`{}` doesn't have a `<head>` tag!", article_title)));
            }
        };
        head.add_element_ptr(hxml::Element::from_string("<link href=\"image.css\" rel=\"stylesheet\"/>".to_string()).unwrap());

        let body = match hxml::dom::get_element_by_tag_name(None, "body".to_string()){
            Some(body) => body,
            None => {
                return Err(Error::RenderError(EngineType::XML, format!("error at `render_clickable_image()`\n`{}` doesn't have a `<body>` tag!", article_title)));
            }
        };
        let modal_box = hxml::Content::from_string(image_box.clone()).unwrap();

        body.add_contents(modal_box);
    }

    Ok(())
}

pub fn render_lazy_loaded_images(article_title: String) -> Result<(), Error> {
    let mut images = hxml::dom::get_elements_by_tag_name(None, "img".to_string());

    if images.len() > 0 {
        let mut image_srcs = Vec::with_capacity(images.len());

        for (index, image) in images.iter_mut().enumerate() {

            match image.get_attribute("src".to_string()) {
                Some(src) => {

                    // TODO: what if the image already has an id?
                    match image.get_attribute("id".to_string()) {
                        Some(id) => {
                            println!("Warning!! It's overwritting an id of an image: {}", id);
                        },
                        _ => {}
                    }


                    let img_id = format!("lazy-loaded-image-{}", index);
                    image.set_attribute("id".to_string(), img_id.clone());
                    image.set_attribute("src".to_string(), String::new());

                    image_srcs.push((img_id, src));
                },
                _ => {}
            }

        }

        let mut script = image_srcs.into_iter().map(|(id, src)| format!("document.getElementById(\"{}\").src=\"{}\";", id, src)).collect::<Vec<String>>().join("\n");
        script = format!("<script>/*<![CDATA[*/{}/*]]>*/</script>", script);

        let body = match hxml::dom::get_element_by_tag_name(None, "body".to_string()){
            Some(body) => body,
            None => {
                return Err(Error::RenderError(EngineType::XML, format!("error at `render_lazy_loaded_images()`\n`{}` doesn't have a `<body>` tag!", article_title)));
            }
        };

        body.add_contents(hxml::Content::from_string(script.clone()).unwrap());
    }

    Ok(())
}

pub fn import_extra_files(article_title: String, extra_scripts: Vec<String>, extra_styles: Vec<String>) -> Result<(), Error> {
    let body = match hxml::dom::get_element_by_tag_name(None, "body".to_string()) {
        Some(body) => body,
        None => {
            return Err(Error::RenderError(EngineType::XML, format!("error at `import_extra_files({:?})`\n`{}` doesn't have a `<body>` tag!", article_title, article_title)));
        }
    };
    let head = match hxml::dom::get_element_by_tag_name(None, "head".to_string()) {
        Some(head) => head,
        None => {
            return Err(Error::RenderError(EngineType::XML, format!("error at `import_extra_files({:?})`\n`{}` doesn't have a `<head>` tag!", article_title, article_title)));
        }
    };

    for script in extra_scripts.iter() {
        body.add_element_ptr(hxml::Element::from_string(format!("<script src=\"{script}\" defer=\"defer\"></script>")).unwrap());
    }

    for style in extra_styles.iter() {
        head.add_element_ptr(hxml::Element::from_string(format!("<link href=\"{style}\" rel=\"stylesheet\"/>")).unwrap());
    }

    Ok(())
}

pub fn set_title(old_title: String, new_title: String) -> Result<(), Error> {
    let title_tag = match hxml::dom::get_element_by_tag_name(None, "title".to_string()) {
        Some(title) => title,
        None => {
            return Err(Error::RenderError(EngineType::XML, format!("error at `set_title()`\n`{}` doesn't have a `<title>` tag!", old_title)));
        }
    };

    let contents = title_tag.get_contents_mut();
    *contents = vec![Content::new_char_data(new_title)];

    Ok(())
}

pub fn load_external_files(title: String, path: &str) -> Result<(), Error> {
    let links = hxml::dom::get_elements_by_tag_name(None, "link".to_string());
    let scripts = hxml::dom::get_elements_by_tag_name(None, "script".to_string());
    let body = match hxml::dom::get_element_by_tag_name(None, "body".to_string()) {
        Some(body) => body,
        None => {
            return Err(Error::RenderError(EngineType::XML, format!("error at `load_external_files({:?}, {:?}, ...)`\n`{}` doesn't have a `<body>` tag!", title, path, title)));
        }
    };

    for link in links.iter() {
        match link.get_attribute("href".to_string()) {
            Some(href) => {
                hxml::dom::delete(*link);

                match extension(&href) {
                    Ok(Some(ext)) if ext == "css".to_string() => {
                        match read_string(&join(path, &href).unwrap()) {
                            Ok(css) => {
                                body.add_element_ptr(hxml::Element::from_string(format!("<style>/*<![CDATA[*/{css}/*]]>*/</style>")).unwrap());
                            },
                            Err(e) => {
                                return Err(Error::PathError(format!("error {:?} at `load_external_files({:?}, {:?}, ...)`\n`read_string({:?})` failed", e.render_err(), title, path, join(path, &href).unwrap())));
                            }
                        }
                    },
                    Err(e) => {
                        return Err(Error::PathError(format!("error {:?} at `load_external_files({:?}, {:?}, ...)`\n`extension({:?})` failed", e.render_err(), title, path, &href)));
                    },
                    _ => {}
                }
            },
            _ => {}
        }
    }

    for script in scripts.iter() {
        match script.get_attribute("src".to_string()) {
            Some(src) => {
                hxml::dom::delete(*script);

                match extension(&src) {
                    Ok(Some(ext)) if ext == "js" => {
                        match read_string(&join(path, &src).unwrap()) {
                            Ok(js) => {
                                body.add_element_ptr(hxml::Element::from_string(format!("<script>/*<![CDATA[*/{js}/*]]>*/</script>")).unwrap());
                            },
                            Err(e) => {
                                return Err(Error::PathError(format!("error {:?} at `load_external_files({:?}, {:?}, ...)`\n`read_string({:?})` failed", e.render_err(), title, path, join(path, &src).unwrap())));
                            }
                        }
                    },
                    Err(e) => {
                        return Err(Error::PathError(format!("error {:?} at `load_external_files({:?}, {:?}, ...)`\n`extension({:?})` failed", e.render_err(), title, path, src)));
                    },
                    _ => {},
                }
            },
            _ => {}
        }
    }

    Ok(())
}
