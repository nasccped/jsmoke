use super::error::PathWrapperError;
use std::{path::PathBuf, str::FromStr};

/// Wrapper for [`PathBuf`] stuff.
#[derive(Debug, Clone)]
pub struct PathWrapper(PathBuf);

impl FromStr for PathWrapper {
    type Err = PathWrapperError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        PathBuf::from_str(s)
            .map(Self)
            .map_err(|_| PathWrapperError::InvalidPath(s.into()))
    }
}

#[cfg(test)]
mod test {
    use super::PathWrapper;
    use std::str::FromStr;

    // NOTE: the code bellow is intended to work based on the current project state (since it uses
    // real path and directories to test if it's valid or smtg).

    #[test]
    fn valids() {
        ["src", "src/cli"]
            .into_iter()
            .for_each(|pth| assert!(PathWrapper::from_str(pth).is_ok()));
    }
}
