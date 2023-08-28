//! **parakeet** provides an easy to use wrapper around [headless_chrome](https://docs.rs/headless_chrome)
//! that provides an asynchronous function for PDF rendering. This is useful for rendering PDFs in a web server,
//! as launching is slow, therefore should be done once. It controls many tabs at once, giving you the ability
//! to render multiple PDFs at once.
//!
pub mod browser;
pub mod page_props;
pub mod rendering_engine;
