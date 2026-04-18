//! Minimal rustutils for portable virtmgr (host/port).

pub mod inherited_fd {
    use anyhow::Result;
    use std::os::raw::c_int;

    /// Initialize inherited FD table (no-op on host).
    pub unsafe fn init_once() -> Result<()> {
        Ok(())
    }

    /// Take ownership of a FD passed from the parent (CRT `int` on Windows MinGW).
    pub fn take_fd_ownership(fd: c_int) -> Result<c_int, anyhow::Error> {
        Ok(fd)
    }
}

pub mod system_properties {
    use anyhow::Result;
    use std::env;

    fn normalize_key(key: &str) -> String {
        key.chars()
            .map(|ch| match ch {
                'a'..='z' => ch.to_ascii_uppercase(),
                'A'..='Z' | '0'..='9' | '_' => ch,
                _ => '_',
            })
            .collect()
    }

    pub fn read(key: &str) -> Result<Option<String>> {
        let normalized = normalize_key(key);
        for env_key in [
            format!("ANDROID_PROP_{normalized}"),
            format!("VIRTMGR_PROP_{normalized}"),
        ] {
            if let Ok(value) = env::var(&env_key) {
                return Ok(Some(value));
            }
        }
        Ok(None)
    }
}
