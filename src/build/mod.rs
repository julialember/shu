pub mod build;
mod split;

pub use build::{
    Build, BuildError
};

use split::{
    SplitArgs, SplitError
};

impl From<SplitError> for BuildError<'_> {
    fn from(value: SplitError) -> Self {
        BuildError::SplitError(value) 
    }
}
