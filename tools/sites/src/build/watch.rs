use std::{
    fmt,
    path::{Path, PathBuf},
};

use anyhow::Context as _;
use notify::{immediate_watcher, RecommendedWatcher, Watcher as _};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver};

pub struct Watcher {
    /// Keeping this around so it won't be dropped
    _watcher: RecommendedWatcher,

    pub rx: UnboundedReceiver<notify::Result<notify::Event>>,
}

impl Watcher {
    pub fn new(path: &Path) -> anyhow::Result<Self> {
        let (tx, rx) = unbounded_channel();

        let mut watcher: RecommendedWatcher =
            immediate_watcher(move |event| {
                // The function returns an error, if the received has been
                // closed. This shouldn't happen unless there's a bug, in which
                // case crashing this thread probably isn't the worst idea.
                tx.send(event).unwrap()
            })?;
        watcher.watch(path, notify::RecursiveMode::Recursive)?;

        Ok(Self {
            _watcher: watcher,
            rx,
        })
    }
}

#[derive(Debug)]
pub struct Trigger {
    pub kind: &'static str,
    pub paths: Vec<PathBuf>,
    pub prefix: PathBuf,
}

impl Trigger {
    pub fn new(
        event: notify::Result<notify::Event>,
        prefix: &Path,
    ) -> anyhow::Result<Option<Self>> {
        let event = event?;

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
        write!(f, "{} ", self.kind)?;

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
