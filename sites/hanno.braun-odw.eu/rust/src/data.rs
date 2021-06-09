use std::vec;

use ssg::markdown::Markdown;

pub struct SideProjects(pub Vec<SideProject>);

impl IntoIterator for SideProjects {
    type Item = SideProject;

    type IntoIter = vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

pub struct SideProject {
    pub title: &'static str,
    pub date: &'static str,
    pub description: Markdown,
}
