//! # footprint-parser
//!
//! Parser for the [Footprint Protocol](https://footprint.eco) — an open standard
//! for embedding carbon footprint metadata in HTML pages via `<meta>` tags.
//!
//! ## Quick start
//!
//! ```rust
//! use footprint_parser::parse;
//!
//! let html = r#"<html><head>
//!   <meta property="fp:product" content="Fairphone 5" />
//!   <meta property="fp:co2e" content="23.6" />
//!   <meta property="fp:co2e:unit" content="kg" />
//! </head></html>"#;
//!
//! let data = parse(html).unwrap();
//! println!("{}: {} {}", data.product, data.co2e, data.co2e_unit);
//! ```

pub mod error;
pub mod parser;
pub mod types;

pub use error::ParseError;
pub use parser::parse;
pub use types::{Breakdown, Co2Unit, FootprintData, Scope};
