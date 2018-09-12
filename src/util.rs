// Copyright (c) 2018 libdeadmock developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `libdeadmock` utilities
use failure::Error;
use std::fs::{self, DirEntry};
use std::path::Path;

pub fn visit_dirs<F>(dir: &Path, cb: &mut F) -> Result<(), Error>
where
    F: FnMut(&DirEntry) -> Result<(), Error>,
{
    if fs::metadata(dir)?.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            if fs::metadata(entry.path())?.is_dir() {
                visit_dirs(&entry.path(), cb)?;
            } else {
                cb(&entry)?;
            }
        }
    }
    Ok(())
}