#[cfg(target_os = "windows")]
extern crate winapi;

/// An extension trait for `std::fs::Path` providing an `is_executable` method.
///
/// See the module documentation for examples.
pub trait IsExecutable {
    /// Returns `true` if there is a file at the given path and it is
    /// executable. Returns `false` otherwise.
    ///
    /// See the module documentation for details.
    fn is_executable(&self) -> bool;
}

#[cfg(unix)]
mod unix {
    use std::os::unix::fs::PermissionsExt;
    use std::path::Path;

    use super::IsExecutable;

    impl IsExecutable for Path {
        fn is_executable(&self) -> bool {
            let metadata = match self.metadata() {
                Ok(metadata) => metadata,
                Err(_) => return false,
            };
            let permissions = metadata.permissions();
            metadata.is_file() && permissions.mode() & 0o111 != 0
        }
    }
}

#[cfg(target_os = "windows")]
mod windows {
    use std::os::windows::ffi::OsStrExt;
    use std::path::Path;

    use winapi::ctypes::{c_ulong, wchar_t};
    use winapi::um::winbase::GetBinaryTypeW;

    use super::IsExecutable;

    impl IsExecutable for Path {
        fn is_executable(&self) -> bool {
            // Check using file extension
            if let Some(pathext) = std::env::var_os("PATHEXT") {
                if let Some(extension) = self.extension() {
                    // Restructure pathext as Vec<String>
                    // https://github.com/nushell/nushell/blob/93e8f6c05e1e1187d5b674d6b633deb839c84899/crates/nu-cli/src/completion/command.rs#L64-L74
                    let pathext = pathext
                        .to_string_lossy()
                        .split(';')
                        // Filter out empty tokens and ';' at the end
                        .filter(|f| f.len() > 1)
                        // Cut off the leading '.' character
                        .map(|ext| ext[1..].to_string())
                        .collect::<Vec<_>>();
                    let extension = extension.to_string_lossy();

                    return pathext
                        .iter()
                        .any(|ext| extension.eq_ignore_ascii_case(ext));
                }
            }

            // Check using file properties
            // This code is only reached if there is no file extension or retrieving PATHEXT fails
            let windows_string = self
                .as_os_str()
                .encode_wide()
                .chain(Some(0))
                .collect::<Vec<wchar_t>>();
            let windows_string_ptr = windows_string.as_ptr();

            let mut binary_type: c_ulong = 42;
            let binary_type_ptr = &mut binary_type as *mut c_ulong;

            let ret = unsafe { GetBinaryTypeW(windows_string_ptr, binary_type_ptr) };
            if binary_type_ptr.is_null() {
                return false;
            }
            if ret != 0 {
                let binary_type = unsafe { *binary_type_ptr };
                match binary_type {
                    0   // A 32-bit Windows-based application
                    | 1 // An MS-DOS-based application
                    | 2 // A 16-bit Windows-based application
                    | 3 // A PIF file that executes an MS-DOS-based application
                    | 4 // A POSIX – based application
                    | 5 // A 16-bit OS/2-based application
                    | 6 // A 64-bit Windows-based application
                    => return true,
                    _ => (),
                }
            }

            false
        }
    }
}
