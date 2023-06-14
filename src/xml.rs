use crate::engine::EngineType;
use crate::error::Error;
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

pub fn add_js(article_title: String, collapsible_tables: bool, tooltips: bool, sidebar: bool) -> Result<(), Error> {

    if !collapsible_tables && !tooltips && !sidebar {
        return Ok(());
    }

    let body = match hxml::dom::get_element_by_tag_name(None, "body".to_string()) {
        Some(body) => body,
        None => {
            return Err(Error::RenderError(EngineType::XML, format!("error at `render_collapsible_tables()`\n`{}` doesn't have a `<body>` tag!", article_title)));
        }
    };

    if collapsible_tables {
        body.add_element_ptr(hxml::Element::from_string("<script src=\"collapsible_tables.js\"></script>".to_string()).unwrap());
    }

    if collapsible_tables {
        body.add_element_ptr(hxml::Element::from_string("<script src=\"tooltips.js\"></script>".to_string()).unwrap());
    }

    if collapsible_tables {
        body.add_element_ptr(hxml::Element::from_string("<script src=\"sidebar.js\"></script>".to_string()).unwrap());
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