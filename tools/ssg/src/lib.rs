pub mod args;
pub mod build;
pub mod serve;

pub use ssg_derive::Component;

pub trait Component: Into<build::html::model::Element> {}
