use std::path::PathBuf;

pub struct Trigger {
    pub kind: &'static str,
    pub paths: Vec<PathBuf>,
}

impl Trigger {
    pub fn new(event: notify::Event) -> Option<Self> {
        let kind = match event.kind {
            notify::EventKind::Any => "any",
            notify::EventKind::Access(_) => {
                // Access is non-mutating, so not interesting to us.
                return None;
            }
            notify::EventKind::Create(_) => "create",
            notify::EventKind::Modify(_) => "modify",
            notify::EventKind::Remove(_) => "remove",
            notify::EventKind::Other => "other",
        };

        Some(Self {
            kind,
            paths: event.paths,
        })
    }
}
