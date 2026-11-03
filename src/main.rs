use std::{
    ffi::OsStr,
    fs::{self},
    path::Path,
};

use pulldown_cmark::{Event, MetadataBlockKind, Options, Parser, Tag, TagEnd};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Frontmatter {
    title: String,
    categories: Vec<String>,
    date: Option<String>,
}

#[derive(Debug)]
struct Post {
    filename: String,
    frontmatter: Option<Frontmatter>,
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

            let options = Options::ENABLE_YAML_STYLE_METADATA_BLOCKS
                | Options::ENABLE_GFM
                | Options::ENABLE_TABLES;

            let mut parser = Parser::new_ext(&raw, options).peekable();

            let frontmatter = if matches!(
                parser.peek(),
                Some(Event::Start(Tag::MetadataBlock(
                    MetadataBlockKind::YamlStyle
                )))
            ) {
                parser.next();

                let mut yaml = String::new();

                for event in parser.by_ref() {
                    match event {
                        Event::Text(text) => yaml.push_str(&text),
                        Event::End(TagEnd::MetadataBlock(MetadataBlockKind::YamlStyle)) => break,
                        _ => {}
                    }
                }

                let mut fm: Frontmatter =
                    yaml_serde::from_str(&yaml).expect("yaml deserialization failed");

                if fm.date.is_none() {
                    let (filedate, _) = filename.split_at("YYYY-MM-DD".len());
                    fm.date = Some(filedate.to_string());
                }

                Some(fm)
            } else {
                None
            };

            let mut html = String::new();
            pulldown_cmark::html::push_html(&mut html, parser);

            Some(Post {
                filename,
                frontmatter,
                raw,
                html,
            })
        })
        .collect::<Vec<_>>();

    let p = posts.first().expect("No posts found");

    println!("{:?}", p.frontmatter);

    Ok(())
}
