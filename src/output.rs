use std::path::Path;
use types::*;

output!(/// Pass a library to the compiler as a `-l` flag
    fn link_lib, "rustc-link-lib", LibKind, <&Path>);

output!(/// Pass a library search path to the compiler as a `-L` flag
    fn link_search, "rustc-link-search", SearchKind, <&Path>);

output!(/// Pass a cfg flag to the compiler
    fn cfg, "rustc-cfg", &str);

output!(/// Specify a file or directory whose timestamp will trigger a rerun of the build script
    fn rerun, "rerun-if-changed", <&Path>);

output!(/// Emit a warning
    fn warning, "warning", &str);

output!(/// Pass metadata to dependent crates (see [cargo docs](http://doc.crates.io/build-script.html#the-links-manifest-key))
    fn meta, None, &str, &str);

