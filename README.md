**parakeet** provides an easy to use wrapper around [headless_chrome](https://docs.rs/headless_chrome) that allows an asynchronous function for PDF rendering. This is useful for rendering PDFs in a web server, as launching is slow, therefore should be done once. It controls many tabs at once, giving you the ability to render multiple PDFs at once.

## Usage 
```rust
...
// once the application starts
const TAB_COUNT: usize = 10;
let engine = Arc::new(RenderingEngine::new(10));
...

...
// when you want to render a PDF
let pdf_blob = engine.render_html(&html).await?;
...
```