use notes_cli_markdown::parse;
use notes_cli_rust_gen::generate;
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use std::path::PathBuf;
use std::{env, fs};
use syn::{Error, LitStr};

/// The declaration of the Markdown file `path` names, rebuilt when the file changes.
pub fn file(path: &LitStr) -> TokenStream {
    // Relative to the crate root, which rustc and rust-analyzer both name in `CARGO_MANIFEST_DIR`.
    let root = env::var_os("CARGO_MANIFEST_DIR").map(PathBuf::from).unwrap_or_default();
    let file = root.join(path.value());
    let shown = file.display().to_string();
    let markdown = match fs::read_to_string(&file) {
        Ok(markdown) => markdown,
        Err(error) => return Error::new(path.span(), format!("cannot read {shown}: {error}")).to_compile_error(),
    };
    // `include_str!` makes the file an input of the crate, so editing it recompiles the crate.
    declare(&markdown, path.span(), &quote! { const _: &str = include_str!(#shown); })
}

/// The declaration of the Markdown `markdown` holds.
pub fn inline(markdown: &LitStr) -> TokenStream {
    declare(&markdown.value(), markdown.span(), &TokenStream::new())
}

fn declare(markdown: &str, span: Span, track: &TokenStream) -> TokenStream {
    let spec = match parse(markdown) {
        Ok(spec) => spec,
        Err(error) => return Error::new(span, error.to_string()).to_compile_error(),
    };
    let rust = match generate(&spec) {
        Ok(rust) => rust,
        Err(error) => return Error::new(span, error.to_string()).to_compile_error(),
    };
    let rust: TokenStream = match rust.parse() {
        Ok(rust) => rust,
        Err(error) => return Error::new(span, format!("generated Rust does not parse: {error}")).to_compile_error(),
    };
    let module = format_ident!("{}_cli", spec.name.replace('-', "_"));
    quote! {
        mod #module {
            #track
            #rust
        }
        pub use #module::*;
    }
}
