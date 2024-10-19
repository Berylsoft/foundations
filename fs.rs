use alloc::{vec::Vec, borrow::ToOwned as _};
use std::{fs, io, path::{Path, PathBuf}};

pub fn normalize(path: &Path) -> PathBuf {
    use std::path::Component;
    let mut components = path.components().peekable();
    let mut ret = if let Some(c @ Component::Prefix(..)) = components.peek() {
        let buf = PathBuf::from(c.as_os_str());
        let _ = components.next();
        buf
    } else {
        PathBuf::new()
    };

    for component in components {
        match component {
            Component::Prefix(..) => unreachable!(),
            Component::RootDir => {
                ret.push(component.as_os_str());
            }
            Component::CurDir => {}
            Component::ParentDir => {
                let _ = ret.pop();
            }
            Component::Normal(c) => {
                ret.push(c);
            }
        }
    }

    ret
}

// TODO Iterator version (with no alloc?)

pub fn iter_path_core(
    current_path: &Path,
    depth: Option<u64>,
    currect_depth: u64,
    list: &mut Vec<(PathBuf, bool)>,
    normalized: bool,
) -> io::Result<()> {
    for entry in fs::read_dir(current_path)? {
        let entry = entry?;
        let path = if normalized {
            normalize(&entry.path())
        } else {
            entry.path()
        };
        let is_dir = entry.file_type()?.is_dir();
        if let Some(depth) = depth {
            if currect_depth >= depth {
                continue;
            }
        }
        #[cfg(windows)]
        if entry.file_name() == "System Volume Information" || entry.file_name() == "$RECYCLE.BIN" {
            continue;
        }
        if is_dir {
            iter_path_core(&path, depth, currect_depth + 1, list, normalized)?;
        }
        list.push((path, is_dir));
    }
    Ok(())
}

pub fn iter_path(path: &Path, depth: Option<u64>, sort: bool, normalized: bool) -> io::Result<Vec<(PathBuf, bool)>> {
    let mut list = Vec::new();
    let is_dir = fs::metadata(path)?.file_type().is_dir();
    if is_dir {
        iter_path_core(path, depth, 0, &mut list, normalized)?;
    } else {
        list.push((path.to_owned(), is_dir));
    }
    if sort {
        list.sort_by(|a, b| a.0.cmp(&b.0));
    }
    Ok(list)
}
