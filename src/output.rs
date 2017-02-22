use std::path::Path;
use types::*;

output!(link_lib, "rustc-link-lib", LibKind, <&Path>);
output!(link_search, "rustc-link-search", SearchKind, <&Path>);
output!(cfg, "rustc-cfg", &str);
output!(rerun_if_changed, "rerun-if-changed", <&Path>);
output!(warning, "warning", &str);
output!(meta, None, &str, &str);

