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
use std::collections::{HashMap, HashSet};

use super::{Entry, Error, File, FileSystemTrait, Result};

pub struct FileSystem {
    files: Files,
    directories: Directories,
    archive: parking_lot::Mutex<Box<dyn File>>,
}

type Files = HashMap<Utf8PathBuf, ArchiveEntry>;
type Directories = HashMap<Utf8PathBuf, HashSet<Utf8PathBuf>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct ArchiveEntry {
    offset: u64,
    size: u64,
    start_magic: u32,
}

const MAGIC: u32 = 0xDEADCAFE;
const HEADER: &[u8] = b"RGSSAD\0";

impl FileSystem {
    pub fn new(mut file: Box<dyn File>) -> Result<Self> {
        let version = read_header(&mut file)?;

        let (files, directories) = match version {
            1 | 2 => read_rmxp(&mut file),
            3 => read_vxa(&mut file),
            _ => Err(Error::InvalidHeader),
        }?;

        Ok(FileSystem {
            files,
            directories,
            archive: parking_lot::Mutex::new(file),
        })
    }
}

impl FileSystemTrait for FileSystem {
    fn read_file(&self, path: &camino::Utf8Path) -> Result<Box<dyn File>> {
        let entry = self.files.get(path).ok_or(Error::NotExist)?;
        let mut buf = vec![0; entry.size as usize];

        {
            let mut archive = self.archive.lock();
            archive.seek(std::io::SeekFrom::Start(entry.offset))?;
            archive.read_exact(&mut buf)?;
        }

        let mut magic = entry.start_magic;
        let mut j = 0;
        for byte in buf.iter_mut() {
            if j == 4 {
                j = 0;
                magic = magic.wrapping_mul(7).wrapping_add(3);
            }

            *byte ^= magic.to_le_bytes()[j];

            j += 1;
        }

        let cursor = std::io::Cursor::new(buf);
        Ok(Box::new(cursor))
    }

    fn read_dir(&self, path: &camino::Utf8Path) -> Result<Vec<super::Entry>> {
        let directory = self.directories.get(path).ok_or(Error::NotExist)?;
        directory
            .iter()
            .map(|entry| {
                let path = path.join(entry);
                let is_file = self.files.contains_key(&path);

                Ok(Entry { path, is_file })
            })
            .try_collect()
    }
}

fn read_rmxp(mut file: &mut dyn File) -> Result<(Files, Directories)> {
    let mut files = Files::with_capacity(1000);
    let mut directories = Directories::with_capacity(4);

    let mut magic = MAGIC;

    while let Ok(name_len) = read_u32_xor(&mut file, advance_magic(&mut magic)) {
        let mut name = vec![0; name_len as usize];
        file.read_exact(&mut name).unwrap();
        for byte in name.iter_mut() {
            let char = *byte ^ advance_magic(&mut magic) as u8;
            if char == b'\\' {
                *byte = b'/';
            } else {
                *byte = char;
            }
        }
        let name = Utf8PathBuf::from(String::from_utf8(name)?);

        process_path(&mut directories, &name);

        let entry_len = read_u32_xor(&mut file, advance_magic(&mut magic))?;

        let entry = ArchiveEntry {
            size: entry_len as u64,
            offset: file.stream_position()?,
            start_magic: magic,
        };

        files.insert(name, entry);

        file.seek(std::io::SeekFrom::Start(entry.offset + entry.size))?;
    }

    Ok((files, directories))
}

fn read_vxa(mut file: &mut dyn File) -> Result<(Files, Directories)> {
    let mut files = Files::with_capacity(1000);
    let mut directories = Directories::with_capacity(4);

    let mut u32_buf = [0; 4];
    file.read_exact(&mut u32_buf)?;

    let base_magic = u32::from_le_bytes(u32_buf);
    let base_magic = (base_magic * 9) + 3;

    while let Ok(offset) = read_u32_xor(&mut file, base_magic) {
        if offset == 0 {
            break;
        }

        let entry_len = read_u32_xor(&mut file, base_magic)?;
        let magic = read_u32_xor(&mut file, base_magic)?;
        let name_len = read_u32_xor(&mut file, base_magic)?;

        let mut name = vec![0; name_len as usize];
        file.read_exact(&mut name)?;
        for (i, byte) in name.iter_mut().enumerate() {
            let char = *byte ^ (base_magic >> (8 * (i % 4))) as u8;
            if char == b'\\' {
                *byte = b'/';
            } else {
                *byte = char;
            }
        }
        let name = Utf8PathBuf::from(String::from_utf8(name)?);

        process_path(&mut directories, &name);

        let entry = ArchiveEntry {
            size: entry_len as u64,
            offset: offset as u64,
            start_magic: magic,
        };
        files.insert(name, entry);
    }

    Ok((files, directories))
}

fn process_path(directories: &mut Directories, path: &Utf8Path) {
    for (a, b) in path.ancestors().tuple_windows() {
        directories
            .entry(b.to_path_buf())
            .or_default()
            .insert(a.strip_prefix(b).unwrap_or(a).to_path_buf());
    }
}

fn read_u32(file: &mut impl File) -> Result<u32> {
    let mut buffer = [0; 4];
    file.read_exact(&mut buffer)?;
    Ok(u32::from_le_bytes(buffer))
}

fn read_u32_xor(file: &mut impl File, key: u32) -> Result<u32> {
    let result = read_u32(file)?;
    Ok(result ^ key)
}

fn advance_magic(magic: &mut u32) -> u32 {
    let old = *magic;

    *magic = magic.wrapping_mul(7).wrapping_add(3);

    old
}

fn read_header(file: &mut impl File) -> Result<u8> {
    let mut header_buf = [0; 8];

    file.read_exact(&mut header_buf)?;

    if !header_buf.starts_with(HEADER) {
        return Err(Error::InvalidHeader);
    }

    Ok(header_buf[7])
}
