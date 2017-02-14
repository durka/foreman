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
    ($fnname:ident -> $fnret:ty, $varname:expr, |($kp:tt, $vp:tt)| $code:expr) => {
        pub fn $fnname() -> $crate::Result<$fnret> {
            ::std::env::vars_os()
                .filter_map(|(k, $vp)| match k.into_string() {
                    Ok(ref mut s) if s.starts_with($varname) => {
                        let $kp = s.split_off($varname.len());
                        $code
                    }

                    _ => None
                })
                .collect()
        }
    };

    ($fnname:ident -> $fnret:ty, $varname:expr, |$param:tt| $code:expr) => {
        pub fn $fnname() -> $crate::Result<$fnret> {
            let $param = ::std::env::var($varname)?;
            $code
        }
    };

    ($fnname:ident -> $fnret:ty, $varname:expr, parse $kind:ident) => {
        input!($fnname -> $fnret, $varname, |s| s.parse().map_err(|e| $crate::Error::from_kind($crate::ErrorKind::$kind(e)))); // FIXME weird error-chain incantation (is $kind necessary?)
    };

    ($fnname:ident -> $fnret:ty, $varname:expr) => {
        input!($fnname -> $fnret, $varname, |x| ::std::result::Result::Ok(x));
    };
}

macro_rules! output {
    (@inner [$fnname:ident] () -> ($(, $n:ident : $t:ty)*) ($($strings:expr),*)) => {
        pub fn $fnname($($n: $t),*) {
            println!(concat!($($strings),*), $($n),*);
        }
    };

    (@inner $thru:tt ($typ:ty $(, $types:ty)*) -> ($($params:tt)*) ($($strings:tt)*)) => {
        output!(@inner $thru ($($types),*) -> ($($params)*, n: $typ) ($($strings)*, "={}"));
    };

    ($fnname:ident, $string:expr, |$($n:ident : $t:ty),*| $code:expr) => {
        pub fn $fnname($($n: $t),*) {
            println!(concat!("cargo:", $string, "={}"), $code);
        }
    };

    ($fnname:ident, $string:expr, $($types:ty),*) => {
        output!(@inner [$fnname] ($($types),*) -> () ("cargo:", $string));
    };
}

