use std::path::Path;

pub struct Notebook {
}

impl Notebook {
    /// make sure `path` is a git repo
    fn open(path: &Path) -> Self {}

    /// git init
    fn create(path: &Path) -> Self {}

    /// ls directory
    fn ls(&self, path: &Path) -> Vec<String> {}

    /// read file
    fn read(&self, path: &Path) -> String {}

    /// write file, git commit
    fn write(&self, path: &Path, text: &str) {}

    /// git pull, merge (TODO: handle conflicts?), git push
    fn synchronize(&self) {}

    /// determine incoming & outgoing changes
    fn sync_status(&self) -> SyncStatus {}

    /// git blame
    fn history(&self, path: &Path) -> History {}
}

pub struct SyncStatus {}

pub struct History {}
