macro_rules! message_display {
    ($enumname:ident) => {
        impl fmt::Display for $enumname {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}", self.get_message().unwrap())
            }
        }
    }
}

macro_rules! input {
    ($(#[$attr:meta])* fn $fnname:ident -> $fnret:ty, $varname:expr,
     |($kp:tt, $vp:tt)| $code:expr) => {
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
    };

    ($(#[$attr:meta])* fn $fnname:ident -> $fnret:ty, $varname:expr,
     |$param:tt| $code:expr) => {
        $(#[$attr])*
        pub fn $fnname() -> $crate::Result<$fnret> {
            let $param = ::std::env::var($varname)?;
            $code
        }
    };

    ($(#[$attr:meta])* fn $fnname:ident -> $fnret:ty, $varname:expr,
     parse $kind:ident) => {
        input!($(#[$attr])* fn $fnname -> $fnret, $varname,
               |s| s.parse().map_err(|e| $crate::Error::from_kind($crate::ErrorKind::$kind(e))));
                    // FIXME weird error-chain incantation (is $kind necessary?)
    };

    ($(#[$attr:meta])* fn $fnname:ident -> $fnret:ty, $varname:expr) => {
        input!($(#[$attr])* fn $fnname -> $fnret, $varname,
               |x| ::std::result::Result::Ok(x));
    };
}

macro_rules! output {
    (@inner [$($fndecl:tt)*]
            () -> ($(, $n:ident : $t:ty)*)
                  ($($strings:expr),*)
                  ($($vars:tt)*)) => {
        $($fndecl)*($($n: $t),*) {
            println!(concat!($($strings),*) $($vars)*);
        }
    };

    (@inner $thru:tt
            (<$typ:ty> $(,)*) -> ($($params:tt)*)
                                 ($($strings:tt)*)
                                 ($($vars:tt)*)) => {
        output!(@inner $thru
                       () -> ($($params)*, n: $typ)
                             ($($strings)*, "{}")
                             ($($vars)*, n.display()));
    };
    (@inner $thru:tt
            ($typ:ty $(,)*) -> ($($params:tt)*)
                               ($($strings:tt)*)
                               ($($vars:tt)*)) => {
        output!(@inner $thru
                       () -> ($($params)*, n: $typ)
                             ($($strings)*, "{}")
                             ($($vars)*, n));
    };
    (@inner $thru:tt
            (<$typ:ty>, $($types:tt)*) -> ($($params:tt)*)
                                          ($($strings:tt)*)
                                          ($($vars:tt)*)) => {
        output!(@inner $thru
                       ($($types),*) -> ($($params)*, n: $typ)
                                        ($($strings)*, "{}", "=")
                                        ($($vars)*, n.display()));
    };
    (@inner $thru:tt
            ($typ:ty, $($types:tt)*) -> ($($params:tt)*)
                                        ($($strings:tt)*)
                                        ($($vars:tt)*)) => {
        output!(@inner $thru
                       ($($types)*) -> ($($params)*, n: $typ)
                                       ($($strings)*, "{}", "=")
                                       ($($vars)*, n));
    };

    ($(#[$attr:meta])* fn $fnname:ident, $string:expr,
     |$($n:ident : $t:ty),*| $code:expr) => {
        $(#[$attr])*
        pub fn $fnname($($n: $t),*) {
            println!(concat!("cargo:", $string, "={}"), $code);
        }
    };

    ($(#[$attr:meta])* fn $fnname:ident, None,
     $($types:tt)*) => {
        output!(@inner [$(#[$attr])* pub fn $fnname]
                       ($($types)*,) -> ()
                                        ("cargo:")
                                        ());
    };

    ($(#[$attr:meta])* fn $fnname:ident, $string:expr,
     $($types:tt)*) => {
        output!(@inner [$(#[$attr])* pub fn $fnname]
                       ($($types)*,) -> ()
                                        ("cargo:", $string, "=")
                                        ());
    };
}
