use strum::EnumMessage;
use std::fmt;

/// A compilation profile (see [cargo docs](http://doc.crates.io/environment-variables.html#environment-variables-cargo-sets-for-build-scripts))
#[derive(PartialEq, Eq, Debug, EnumString)]
pub enum Profile {
    /// Not release mode
    #[strum(serialize="debug")]
    Debug,

    /// Release mode
    #[strum(serialize="release")]
    Release,
}

/// Library kind for [`rustc-link-lib`](http://doc.crates.io/build-script.html#outputs-of-the-build-script)
#[derive(PartialEq, Eq, Debug, EnumMessage)]
pub enum LibKind {
    /// Static library
    #[strum(message="static")]
    Static,

    /// Dynamic library
    #[strum(message="dylib")]
    Dylib,

    /// Framework (macOS)
    #[strum(message="framework")]
    Framework,
}

/// Search strategy for [`rustc-link-search`](http://doc.crates.io/build-script.html#outputs-of-the-build-script)
#[derive(PartialEq, Eq, Debug, EnumMessage)]
pub enum SearchKind {
    /// Search for a dependency
    #[strum(message="dependency")]
    Dependency,

    /// Search for a crate
    #[strum(message="crate")]
    Crate,

    /// Search for a native library
    #[strum(message="native")]
    Native,

    /// Search for a framework (macOS)
    #[strum(message="framework")]
    Framework,

    /// Search everywhere
    #[strum(message="all")]
    All,
}

message_display!(LibKind);
message_display!(SearchKind);
