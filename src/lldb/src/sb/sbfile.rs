use super::*;

cpp_class!(pub unsafe struct SBFile as "SBFile");

unsafe impl Send for SBFile {}

impl SBFile {
    pub fn new() -> SBFile {
        cpp!(unsafe [] -> SBFile as "SBFile" { return SBFile(); })
    }
    #[cfg(unix)]
    pub fn from(obj: impl std::os::unix::io::IntoRawFd, write: bool) -> SBFile {
        SBFile::from_impl(obj.into_raw_fd(), write)
    }
    #[cfg(windows)]
    pub fn from(obj: impl std::os::windows::io::IntoRawHandle, write: bool) -> SBFile {
        let handle = obj.into_raw_handle() as libc::intptr_t;
        let flags = if write { 0 } else { libc::O_RDONLY };
        // Windows file descriptors belong to a CRT instance. Allocate this one
        // inside liblldb, which may use a different CRT from the adapter.
        let fd = cpp!(unsafe [handle as "intptr_t", flags as "int"] -> c_int as "int" {
            #ifdef _WIN32
            return SBFile::OpenFdFromHandle(handle, flags);
            #else
            return -1; // This binding is only called by the Windows implementation.
            #endif
        });
        SBFile::from_impl(fd, write)
    }
    fn from_impl(fd: c_int, write: bool) -> SBFile {
        cpp!(unsafe [fd as "int", write as "bool"] -> SBFile as "SBFile" {
            return SBFile(fd, write ? "w" : "r", true);
        })
    }
    pub fn close(&self) -> SBError {
        cpp!(unsafe [self as "SBFile*"] -> SBError as "SBError" {
            return self->Close();
        })
    }
}

impl IsValid for SBFile {
    fn is_valid(&self) -> bool {
        cpp!(unsafe [self as "SBFile*"] -> bool as "bool" {
            return self->IsValid();
        })
    }
}

#[cfg(all(test, windows))]
#[test]
fn windows_file_handle_transfer() {
    let path = std::env::temp_dir().join(format!("codelldb-sbfile-{}.txt", std::process::id()));
    let file = std::fs::OpenOptions::new().write(true).create_new(true).open(&path).unwrap();
    let mut output = SBFile::from(file, true);
    let data = b"CodeLLDB stream transfer";
    let buffer = data.as_ptr();
    let mut size = data.len();
    let result = cpp!(unsafe [mut output as "SBFile", buffer as "const uint8_t*", mut size as "size_t"]
        -> SBError as "SBError" {
        return output.Write(buffer, size, &size);
    });
    assert!(result.is_success(), "{result:?}");
    assert_eq!(size, data.len());
    assert!(output.close().is_success());
    assert_eq!(std::fs::read(&path).unwrap(), data);

    let mut input = SBFile::from(std::fs::File::open(&path).unwrap(), false);
    let mut received = [0u8; 64];
    let buffer = received.as_mut_ptr();
    let mut size = received.len();
    let result = cpp!(unsafe [mut input as "SBFile", buffer as "uint8_t*", mut size as "size_t"]
        -> SBError as "SBError" {
        return input.Read(buffer, size, &size);
    });
    assert!(result.is_success(), "{result:?}");
    assert_eq!(&received[..size], data);
    assert!(input.close().is_success());
    std::fs::remove_file(path).unwrap();
}

