fn main() {
    input! {
        n: usize,
    };
}

// from https://github.com/qryxip/competitive-programming-library/blob/master/crates/io/sourcefiles/input.rs
mod input {
    //! Provides `input!` macro.
    //!
    //! # Example
    //!
    //! ```no_run
    //! #[macro_use]
    //! extern crate input as _;
    //!
    //! fn main() {
    //!     // https://atcoder.jp/contests/abc166/tasks/abc166_b
    //!
    //!     input! {
    //!         n: usize,
    //!         ass: [[{ input::usize1 }]],
    //!     }
    //!
    //!     let _: usize = n;
    //!     let _: Vec<Vec<usize>> = ass;
    //! }
    //! ```

    use std::{
        cell::RefCell,
        fmt::Debug,
        io::{self, BufRead, Read},
        rc::Rc,
        str::{FromStr, SplitWhitespace},
    };

    #[macro_export]
    macro_rules! input {
        (from $scanner:ident; $($tt:tt)*) => {
            $crate::input_inner!(@scanner($scanner), @tts($($tt)*))
        };
        ($($tt:tt)*) => {
            let __scanner = $crate::input::DEFAULT_SCANNER.with(|__scanner| __scanner.clone());
            let mut __scanner_ref = __scanner.borrow_mut();
            if let $crate::input::Scanner::Uninited = *__scanner_ref {
                *__scanner_ref = $crate::input::Scanner::stdin_auto().unwrap();
            }
            $crate::input_inner!(@scanner(__scanner_ref), @tts($($tt)*));
            ::std::mem::drop(__scanner_ref);
            ::std::mem::drop(__scanner);
        };
    }

    #[macro_export]
    macro_rules! input_inner {
        (@scanner($scanner:ident), @tts()) => {};
        (@scanner($scanner:ident), @tts(mut $single_tt_pat:tt : $readable:tt)) => {
            let mut $single_tt_pat = $crate::read!(from $scanner { $readable });
        };
        (@scanner($scanner:ident), @tts($single_tt_pat:tt : $readable:tt)) => {
            let $single_tt_pat = $crate::read!(from $scanner { $readable });
        };
        (@scanner($scanner:ident), @tts(mut $single_tt_pat:tt : $readable:tt, $($rest:tt)*)) => {
            $crate::input_inner!(@scanner($scanner), @tts(mut $single_tt_pat: $readable));
            $crate::input_inner!(@scanner($scanner), @tts($($rest)*));
        };
        (@scanner($scanner:ident), @tts($single_tt_pat:tt : $readable:tt, $($rest:tt)*)) => {
            $crate::input_inner!(@scanner($scanner), @tts($single_tt_pat: $readable));
            $crate::input_inner!(@scanner($scanner), @tts($($rest)*));
        };
    }

    #[macro_export]
    macro_rules! read {
        (from $scanner:ident { [$tt:tt] }) => {
            $crate::read!(from $scanner { [$tt; $crate::read!(from $scanner { usize })] })
        };
        (from $scanner:ident  { [$tt:tt; $n:expr] }) => {
            (0..$n).map(|_| $crate::read!(from $scanner { $tt })).collect::<Vec<_>>()
        };
        (from $scanner:ident { ($($tt:tt),+) }) => {
            ($($crate::read!(from $scanner { $tt })),*)
        };
        (from $scanner:ident { { $f:expr } }) => {
            $crate::input::FnOnceExt::<_>::call_once_from_reader($f, &mut $scanner)
        };
        (from $scanner:ident { $ty:ty }) => {
            <$ty as $crate::input::Readable>::read_from_scanner(&mut $scanner)
        };
    }

    #[macro_export]
    macro_rules! readable {
    ($name:ident; |$scanner:ident| { $($body:tt)* }) => {
        $crate::readable!($name; |$scanner| -> () { $($body)* });
    };
    ($name:ident; |$scanner:ident| $expr:expr) => {
        $crate::readable!($name; |$scanner| -> () { $expr });
    };
    ($name:ident; |$scanner:ident| -> $output:ty { $($body:tt)* }) => {
        enum $name {}

        impl $crate::input::Readable for $name {
            type Output = $output;

            fn read_from_scanner(mut $scanner: &mut $crate::input::Scanner) -> $output {
                $($body)*
            }
        }
    };
}

    #[inline]
    pub fn usize1(n: usize) -> usize {
        n - 1
    }

    #[inline]
    pub fn bytes(s: String) -> Vec<u8> {
        s.into()
    }

    #[doc(hidden)]
    pub trait FnOnceExt<A> {
        type Output;
        fn call_once_from_reader(this: Self, scanner: &mut Scanner) -> Self::Output;
    }

    impl<A, O, F> FnOnceExt<A> for F
    where
        A: FromStr,
        A::Err: Debug,
        F: FnOnce(A) -> O,
    {
        type Output = O;

        #[inline]
        fn call_once_from_reader(this: Self, scanner: &mut Scanner) -> O {
            this(A::read_from_scanner(scanner))
        }
    }

    pub enum Scanner {
        Uninited,
        Once {
            words: SplitWhitespace<'static>,
        },
        Lines {
            rdr: Box<dyn BufRead>,
            words: SplitWhitespace<'static>,
        },
    }

    impl Scanner {
        pub fn stdin_auto() -> io::Result<Self> {
            if cfg!(debug_assertions) {
                Ok(Self::lines(Box::leak(Box::new(io::stdin())).lock()))
            } else {
                Self::once(io::stdin())
            }
        }

        pub fn once<R: Read>(mut rdr: R) -> io::Result<Self> {
            let mut buf = String::with_capacity(1024);
            rdr.read_to_string(&mut buf)?;
            let words = Box::leak(buf.into_boxed_str()).split_whitespace();
            Ok(Self::Once { words })
        }

        pub fn lines<R: BufRead + 'static>(rdr: R) -> Self {
            Self::Lines {
                rdr: Box::new(rdr),
                words: "".split_whitespace(),
            }
        }

        pub fn parse_next_unwrap<T: FromStr>(&mut self) -> T
        where
            T::Err: Debug,
        {
            match self {
                Self::Uninited => None,
                Self::Once { words } => words.next(),
                Self::Lines { rdr, words } => words.next().or_else(|| {
                    let mut line = "".to_owned();
                    rdr.read_line(&mut line).unwrap();
                    *words = Box::leak(line.into_boxed_str()).split_whitespace();
                    words.next()
                }),
            }
            .expect("reached EOF")
            .parse()
            .unwrap()
        }
    }

    thread_local! {
        pub static DEFAULT_SCANNER: Rc<RefCell<Scanner>> = Rc::new(RefCell::new(Scanner::Uninited));
    }

    pub trait Readable {
        type Output;

        fn read_from_scanner(scanner: &mut Scanner) -> Self::Output;
    }

    impl<T: FromStr> Readable for T
    where
        T::Err: Debug,
    {
        type Output = Self;

        fn read_from_scanner(scanner: &mut Scanner) -> Self {
            scanner.parse_next_unwrap()
        }
    }
}
