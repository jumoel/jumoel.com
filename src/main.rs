use std::{
    ffi::OsStr,
    fs::{self, read_dir},
    path::Path,
};

use pulldown_cmark::Options;

#[derive(Debug)]
struct Post {
    filename: String,
    raw: String,
    html: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new("./_posts");
    let posts = path
        .read_dir()?
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .filter(|p| {
            let path = p.path();

            path.is_file() && path.extension() == Some(OsStr::new("md"))
        })
        .filter_map(|p| {
            let filename = p.file_name().into_string().ok()?;
            let raw = fs::read_to_string(p.path()).ok()?;

            let parser = pulldown_cmark::Parser::new_ext(
                &raw,
                Options::ENABLE_YAML_STYLE_METADATA_BLOCKS
                    & Options::ENABLE_GFM
                    & Options::ENABLE_TABLES,
            );
            let mut html = String::new();
            pulldown_cmark::html::push_html(&mut html, parser);

            Some(Post {
                filename,
                raw,
                html,
            })
        })
        .collect::<Vec<_>>();

    println!("{:?}", posts.first());

    Ok(())
}
