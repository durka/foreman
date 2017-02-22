use strum::EnumMessage;
use std::fmt;

#[derive(PartialEq, Eq, Debug, EnumString)]
pub enum Profile {
    #[strum(serialize="debug")]
    Debug,

    #[strum(serialize="release")]
    Release
}

#[derive(PartialEq, Eq, Debug, EnumMessage)]
pub enum LibKind {
    #[strum(message="static")]
    Static,

    #[strum(message="dylib")]
    Dylib,

    #[strum(message="framework")]
    Framework
}

#[derive(PartialEq, Eq, Debug, EnumMessage)]
pub enum SearchKind {
    #[strum(message="dependency")]
    Dependency,

    #[strum(message="crate")]
    Crate,

    #[strum(message="native")]
    Native,

    #[strum(message="framework")]
    Framework,

    #[strum(message="all")]
    All
}

message_display!(LibKind);
message_display!(SearchKind);

