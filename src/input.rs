use std::iter;
use std::collections::HashMap;
use std::path::PathBuf;
use types::*;

struct Three(String, String, String);

impl iter::FromIterator<Three> for HashMap<String, HashMap<String, String>> {
    fn from_iter<I: IntoIterator<Item=Three>>(iter: I) -> Self {
        let mut map = HashMap::new();
        for Three(a, b, c) in iter {
            map.entry(a).or_insert_with(HashMap::new).insert(b, c);
        }
        map
    }
}

inputs! {
    /// Get the directory contains Cargo.toml
    fn manifest_dir -> PathBuf, "CARGO_MANIFEST_DIR", |s| Ok(PathBuf::from(s));

    /// Get the value of the `links` key in Cargo.toml (if present)
    fn manifest_links -> String, "CARGO_MANIFEST_LINKS";

    /// Get the list of activated features (if any)
    fn features -> Vec<String>, "CARGO_FEATURE_",
       |(feat, _)| {
           Some(Ok(feat.to_lowercase()))
               // FIXME feature name collisions with _ and - (parse Cargo.toml to find out?)
               // cf. rust-lang/cargo#3072
       };

    /// Get the output directory (write generated files here)
    fn out_dir -> PathBuf, "OUT_DIR", |s| Ok(PathBuf::from(s));

    /// Get the target triple of the current compilation
    fn target -> String, "TARGET"; // FIXME return a real type here?

    /// Get the host triple of the current compiler
    fn host -> String, "HOST"; // FIXME return a real type here?

    /// Get the top-level parallelism number (e.g. for passing to `make -jN`)
    fn num_jobs -> u32, "NUM_JOBS", parse ParseInt;

    /// Get the optimization level of the current compilation
    fn opt_level -> u32, "OPT_LEVEL", parse ParseInt;

    /// Get the debug mode of the current compilation
    fn debug -> bool, "DEBUG", parse ParseBool;

    /// Get the compilation profile
    fn profile -> Profile, "PROFILE", parse ParseProfile;

    /// Get a list of all the metadata passed along from dependency build scripts
    /// (see [cargo docs](http://doc.crates.io/build-script.html#the-links-manifest-key))
    fn dep_metadata -> HashMap<String, HashMap<String, String>>, "DEP_",
       |(dep_key, value)| {
           match value.into_string() {
               Ok(value) => {
                   let mut dep_key = dep_key.splitn(2, '_');
                   Some(Ok(Three(
                               dep_key.next().unwrap().to_lowercase(),
                               dep_key.next().unwrap().to_lowercase(),
                               value)))
                       // FIXME - converted to _ (parse Cargo.toml to find out?)
               }

               _ => None
           }
       };

    /// Get the path to the compiler that Cargo is using
    fn rustc -> PathBuf, "RUSTC", |s| Ok(PathBuf::from(s));

    /// Get the path to the documentation generator that Cargo is using
    fn rustdoc -> PathBuf, "RUSTDOC", |s| Ok(PathBuf::from(s));
}

