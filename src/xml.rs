use crate::engine::EngineType;
use crate::error::Error;

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
            Some(head) => head,
            None => {
                return Err(Error::RenderError(EngineType::XML, format!("error at `render_directory()`\n`{}` doesn't have a `<body>` tag!", article_title)));
            }
        };
        let modal_box = hxml::Content::from_string(image_box.clone()).unwrap();

        body.add_contents(modal_box);
    }

    Ok(())
}

pub fn render_collapsible_tables(article_title: String) -> Result<(), Error> {
    let body = match hxml::dom::get_element_by_tag_name(None, "body".to_string()){
        Some(head) => head,
        None => {
            return Err(Error::RenderError(EngineType::XML, format!("error at `render_collapsible_tables()`\n`{}` doesn't have a `<body>` tag!", article_title)));
        }
    };
    body.add_element_ptr(hxml::Element::from_string("<script src=\"collapsible_tables.js\"></script>".to_string()).unwrap());

    Ok(())
}