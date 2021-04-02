use std::path::Path;

use kuchiki::NodeRef;

pub trait Transform {
    fn transform(&mut self, source: &Path, document: &mut NodeRef);
}
