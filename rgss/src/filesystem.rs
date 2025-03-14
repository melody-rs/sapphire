// Copyright (C) 2024 Lily Lyons
//
// This file is part of sapphire.
//
// sapphire is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// sapphire is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with sapphire.  If not, see <http://www.gnu.org/licenses/>.

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

pub trait File: Read + Seek + Send + Sync {}
impl<T> File for T where T: Read + Seek + Send + Sync {}

// designed to be object safe.
// this is so we can load any number of filesystems at runtime
trait FileSystemTrait: Send + Sync {
    fn read_file(&self, path: &Utf8Path) -> Result<Box<dyn File>>;

    fn read_dir(&self, path: &Utf8Path) -> Result<Vec<Entry>>;
}

impl FileSystem {
    pub fn new(root_path: impl AsRef<Utf8Path>, archive_path: Option<&Utf8Path>) -> Result<Self> {
        let host = host::FileSystem::new(root_path.as_ref());

        let mut list = list::FileSystem::new();

        // TODO
        if let Some(archive_path) = archive_path {
            let archive_file = host.read_file(archive_path)?;
            let archive = archive::FileSystem::new(archive_file)?;
            list.push(Box::new(archive));
        }

        list.push(Box::new(host));

        let path_cache = path_cache::FileSystem::new(Box::new(list))?;

        Ok(Self { fs: path_cache })
    }

    pub fn read_file(&self, path: impl AsRef<Utf8Path>) -> Result<Box<dyn File>> {
        self.fs.read_file(path.as_ref())
    }
}
