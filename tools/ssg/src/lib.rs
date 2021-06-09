pub mod args;
pub mod build;
pub mod html;
pub mod markdown;
pub mod serve;

pub use ssg_derive::Component;

pub trait Component: Into<html::model::Element> {}
