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
use itertools::Itertools;

use super::{Entry, Error, File, FileSystemTrait, Result};

pub struct FileSystem {
    root_path: Utf8PathBuf,
}

impl FileSystem {
    pub fn new(root_path: impl AsRef<Utf8Path>) -> Self {
        Self {
            root_path: root_path.as_ref().to_path_buf(),
        }
    }

    pub fn root_path(&self) -> &Utf8Path {
        &self.root_path
    }
}

impl FileSystemTrait for FileSystem {
    fn read_file(&self, path: &Utf8Path) -> Result<Box<dyn File>> {
        let path = self.root_path.join(path);
        if !path.exists() {
            return Err(Error::NotExist);
        }

        let file = std::fs::File::open(path)?;
        Ok(Box::new(file))
    }

    fn read_dir(&self, path: &Utf8Path) -> Result<Vec<Entry>> {
        let path = self.root_path.join(path);
        if !path.exists() {
            return Err(Error::NotExist);
        }

        path.read_dir_utf8()?
            .map_ok(|entry| {
                let path = entry.path();
                // FIXME windows path shenanigans
                let path = path
                    .strip_prefix(&self.root_path)
                    .unwrap_or(path)
                    .to_path_buf();

                let metadata = std::fs::metadata(&path)?;

                Ok(Entry {
                    path,
                    is_file: metadata.is_file(),
                })
            })
            .flatten()
            .try_collect()
    }
}
