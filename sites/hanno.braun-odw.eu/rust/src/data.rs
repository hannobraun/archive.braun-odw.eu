use ssg::markdown::Markdown;

pub struct SideProject {
    pub title: &'static str,
    pub date: &'static str,
    pub description: Markdown,
}
