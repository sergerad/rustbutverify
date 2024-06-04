use std::fs::DirEntry;

use crate::prelude::*;

impl TryFrom<NewType<&DirEntry>> for String {
    type Error = Error;
    fn try_from(val: NewType<&DirEntry>) -> Result<String> {
        val.0
            .path()
            .to_str()
            .map(String::from)
            .ok_or_else(|| Error::Generic("Invalid path".to_string()))
    }
}
