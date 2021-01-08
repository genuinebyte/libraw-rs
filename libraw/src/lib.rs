#![doc(html_root_url = "https://docs.rs/libraw-rs/0.0.2")]

pub use self::error::{Error, Result};
pub use self::image::ProcessedImage;
pub use self::processor::Processor;
pub use self::rawimage::RawImage;
pub use libraw_sys::libraw_image_sizes_t as Sizes;
pub use libraw_sys::libraw_colordata_t as Colordata;

mod error;
mod image;
mod processor;
mod rawimage;
