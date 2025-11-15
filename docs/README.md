# Circuit Documentation

This directory contains the comprehensive documentation website for Circuit, built with [mdBook](https://rust-lang.github.io/mdBook/).

## Quick Start

### Build the Documentation

```bash
./build.sh
```

This script will:
1. Generate Rust API documentation using `cargo doc`
2. Build the mdBook documentation
3. Integrate the cargo docs into the mdBook output

### Serve Locally

```bash
mdbook serve --open
```

Or specify a port:

```bash
mdbook serve --port 8080 --open
```

### Build Only mdBook

```bash
mdbook build
```

### Build Only Cargo Docs

```bash
cd .. && cargo doc --no-deps --all-features
```

## Structure

```
docs/
├── book.toml           # mdBook configuration
├── build.sh            # Build script (builds both mdBook and cargo docs)
├── src/                # Documentation source files
│   ├── SUMMARY.md      # Table of contents
│   ├── introduction.md # Landing page
│   ├── getting-started/
│   ├── guide/
│   ├── platforms/
│   ├── advanced/
│   ├── examples/
│   ├── api/
│   └── contributing/
├── theme/              # Custom theme files
│   ├── custom.css      # Custom styling
│   └── custom.js       # Custom JavaScript
└── book/               # Generated output (gitignored)
    ├── index.html
    ├── rustdoc/        # Integrated cargo docs
    └── ...
```

## Documentation Sections

### Getting Started
- Quick Start
- Installation
- Your First Flow
- Architecture Overview

### User Guide
- Understanding Blocks
- Creating Flows
- The Declarative Language
  - Block Syntax
  - Flow Syntax
  - Type System
- Built-in Blocks
- Creating Custom Blocks
- The Graph Engine
- Values and Types
- Error Handling

### Platform Integration
- Platform Overview
- Swift (iOS/macOS)
- Kotlin (Android)
- React (Web)
- WebAssembly

### Advanced Topics
- Building from Source
- FFI Integration
- Performance Optimization
- Testing Your Blocks
- Cross-Compilation

### Examples
- Calculator Example
- Data Pipeline
- String Processing

### API Reference
- Core API
- Language API
- WASM API
- FFI API
- Generated Rust Docs

### Contributing
- How to Contribute
- Development Setup
- Code Style

## Writing Documentation

### Adding a New Page

1. Create a new Markdown file in the appropriate directory:
   ```bash
   touch src/guide/my-new-page.md
   ```

2. Add it to `src/SUMMARY.md`:
   ```markdown
   - [My New Page](./guide/my-new-page.md)
   ```

3. Write your content using standard Markdown

4. Build and preview:
   ```bash
   mdbook serve
   ```

### Markdown Features

mdBook supports:
- Standard Markdown
- GitHub-flavored Markdown
- Syntax highlighting for code blocks
- Table of contents
- Search functionality
- Custom CSS/JS

### Code Blocks

Use fenced code blocks with language specifiers:

````markdown
```rust
fn main() {
    println!("Hello, Circuit!");
}
```

```flow
flow example {
    node const: core.constant { value = 42 }
}
```
````

### Cross-References

Link to other pages using relative paths:

```markdown
See the [Architecture Overview](./getting-started/architecture.md) for details.
```

### Images

Add images to `src/images/` and reference them:

```markdown
![Architecture Diagram](./images/architecture.png)
```

## Custom Theme

Custom styling is in `theme/`:

- `custom.css` - Custom styles (colors, layout, etc.)
- `custom.js` - Custom JavaScript (copy buttons, anchor links, etc.)

## Configuration

Edit `book.toml` to configure:

- Book metadata (title, authors)
- Theme settings
- Output formats
- Preprocessors
- Search settings

## Deployment

The documentation can be deployed to:

- **GitHub Pages**: Automated via GitHub Actions (recommended)
- **Netlify**: Point to `docs/book/`
- **Any static host**: Serve the `book/` directory

### GitHub Pages (Automated)

The repository includes a GitHub Actions workflow (`.github/workflows/deploy-docs.yml`) that automatically builds and deploys documentation to GitHub Pages on every push to `main`.

#### Setup GitHub Pages

1. Go to your repository settings on GitHub
2. Navigate to **Settings** → **Pages**
3. Under **Source**, select **GitHub Actions**
4. The documentation will be automatically deployed on the next push to `main`
5. Access your docs at: `https://<username>.github.io/<repo-name>/`

#### Manual Trigger

You can also manually trigger the deployment:

1. Go to **Actions** tab on GitHub
2. Select **Deploy Documentation** workflow
3. Click **Run workflow**

#### Local Preview Before Deploy

Always preview locally before pushing:

```bash
cd docs
./build.sh
mdbook serve --open
```

### GitHub Pages (Manual)

If you prefer manual deployment:

```bash
# Build the docs
cd docs
./build.sh

# Copy to gh-pages branch
git checkout gh-pages
cp -r book/* .
git add .
git commit -m "Update documentation"
git push origin gh-pages
```

### Other Hosting Options

**Netlify:**
- Connect your repository
- Set build command: `cd docs && ./build.sh`
- Set publish directory: `docs/book`

**Vercel:**
- Import your repository
- Framework: Other
- Build command: `cd docs && ./build.sh`
- Output directory: `docs/book`

## Maintenance

### Updating Dependencies

```bash
cargo install mdbook --force
```

### Checking Links

```bash
mdbook test
```

### Formatting

Keep Markdown files formatted:
- Use 2 spaces for indentation
- Keep lines under 100 characters when possible
- Use ATX-style headers (`#` not `===`)

## Tools

Recommended tools:

- **mdBook**: Documentation builder
- **mdbook-linkcheck**: Check for broken links
- **mdbook-toc**: Generate tables of contents
- **mdbook-mermaid**: Diagrams (optional)

Install additional tools:

```bash
cargo install mdbook-linkcheck
cargo install mdbook-toc
```

Add to `book.toml`:

```toml
[preprocessor.linkcheck]

[preprocessor.toc]
```

## Contributing to Docs

1. Find the relevant file in `src/`
2. Make your changes
3. Test locally with `mdbook serve`
4. Submit a pull request

## Help

- [mdBook Documentation](https://rust-lang.github.io/mdBook/)
- [Markdown Guide](https://www.markdownguide.org/)
- [Circuit GitHub](https://github.com/blankly-app/circuit)

## License

Documentation is licensed under the same license as the Circuit project (MIT).
