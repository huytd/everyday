use std::{fs::File, io::{Result, BufReader, BufRead, Write}};

#[derive(Debug)]
struct Post {
    title: String,
    content: String
}

impl Post {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            content: String::new()
        }
    }

    pub fn add_line(&mut self, line: &str) {
        self.content.push_str(&line);
        self.content.push_str("\n");
    }
}

fn main() -> Result<()> {
    let file = File::open("DEVLOG.md")?;
    let reader = BufReader::new(file);

    let mut posts: Vec<Post> = vec![];
    let mut current_post: Option<Post> = None;
    for line in reader.lines() {
        let line = line?;
        if line.starts_with("# ") {
            if let Some(post) = current_post {
                posts.push(post);
            }
            current_post = Some(Post {
                title: line.replace("# ", ""),
                content: line + "\n"
            });
        } else {
            if let Some(post) = &mut current_post {
                post.add_line(&line);
            }
        }
    }

    if let Some(post) = current_post {
        posts.push(post);
    }

    for post in posts {
        let file_name = format!("./output/{}.md", post.title.replace("/", " "));
        let mut out_file = File::create(&file_name)?;
        out_file.write_all(post.content.as_bytes())?;
    }

    Ok(())
}
