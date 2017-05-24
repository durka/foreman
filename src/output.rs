use std::path::Path;
use types::*;

output!(/// Pass a library to the compiler as a `-l` flag
    fn link_lib, "rustc-link-lib", LibKind, <&Path>);

output!(/// Pass a library search path to the compiler as a `-L` flag
    fn link_search, "rustc-link-search", SearchKind, <&Path>);

output!(/// Pass a cfg flag to the compiler
    fn cfg, "rustc-cfg", &str);

output!(/// Set a variable in the compiler's environment
    fn env_var, "rustc-env", &str, &str);

output!(/// Specify a file or directory whose timestamp will trigger a rerun of the build script
        /// (note: to recursively track a directory, see `rerun_walk`)
    fn rerun, "rerun-if-changed", <&Path>);

/// Recursively walk a directory, calling the provided callback for each file and directory, and if
/// it returns true, tell cargo to trigger a rerun of the build script. Hidden files are included,
/// but errors are ignored (so directories that you do not have permission to access will be
/// silently skipped).
pub fn rerun_walk<CB: FnMut(&Path) -> bool>(p: &Path, mut callback: CB) {
    use walkdir::WalkDir;

    for entry in WalkDir::new(p) {
        if let Ok(entry) = entry {
            if callback(entry.path()) {
                rerun(entry.path());
            }
        }
    }
}

output!(/// Emit a warning
    fn warning, "warning", &str);

output!(/// Pass metadata to dependent crates
        /// (see [cargo docs](http://doc.crates.io/build-script.html#the-links-manifest-key))
    fn meta, None, &str, &str);
