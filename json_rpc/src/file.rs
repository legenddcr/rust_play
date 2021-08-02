use std::collections::HashMap;
use std::ffi::OsString;
use std::fmt;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::str;
use std::time::SystemTime;
#[cfg(target_family = "unix")]
use std::os::unix::fs::PermissionsExt;

// use serde::de::{self, Deserialize, Deserializer, Unexpected};
// use serde::ser::{Serialize, Serializer};

const UTF8_BOM: &str = "\u{feff}";

#[derive(Debug, Clone, Copy)]
pub enum CharacterEncoding {
    Utf8,
    Utf8WithBom,
}

impl CharacterEncoding {
    fn guess(s: &[u8]) -> Self {
        if s.starts_with(UTF8_BOM.as_bytes()) {
            CharacterEncoding::Utf8WithBom
        } else {
            CharacterEncoding::Utf8
        }
    }
}

/// Returns the modification timestamp for the file at a given path,
/// if present.
fn get_mod_time<P: AsRef<Path>>(path: P) -> Option<SystemTime> {
    File::open(path).and_then(|f| f.metadata()).and_then(|meta| meta.modified()).ok()
}

/// Returns the file permissions for the file at a given path on UNIXy systems,
/// if present.
#[cfg(target_family = "unix")]
fn get_permissions<P: AsRef<Path>>(path: P) -> Option<u32> {
    File::open(path).and_then(|f| f.metadata()).map(|meta| meta.permissions().mode()).ok()
}

fn try_decode(bytes: Vec<u8>, encoding: CharacterEncoding, path: &Path) -> Result<Rope, FileError> {
    match encoding {
        CharacterEncoding::Utf8 => Ok(Rope::from(
            str::from_utf8(&bytes).map_err(|_e| FileError::UnknownEncoding(path.to_owned()))?,
        )),
        CharacterEncoding::Utf8WithBom => {
            let s = String::from_utf8(bytes)
                .map_err(|_e| FileError::UnknownEncoding(path.to_owned()))?;
            Ok(Rope::from(&s[UTF8_BOM.len()..]))
        }
    }
}

#[derive(Debug)]
pub struct FileInfo {
    pub encoding: CharacterEncoding,
    pub path: PathBuf,
    pub mod_time: Option<SystemTime>,
    pub has_changed: bool,
    #[cfg(target_family = "unix")]
    pub permissions: Option<u32>,
}

/// BufferIds uniquely identify open buffers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BufferId(pub(crate) usize);

/// Tracks all state related to open files.
pub struct FileManager {
    open_files: HashMap<PathBuf, BufferId>,
    file_info: HashMap<BufferId, FileInfo>,
}

pub enum FileError {
    Io(io::Error, PathBuf),
    UnknownEncoding(PathBuf),
    HasChanged(PathBuf),
}

fn try_load_file<P>(path: P) -> Result<(Rope, FileInfo), FileError>
where
    P: AsRef<Path>,
{
    // TODO: support for non-utf8
    // it's arguable that the rope crate should have file loading functionality
    let mut f =
        File::open(path.as_ref()).map_err(|e| FileError::Io(e, path.as_ref().to_owned()))?;
    let mut bytes = Vec::new();
    f.read_to_end(&mut bytes).map_err(|e| FileError::Io(e, path.as_ref().to_owned()))?;

    let encoding = CharacterEncoding::guess(&bytes);
    let rope = try_decode(bytes, encoding, path.as_ref())?;
    let info = FileInfo {
        encoding,
        mod_time: get_mod_time(&path),
        #[cfg(target_family = "unix")]
        permissions: get_permissions(&path),
        path: path.as_ref().to_owned(),
        has_changed: false,
    };
    Ok((rope, info))
}

impl FileManager {
    pub fn new() -> Self {
        FileManager { open_files: HashMap::new(), file_info: HashMap::new() }
    }

    pub fn open(&mut self, path: &Path, id: BufferId) -> Result<Rope, FileError> {
        if !path.exists() {
            return Ok(Rope::from(""));
        }

        let (rope, info) = try_load_file(path)?;

        self.open_files.insert(path.to_owned(), id);
        // if self.file_info.insert(id, info).is_none() {
        //     #[cfg(feature = "notify")]
        //     self.watcher.watch(path, false, OPEN_FILE_EVENT_TOKEN);
        // }
        self.file_info.insert(id, info);
        Ok(rope)
    }

    pub fn close(&mut self, id: BufferId) {
        if let Some(info) = self.file_info.remove(&id) {
            self.open_files.remove(&info.path);
            // #[cfg(feature = "notify")]
            // self.watcher.unwatch(&info.path, OPEN_FILE_EVENT_TOKEN);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::counter::utils::Counter;

    #[test]
    fn test_create_file_mananger() {
        let path = "/test/test.txt";
        let file_manager = FileManager::new();

        let id_counter = Counter::default();
        let buffer_id = BufferId(id_counter.next());

        // let rope = match path.as_ref() {
        //     Some(p) => file_manager.open(p, buffer_id)?,
        //     None => Rope::from(""),
        // };
    }
}
