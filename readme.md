# Leapcell Static Blog Example

This project demonstrates how to deploy a static blog built with Rust and Axum on Leapcell. The blog generates HTML pages from Markdown files stored in the `content` folder and serves them using Axum. The project is designed to educate users on how to deploy a static website on Leapcell.

## Prerequisites

Before running the project, ensure that you have the following:

1. Rust and Cargo installed on your local machine or Leapcell platform.
2. The necessary dependencies and configuration files in place.

## Setup

1. Clone the repository:
   ```bash
   git clone https://github.com/leapcell/axum-blog.git
   cd static-blog-rust
   ```

2. Install the required dependencies:
   ```bash
   cargo build
   ```

3. Start the application:
   ```bash
   cargo run
   ```

4. The application will be available at `http://localhost:8080`. It will serve the generated static pages based on the Markdown files in the `content` directory.

## File Structure

Here is the structure of the project:

```
.
├── Cargo.toml                      # Rust project configuration file
├── LICENSE                          # License file
├── content
│   ├── first-review.md              # First markdown content file
│   └── second.md                   # Second markdown content file
├── leapcell.yaml                   # Leapcell configuration file for deployment
├── src
│   └── main.rs                     # Main Rust file, sets up Axum and routes
└── templates
    ├── index.html                  # Template for the homepage
    └── single.html                 # Template for individual blog post pages
```

## Explanation of Files

### `Cargo.toml`

This is the configuration file for the Rust project. It specifies the project dependencies, including Axum for web handling and other necessary libraries for parsing Markdown files and rendering templates.

### `content/`

This directory contains the Markdown files that will be parsed and converted into static HTML pages. Each `.md` file represents a blog post or page:

- **`first-review.md`**: Example of a blog post or content file.
- **`second.md`**: Another example content file.

### `leapcell.yaml`

This is the Leapcell configuration file that contains settings for deploying your application on the Leapcell platform. It specifies how the service should be built, deployed, and scaled.

### `src/main.rs`

This is the main entry point of the Axum server. It handles:

- Setting up Axum routes to serve the content.
- Reading and parsing Markdown files from the `content/` folder.
- Rendering the content using the `index.html` or `single.html` templates.

### `templates/`

Contains HTML templates for rendering the static pages:

- **`index.html`**: The template for the homepage, which lists all the available blog posts.
- **`single.html`**: The template for displaying an individual blog post.

## How It Works

1. The Axum server is set up to read the Markdown files from the `content/` folder.
2. The server converts each Markdown file into an HTML page using the `single.html` template.
3. The homepage (`index.html`) lists the blog posts and links to individual posts.
4. When a user clicks on a post, the server renders the individual post page using the `single.html` template.

## Deployment on Leapcell

To deploy this project on Leapcell:

1. Ensure that the Rust environment is prepared by following the setup steps.
2. Configure the `leapcell.yaml` file for deployment.
3. Deploy the application on Leapcell, where it will automatically scale as needed to serve static content.

---

This example demonstrates how you can easily deploy a static website using Rust and Axum on Leapcell. The platform provides seamless scaling and easy deployment for static content, without the need for managing infrastructure.

If you have any questions or need help with the deployment process, feel free to reach out.