use std::{
    fmt,
    path::{Path, PathBuf},
};

use anyhow::Context as _;

pub struct Trigger {
    pub kind: &'static str,
    pub paths: Vec<PathBuf>,
    pub prefix: PathBuf,
}

impl Trigger {
    pub fn new(
        event: notify::Event,
        prefix: &Path,
    ) -> anyhow::Result<Option<Self>> {
        let kind = match event.kind {
            notify::EventKind::Access(_) => {
                // Access is non-mutating, so not interesting to us.
                return Ok(None);
            }

            notify::EventKind::Any => "any",
            notify::EventKind::Create(_) => "create",
            notify::EventKind::Modify(_) => "modify",
            notify::EventKind::Remove(_) => "remove",
            notify::EventKind::Other => "other",
        };

        let prefix = prefix.canonicalize().with_context(|| {
            format!("Failed to canonicalize path `{}`", prefix.display())
        })?;

        Ok(Some(Self {
            kind,
            paths: event.paths,
            prefix,
        }))
    }
}

impl fmt::Display for Trigger {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let num_paths = self.paths.len();
        for (i, path) in self.paths.iter().enumerate() {
            // If we can't strip the prefix, just leave the path as-is.
            let path = path.strip_prefix(&self.prefix).unwrap_or(&path);

            write!(f, "{}", path.display())?;
            if i < num_paths - 1 {
                write!(f, ", ")?;
            }
        }

        Ok(())
    }
}
