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

input!(manifest_dir -> PathBuf, "CARGO_MANIFEST_DIR", |s| Ok(PathBuf::from(s)));
input!(manifest_links -> String, "CARGO_MANIFEST_LINKS");
input!(features -> Vec<String>, "CARGO_FEATURE_",
       |(feat, _)| {
           Some(Ok(feat.to_lowercase()))
               // FIXME feature name collisions with _ and - (parse Cargo.toml to find out?) cf. rust-lang/cargo#3072
       });
input!(out_dir -> PathBuf, "OUT_DIR", |s| Ok(PathBuf::from(s)));
input!(target -> String, "TARGET"); // FIXME return a real type here?
input!(host -> String, "HOST"); // FIXME return a real type here?
input!(num_jobs -> u32, "NUM_JOBS", parse ParseInt);
input!(opt_level -> u32, "OPT_LEVEL", parse ParseInt);
input!(debug -> bool, "DEBUG", parse ParseBool);
input!(profile -> Profile, "PROFILE", parse ParseProfile);
input!(dep_metadata -> HashMap<String, HashMap<String, String>>, "DEP_",
       |(dep_key, value)| {
           match value.into_string() {
               Ok(value) => {
                   let mut dep_key = dep_key.splitn(2, '_');
                   Some(Ok(Three(dep_key.next().unwrap().to_lowercase(), dep_key.next().unwrap().to_lowercase(), value)))
                       // FIXME - converted to _ (parse Cargo.toml to find out?)
               }

               _ => None
           }
       });
input!(rustc -> String, "RUSTC");
input!(rustdoc -> String, "RUSTDOC");


