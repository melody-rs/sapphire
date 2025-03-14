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
use std::collections::HashMap;

use super::{Error, FileSystemTrait, Result};

pub struct FileSystem {
    fs: Box<dyn FileSystemTrait>,
    // desensitized path to original path
    // TODO replace with camino? maybe?
    cache: HashMap<Utf8PathBuf, Utf8PathBuf>,
}

impl FileSystem {
    pub fn new(fs: Box<dyn FileSystemTrait>) -> Result<Self> {
        let mut this = FileSystem {
            fs,
            cache: HashMap::with_capacity(1000),
        };
        this.regen_cache()?;

        Ok(this)
    }

    pub fn regen_cache(&mut self) -> Result<()> {
        fn read_dir_recursive(
            fs: &dyn FileSystemTrait,
            path: &Utf8Path,
            mut f: impl FnMut(&Utf8Path),
        ) -> Result<()> {
            fn internal(
                fs: &dyn FileSystemTrait,
                path: &Utf8Path,
                f: &mut impl FnMut(&Utf8Path),
            ) -> Result<()> {
                for entry in fs.read_dir(path)? {
                    f(&entry.path);
                    if !entry.is_file {
                        internal(fs, &entry.path, f)?;
                    }
                }
                Ok(())
            }
            internal(fs, path, &mut f)
        }

        self.cache.clear();
        read_dir_recursive(&*self.fs, Utf8Path::new(""), |path| {
            let mut lowercase = to_lowercase(path);
            lowercase.set_extension("");

            self.cache.insert(lowercase, path.to_path_buf());
        })?;
        Ok(())
    }

    pub fn desensitize(&self, path: impl AsRef<camino::Utf8Path>) -> Option<&camino::Utf8Path> {
        let mut path = to_lowercase(path);
        path.set_extension("");
        self.cache.get(&path).map(Utf8PathBuf::as_path)
    }
}

fn to_lowercase(p: impl AsRef<Utf8Path>) -> Utf8PathBuf {
    p.as_ref().as_str().to_lowercase().into()
}

impl FileSystemTrait for FileSystem {
    fn read_file(&self, path: &camino::Utf8Path) -> Result<Box<dyn super::File>> {
        let path = self.desensitize(path).ok_or(Error::NotExist)?;
        self.fs.read_file(path)
    }

    fn read_dir(&self, path: &camino::Utf8Path) -> Result<Vec<super::Entry>> {
        let path = self.desensitize(path).ok_or(Error::NotExist)?;
        self.fs.read_dir(path)
    }
}
