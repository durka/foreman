use std::ffi::OsStr;
use std::path::Path;
use types::*;

outputs! {
    /// Pass a library to the compiler as a `-l` flag
    fn link_lib(LibKind, <S>) => "rustc-link-lib";

    /// Pass a library search path to the compiler as a `-L` flag
    fn link_search(SearchKind, <S>) => "rustc-link-search";

    /// Pass a cfg flag to the compiler
    fn cfg(&str) => "rustc-cfg";

    /// Set a variable in the compiler's environment
    fn env_var(&str, &str) => "rustc-env";

    /// Specify a file or directory whose timestamp will trigger a rerun of the build script
    /// (note: to recursively track a directory, see `rerun_walk`)
    fn rerun(<S>) => "rerun-if-changed";

    /// Specify environment variables where new values will trigger a rerun of the build script
    fn rerun_env(&str) => "rerun-if-env-changed";

    /// Emit a warning
    fn warning(&str) => "warning";

    /// Pass metadata to dependent crates
    /// (see [cargo docs](http://doc.crates.io/build-script.html#the-links-manifest-key))
    fn meta(&str, &str) => None;
}

/// Recursively walk a directory, calling the provided callback for each file and directory, and if
/// it returns true, tell cargo to trigger a rerun of the build script. Hidden files are included,
/// but errors are ignored (so directories that you do not have permission to access will be
/// silently skipped).
pub fn rerun_walk<S: AsRef<OsStr> + ?Sized, CB: FnMut(&Path) -> bool>(p: &S, mut callback: CB) {
    use walkdir::WalkDir;

    for entry in WalkDir::new(p.as_ref()) {
        if let Ok(entry) = entry {
            if callback(entry.path()) {
                rerun(entry.path());
            }
        }
    }
}

