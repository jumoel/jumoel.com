use std::{
    ffi::OsStr,
    fmt::Display,
    fs::{self, DirBuilder, File},
    io::Write,
    path::{Path, PathBuf},
};

use askama::Template;
use chrono::{NaiveDateTime, ParseError};
use pulldown_cmark::{Event, MetadataBlockKind, Options, Parser, Tag, TagEnd};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(try_from = "String")]
struct PostDate(NaiveDateTime);

impl PostDate {
    fn format(&self, pattern: &str) -> String {
        self.0.format(pattern).to_string()
    }
}

impl Display for PostDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl TryFrom<String> for PostDate {
    type Error = ParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        NaiveDateTime::parse_from_str(&value, "%Y-%m-%d %H:%M:%S").map(Self)
    }
}

#[derive(Debug, Deserialize)]
struct Frontmatter {
    title: String,
    categories: Vec<String>,
    date: PostDate,
}

#[derive(Debug)]
struct Post {
    filename: String,
    frontmatter: Frontmatter,
    raw_content: String,
    html: String,
}

#[derive(Debug, Template)]
#[template(path = "post.html")]
struct PostTemplate<'a> {
    title: &'a str,
    date: &'a PostDate,
    content: &'a str,
    frontmatter: &'a Frontmatter,
}

#[derive(Debug, Template)]
#[template(path = "page.html")]
struct PageTemplate<'a> {
    posts: &'a Vec<Post>,
    raw_content: &'a str,
}

#[derive(Debug)]
struct Page {
    outfile: PathBuf,
    content: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_folder = Path::new("./_build");
    if Path::new(out_folder).is_dir() {
        fs::remove_dir_all(out_folder)?;
    }

    let posts = Path::new("./_posts")
        .read_dir()?
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .filter(|p| {
            let path = p.path();

            path.is_file() && path.extension() == Some(OsStr::new("md"))
        })
        .filter_map(|p| {
            let filename = p.file_name().into_string().ok()?;
            let raw_content = fs::read_to_string(p.path()).ok()?;

            let options = Options::ENABLE_YAML_STYLE_METADATA_BLOCKS
                | Options::ENABLE_GFM
                | Options::ENABLE_TABLES;

            let mut parser = Parser::new_ext(&raw_content, options).peekable();

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

                let fm: Frontmatter = yaml_serde::from_str(&yaml).unwrap_or_else(|err| {
                    panic!("YAML deserialization error.\n{}\n{}", err, filename)
                });

                Some(fm)
            } else {
                None
            };

            let frontmatter =
                frontmatter.unwrap_or_else(|| panic!("Missing frontmatter for file: {}", filename));

            let mut html = String::new();
            pulldown_cmark::html::push_html(&mut html, parser);

            Some(Post {
                filename,
                frontmatter,
                raw_content,
                html,
            })
        })
        .collect::<Vec<_>>();

    for post in &posts {
        let rendered = PostTemplate {
            title: &post.frontmatter.title,
            date: &post.frontmatter.date,
            content: &post.html,
            frontmatter: &post.frontmatter,
        };

        let slug_re = regex::Regex::new(r"\d{4}-\d{2}-\d{2}-(?P<slug>.+)\.md")?;
        let captures = slug_re
            .captures(&post.filename)
            .expect("Extracting slug from filename failed");
        let slug = &captures["slug"];

        let out_path = out_folder.join(format!("posts/{}/index.html", slug));
        let out_dir = out_path.parent().expect("Outfile needs a parent path");

        DirBuilder::new().recursive(true).create(&out_dir)?;
        let mut fd = std::fs::File::create(&out_path)?;

        rendered.write_into(&mut fd)?;
    }

    let pages = Path::new("./_pages")
        .read_dir()?
        .filter_map(Result::ok)
        .filter_map(|f| {
            let path = f.path();

            let outfile = match (path.extension(), path.file_stem()) {
                (Some(ext), Some(name)) if ext == "html" && name == "index" => {
                    format!("index.html")
                }
                (Some(ext), Some(name)) if ext == "html" => {
                    format!("{}/index.html", name.to_str()?)
                }
                _ => return None,
            };

            let raw_content = fs::read_to_string(path).ok()?;

            let template = PageTemplate {
                posts: &posts,
                raw_content: &raw_content,
            };

            let content = template.render().ok()?;

            Some(Page {
                outfile: out_folder.join(outfile),
                content,
            })
        })
        .collect::<Vec<_>>();

    for page in &pages {
        DirBuilder::new()
            .recursive(true)
            .create(&page.outfile.parent().expect("Outfile needs a parent path"))?;
        let mut fd = File::create(&page.outfile)?;
        fd.write_all(&page.content.as_bytes())?;
    }

    // TODO: Omskriv det her til MiniJinja, jeg har brug for runtime templates

    Ok(())
}
