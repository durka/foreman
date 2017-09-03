#[macro_use] extern crate assert_matches;
extern crate foreman;
use foreman::*;
use std::env;
use std::path::Path;

fn main() {
    assert_matches!(manifest_dir(),   Ok(ref s) if s == &env::current_dir().unwrap());
    assert_matches!(manifest_links(), Ok(ref s) if s == "whatever");
    assert_matches!(features(),       Ok(ref s) if s == &["feat"]);
    assert_matches!(out_dir(),        Ok(ref s) if s.strip_prefix(&env::current_dir().unwrap()).is_ok());
    assert!(        target()          .is_ok());
    assert!(        host()            .is_ok());
    assert_matches!(num_jobs(),       Ok(n) if n > 0);
    assert_matches!(opt_level(),      Ok(n) if n == 0);
    assert_matches!(debug(),          Ok(b) if b);
    assert_matches!(profile(),        Ok(Profile::Debug));
    assert_matches!(dep_metadata(),   Ok(ref m) if m.get("foo").and_then(|m| m.get("meta")) == Some(&"data".into()));
    assert!(        rustc()           .is_ok());
    assert!(        rustdoc()         .is_ok());

    link_lib(LibKind::Dylib, "whatever");
    link_search(SearchKind::All, Path::new("wherever"));
    cfg("whoever");
    env_var("FOO", "foo");
    rerun("whichever");
    rerun_walk("src", |_| true);
    rerun_env("FOO");
    warning("success");
}

