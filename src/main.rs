mod file_io;
mod engine;
mod error;
mod log;
mod yaml_hash;

use engine::{EngineType, render_directory};

fn main() {

    let color_context = get_colors();

    render_directory(
        "./templates/sass", "tera",
        EngineType::Tera,
        "./output/styles", "scss",
        &Some(color_context),
        &None
    ).unwrap();

    render_directory(
        "./output/styles", "scss",
        EngineType::Scss,
        "./output/styles", "css",
        &None,
        &None
    ).unwrap();
}

use mdxt::{COLORS, Color};

fn get_colors() -> tera::Context {

    let mut context = tera::Context::new();

    let color_names = COLORS.iter().map(|color| color.name.clone()).collect::<Vec<String>>();
    context.insert("colors", &color_names);

    let hex = COLORS.iter().map(|color| color.to_hex()).collect::<Vec<String>>();
    context.insert("hex", &hex);

    let compl_hex = COLORS.iter().map(|color| color.complement().to_hex()).collect::<Vec<String>>();
    context.insert("compl_hex", &compl_hex);

    context
}