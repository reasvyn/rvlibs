use core::fmt;

/// Semantic version information for this build of rvlibs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Version {
    pub major: u16,
    pub minor: u16,
    pub patch: u16,
    pub pre: &'static str,
}

impl Version {
    pub const fn new(major: u16, minor: u16, patch: u16, pre: &'static str) -> Self {
        Self { major, minor, patch, pre }
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)?;
        if !self.pre.is_empty() {
            write!(f, "-{}", self.pre)?;
        }
        Ok(())
    }
}

/// The current rvlibs version, sourced from `Cargo.toml` at build time.
pub fn current() -> Version {
    Version {
        major: env!("CARGO_PKG_VERSION_MAJOR").parse().unwrap(),
        minor: env!("CARGO_PKG_VERSION_MINOR").parse().unwrap(),
        patch: env!("CARGO_PKG_VERSION_PATCH").parse().unwrap(),
        pre: "",
    }
}
