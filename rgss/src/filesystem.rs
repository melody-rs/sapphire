use camino::{Utf8Path, Utf8PathBuf};
use std::io::{Read, Seek};

// Heavily simplified implementation of Luminol's filesystem crate.
// Mainly adapted from https://github.com/Astrabit-ST/Luminol/tree/6998d6425c7eb624eae8d2d4fffbb468da27c02f/crates/filesystem/src
// We could possibly use Luminol's crate but that'd be overkill.
// We only need to read files and desensitize paths.
mod archive;
mod host;
mod list;
// TODO use generics to support mounting more filesystems
mod path_cache;

pub struct FileSystem {
    fs: path_cache::FileSystem,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("File or directory does not exist")]
    NotExist,
    #[error("IO Error {0}")]
    Io(#[from] std::io::Error),
    #[error("UTF-8 Error {0}")]
    Utf8(#[from] std::string::FromUtf8Error),
    #[error("Archive header is incorrect")]
    InvalidHeader,
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(PartialEq, Eq)]
struct Entry {
    path: Utf8PathBuf,
    is_file: bool,
}

pub trait File: Read + Seek + Send + Sync {
    fn file_len(&self) -> Option<u64>;
}

impl File for &mut dyn File {
    fn file_len(&self) -> Option<u64> {
        (**self).file_len()
    }
}

impl File for Box<dyn File> {
    fn file_len(&self) -> Option<u64> {
        (**self).file_len()
    }
}

// designed to be object safe.
// this is so we can load any number of filesystems at runtime
trait FileSystemTrait: Send + Sync {
    fn open_file(&self, path: &Utf8Path) -> Result<Box<dyn File>>;

    fn read_dir(&self, path: &Utf8Path) -> Result<Vec<Entry>>;
}

impl FileSystem {
    pub fn new(root_path: impl AsRef<Utf8Path>, archive_path: Option<&Utf8Path>) -> Result<Self> {
        let host = host::FileSystem::new(root_path.as_ref());

        let mut list = list::FileSystem::new();

        // TODO
        if let Some(archive_path) = archive_path {
            let archive_file = host.open_file(archive_path)?;
            let archive = archive::FileSystem::new(archive_file)?;
            list.push(Box::new(archive));
        }

        list.push(Box::new(host));

        let path_cache = path_cache::FileSystem::new(Box::new(list))?;

        Ok(Self { fs: path_cache })
    }

    pub fn open_file(&self, path: impl AsRef<Utf8Path>) -> Result<Box<dyn File>> {
        self.fs.open_file(path.as_ref())
    }
}
