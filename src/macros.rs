macro_rules! message_display {
    ($enumname:ident) => {
        impl fmt::Display for $enumname {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}", self.get_message().unwrap())
            }
        }
    }
}

macro_rules! inputs {
    () => {};

    ($(#[$attr:meta])* fn $fnname:ident -> $fnret:ty, $varname:expr, |($kp:tt, $vp:tt)| $code:expr; $($rest:tt)*) => {
        $(#[$attr])*
        pub fn $fnname() -> $crate::Result<$fnret> {
            ::std::env::vars_os()
                .filter_map(|(k, $vp)| match k.into_string() {
                    Ok(ref mut s) if s.starts_with($varname) => {
                        let $kp = &s[$varname.len()..];
                        $code
                    }

                    _ => None
                })
                .collect()
        }

        inputs!($($rest)*);
    };

    ($(#[$attr:meta])* fn $fnname:ident -> $fnret:ty, $varname:expr, |$param:tt| $code:expr; $($rest:tt)*) => {
        $(#[$attr])*
        pub fn $fnname() -> $crate::Result<$fnret> {
            let $param = ::std::env::var($varname)?;
            $code
        }

        inputs!($($rest)*);
    };

    ($(#[$attr:meta])* fn $fnname:ident -> $fnret:ty, $varname:expr, parse $kind:ident; $($rest:tt)*) => {
        inputs! {
            $(#[$attr])*
            fn $fnname -> $fnret, $varname,
               |s| s.parse().map_err(|e| $crate::Error::from_kind($crate::ErrorKind::$kind(e)));
                    // FIXME weird error-chain incantation (is $kind necessary?)
            
            $($rest)*
        }
    };

    ($(#[$attr:meta])* fn $fnname:ident -> $fnret:ty, $varname:expr; $($rest:tt)*) => {
        inputs! {
            $(#[$attr])*
            fn $fnname -> $fnret, $varname, |x| ::std::result::Result::Ok(x);

            $($rest)*
        }
    };
}

macro_rules! outputs {
    () => {};

    (@inner [($($fndecl:tt)*) ($(($($gen:tt)*))*) ($($rest:tt)*)]
            () -> ($(, $n:ident : $t:ty)*)
                  ($($strings:expr),*)
                  ($($vars:tt)*)) => {
        $($fndecl)*<$($($gen)*),*>($($n: $t),*) {
            println!(concat!($($strings),*) $($vars)*);
        }

        outputs!($($rest)*);
    };

    (@inner [$fndecl:tt ($($gen:tt)*) $rest:tt]
            (<$typ:ident> $(,)*) -> ($($params:tt)*)
                                 ($($strings:tt)*)
                                 ($($vars:tt)*)) => {
        outputs!(@inner [$fndecl ($($gen)* ($typ: ::std::convert::AsRef<::std::ffi::OsStr> + ?::std::marker::Sized)) $rest]
                        () -> ($($params)*, n: &$typ)
                              ($($strings)*, "{}")
                              ($($vars)*, ::std::path::Path::new(n).display()));
    };
    (@inner $thru:tt
            ($typ:ty $(,)*) -> ($($params:tt)*)
                               ($($strings:tt)*)
                               ($($vars:tt)*)) => {
        outputs!(@inner $thru
                        () -> ($($params)*, n: $typ)
                              ($($strings)*, "{}")
                              ($($vars)*, n));
    };
    (@inner [$fndecl:tt ($($gen:tt)*) $rest:tt]
            (<$typ:ident>, $($types:tt)*) -> ($($params:tt)*)
                                             ($($strings:tt)*)
                                             ($($vars:tt)*)) => {
        outputs!(@inner [$fndecl ($($gen)* ($typ: ::std::convert::AsRef<::std::ffi::OsStr> + ?::std::marker::Sized)) $rest]
                        ($($types),*) -> ($($params)*, n: &$typ)
                                         ($($strings)*, "{}", "=")
                                         ($($vars)*, ::std::path::Path::new(n).display()));
    };
    (@inner $thru:tt
            ($typ:ty, $($types:tt)*) -> ($($params:tt)*)
                                        ($($strings:tt)*)
                                        ($($vars:tt)*)) => {
        outputs!(@inner $thru
                        ($($types)*) -> ($($params)*, n: $typ)
                                        ($($strings)*, "{}", "=")
                                        ($($vars)*, n));
    };

    ($(#[$attr:meta])* fn $fnname:ident($($types:tt)*) => None; $($rest:tt)*) => {
        outputs!(@inner [($(#[$attr])* pub fn $fnname) () ($($rest)*)]
                        ($($types)*,) -> ()
                                         ("cargo:")
                                         ());
    };

    ($(#[$attr:meta])* fn $fnname:ident($($types:tt)*) => $string:expr; $($rest:tt)*) => {
        outputs!(@inner [($(#[$attr])* pub fn $fnname) () ($($rest)*)]
                        ($($types)*,) -> ()
                                         ("cargo:", $string, "=")
                                         ());
    };
}

