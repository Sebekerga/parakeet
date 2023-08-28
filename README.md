**parakeet** provides an easy to use wrapper around [headless_chrome](https://docs.rs/headless_chrome) that provides an asynchronous function for PDF rendering of HTML documents. This is useful for rendering PDFs in a web server, as launching is slow, therefore should be done once. It controls many tabs at once, thus giving ability to render multiple PDFs at once.

## Usage 
```rust
use std::sync::Arc;
use parakeet_pdf::{
    page_props::PagePropertiesBuilder,
    rendering_engine::RenderingEngine,
};

#[tokio::main]
async fn main() {
    const TAB_COUNT: usize = 5;
    let engine = Arc::new(RenderingEngine::new(TAB_COUNT).unwrap());

    let html = format!("Hello, html!");
    let page_props = PagePropertiesBuilder::from_size(10.0, 10.0).build();   
    let _pdf = engine.render_html(&html, page_props).await.unwrap();
}
```