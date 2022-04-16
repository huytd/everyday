use chrono::prelude::*;
use std::{fs::{self, File}, io::{Result, Write}};

#[derive(Debug)]
struct Post {
    content: String,
    date: NaiveDate
}

fn main() -> Result<()> {
    let mut posts: Vec<Post> = vec![];
    let mut total_content_size = 0;

    let source = fs::read_dir("notes")?;
    for entry in source {
        if let Ok(entry) = entry {
            if let Some(file_name) = entry.file_name().to_str() {
                if !file_name.starts_with(".") && !file_name.starts_with("_") && file_name.ends_with(".md") {
                    let content = fs::read_to_string(format!("notes/{}", file_name))?;
                    let title = content.lines().next().unwrap().replace("# ", "");
                    let date_str = title.split_once(" - ").unwrap().0;
                    if let Ok(date) = NaiveDate::parse_from_str(date_str, "%m.%d.%Y") {
                        total_content_size += content.len() + 2;
                        let post = Post {
                            content,
                            date
                        };
                        posts.push(post);
                    }
                }
            }
        }
    }

    posts.sort_by(|a, b| b.date.cmp(&a.date));
    let mut content = String::with_capacity(total_content_size);

    for post in &posts {
        content.push_str(&post.content);
        content.push_str("\n\n");
    }

    let mut out_file = File::create("DEVLOG.md")?;
    out_file.write_all(content.as_bytes())?;

    println!("Processing completed. Total {} posts", posts.len());

    Ok(())
}
