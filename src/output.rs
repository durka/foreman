use std::path::Path;
use types::*;

output!(link_lib, "rustc-link-lib", LibKind, &str);
output!(link_search, "rustc-link-search", SearchKind, &str);
output!(flag, "rustc-flags", |k: FlagKind, f: &str| format!("-{} {}", k, f));
output!(cfg, "rustc-cfg", &str);
output!(rerun_if_changed, "rerun-if-changed", |p: &Path| p.display());
output!(warning, "warning", &str);

pub fn meta(key: &str, val: &str) {
    println!("cargo:{}={}", key, val);
}

