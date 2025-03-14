use super::{Error, FileSystemTrait, Result};

#[derive(Default)]
pub struct FileSystem {
    filesystems: Vec<Box<dyn FileSystemTrait>>,
}

impl FileSystem {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, fs: Box<dyn FileSystemTrait>) {
        self.filesystems.push(fs)
    }
}

impl FileSystemTrait for FileSystem {
    fn read_file(&self, path: &camino::Utf8Path) -> Result<Box<dyn super::File>> {
        for fs in self.filesystems.iter() {
            let result = fs.read_file(path);
            match result {
                Ok(f) => return Ok(f),
                Err(Error::NotExist) => continue,
                Err(e) => return Err(e),
            }
        }
        Err(Error::NotExist)
    }

    fn read_dir(&self, path: &camino::Utf8Path) -> Result<Vec<super::Entry>> {
        let mut entries = Vec::new();
        let mut total_not_exist = 0;

        for fs in self.filesystems.iter() {
            let result = fs.read_dir(path);
            match result {
                Ok(f) => entries.extend(f),
                Err(Error::NotExist) => total_not_exist += 1,
                Err(e) => return Err(e),
            }
        }

        if total_not_exist == self.filesystems.len() {
            return Err(Error::NotExist);
        }

        entries.dedup();

        Ok(entries)
    }
}
