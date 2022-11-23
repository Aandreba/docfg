//! Crate thet facilitates the documentation of feature-gated code. This crate requires the use of the `doc_cfg` nightly feature.
//! 
//! Remember to add the following to your `Cargo.toml` file for the crate make efect when documenting on `docs.rs`
//! ```
//! [package.metadata.docs.rs]
//! all-features = true
//! rustdoc-args = ["--cfg", "docsrs"]
//! ```
//! 
//! And when generating local documentation, use
//! ```cargo rustdoc -- --cfg docsrs```
//! 
//! ## Example
//! 
//! ```rust
//! #[docfg(test)]
//! fn test () {
//!     // ...  
//! }
//! ```
//! 
//! ```rust
//! #[cfg_attr(docsrs, doc(cfg(test)))]
//! #[cfg(test)]
//! fn test () {
//!     // ...
//! }
//! ```

use proc_macro2::TokenStream;

#[proc_macro_attribute]
pub fn docfg (attrs: proc_macro::TokenStream, items: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let attrs = TokenStream::from(attrs);
    let items = TokenStream::from(items);

    return quote::quote! {
        #[cfg_attr(docsrs, doc(cfg(#attrs)))]
        #[cfg(#attrs)]
        #items
    }.into()
}