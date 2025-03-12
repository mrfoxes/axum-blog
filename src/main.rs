use axum::{
    extract::Path,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, Router},
    Extension,
};
use handlebars::Handlebars;
use serde::Serialize;
use std::{
    collections::HashMap,
    fs,
    path::PathBuf,
    sync::Arc,
};
use chrono::{DateTime, Local, TimeZone};

// Define metadata structure for blog posts
#[derive(Serialize, Default, Debug)]
struct Metadata {
    title: String,
    date: String,
    tags: Vec<String>,
    summary: String,
}

// Define blog post structure
#[derive(Serialize, Debug)]
struct Post {
    title: String,
    content: String,
    summary: String,
    date: String,
    tags: Vec<String>,
    filename: String,
    slug: String,
}

// Parse metadata from Markdown file content
fn parse_metadata(content: &str) -> Option<Metadata> {
    let parts: Vec<&str> = content.splitn(3, "---").collect();
    if parts.len() < 3 {
        return None;
    }
    let meta_str = parts[1];
    let mut meta = Metadata::default();
    for line in meta_str.lines() {
        let kv: Vec<&str> = line.splitn(2, ':').collect();
        if kv.len() == 2 {
            let key = kv[0].trim();
            let value = kv[1].trim();
            match key {
                "title" => meta.title = value.to_string().replace("\"", ""),
                "date" => meta.date = value.to_string(),
                "tags" => meta.tags = value.split(',').map(|s| s.trim().to_string().replace("\"", "")).collect(),
                "summary" => meta.summary = value.to_string().replace("\"", ""),
                _ => {}
            }
        }
    }
    Some(meta)
}

// Convert Markdown content to HTML
fn markdown_to_html(markdown: &str) -> String {
    let parser = pulldown_cmark::Parser::new(markdown);
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);
    html_output
}

// Retrieve all blog posts from content directory
fn get_posts() -> Vec<Post> {
    let content_dir = PathBuf::from("content");
    let mut posts = Vec::new();

    if let Ok(entries) = fs::read_dir(content_dir) {
        for entry in entries.flatten() {
            if entry.path().extension().map(|s| s == "md").unwrap_or(false) {
                if let Ok(content) = fs::read_to_string(entry.path()) {
                    if let Some(metadata) = parse_metadata(&content) {
                        let content_str = content.splitn(3, "---").nth(2).unwrap_or("");
                        let html_content = markdown_to_html(content_str);

                        let date = match DateTime::parse_from_str(&metadata.date, "%Y-%m-%d") {
                            Ok(d) => Local.from_utc_datetime(&d.naive_utc()),
                            Err(_) => entry.metadata().ok().and_then(|m| m.modified().ok())
                               .map(|t| DateTime::<Local>::from(t)).unwrap_or_else(Local::now),
                        };

                        let post = Post {
                            title: metadata.title.clone(),
                            content: html_content,
                            summary: metadata.summary,
                            date: date.format("January 2, 2006").to_string(),
                            tags: metadata.tags,
                            filename: entry.file_name().to_str().unwrap_or("").to_string(),
                            slug: metadata.title.to_lowercase().replace(" ", "-"),
                        };
                        posts.push(post);
                    }
                }
            }
        }
    }

    posts.sort_by_key(|p| p.date.clone());
    posts.reverse();
    posts
}

// Home route handler
async fn index(
    Extension(hb): Extension<Arc<Handlebars<'_>>>,
) -> impl IntoResponse {
    let posts = get_posts();
    let mut data = HashMap::new();
    data.insert("posts", posts);
    match hb.render("index.html", &data) {
        Ok(rendered) => Html(rendered).into_response(),
        Err(e) => {
            eprintln!("Failed to render index template: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Template rendering error").into_response()
        },
    }
}

// Single post route handler
async fn single_post(
    Extension(hb): Extension<Arc<Handlebars<'_>>>,
    Path(post_title): Path<String>,
) -> impl IntoResponse {
    let posts = get_posts();
    let post = posts.into_iter().find(|p| p.slug == post_title);

    if let Some(post) = post {
        let mut data = HashMap::new();
        data.insert("post", post);
        match hb.render("single.html", &data) {
            Ok(rendered) => Html(rendered).into_response(),
            Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Template rendering error").into_response(),
        }
    } else {
        (StatusCode::NOT_FOUND, "Post not found").into_response()
    }
}

#[tokio::main]
async fn main() {
    let mut hb = Handlebars::new();
    if let Err(e) = hb.register_template_file("index.html", "templates/index.html") {
        eprintln!("Failed to register index template: {}", e);
        return;
    }
    if let Err(e) = hb.register_template_file("single.html", "templates/single.html") {
        eprintln!("Failed to register single template: {}", e);
        return;
    }
    let hb = Arc::new(hb);

    let app = Router::new()
      .route("/", get(index))
      .route("/blog/{post_title}", get(single_post))
      .layer(Extension(hb));

    println!("Listening on port 8080");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}