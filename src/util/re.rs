use regex::{Captures, Regex};
use std::convert::Infallible;
use std::error;
use std::str::FromStr;

/// Converts a string literal into a Regex, caching the value in a static variable for reuse.
#[macro_export]
macro_rules! regex {
    ($re:literal $(,)?) => {{
        static RE: once_cell::sync::OnceCell<regex::Regex> = once_cell::sync::OnceCell::new();
        RE.get_or_init(|| regex::Regex::new($re).unwrap())
    }};
}

/// Matches a regex to a string, then parses each capture group as necessary to
/// produce a tuple of the desired return type.
pub fn parse_with_regex<'a, T: MatchTuple<'a>>(
    re: &Regex,
    s: &'a str,
) -> Result<T, Box<dyn error::Error>> {
    let caps = re.captures(s).ok_or("Regex did not match string.")?;
    if caps.len() != T::len() + 1 {
        Err(format!(
            "Expected {} (non-global) capture groups, found {}.",
            T::len(),
            caps.len() - 1,
        ))?
    }
    T::parse_captures(&caps)
}

pub trait MatchTuple<'a>: Sized {
    fn len() -> usize;
    fn parse_captures(caps: &Captures<'a>) -> Result<Self, Box<dyn error::Error>>;
}

pub trait FromStr2<'a>: Sized {
    type Err;

    fn from_str(s: &'a str) -> Result<Self, Self::Err>;
}

impl<'a, T: FromStr> FromStr2<'a> for T {
    type Err = T::Err;

    fn from_str(s: &'a str) -> Result<Self, Self::Err> {
        s.parse()
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Str<'a>(pub &'a str);

impl<'a> Str<'a> {
    pub fn as_str(&self) -> &'a str {
        self.0
    }
}

impl<'a> FromStr2<'a> for Str<'a> {
    type Err = Infallible;

    fn from_str(s: &'a str) -> Result<Self, Self::Err> {
        Ok(Self(s))
    }
}

macro_rules! impl_match_tuple {
    ($($T:ident),*) => {
        impl <'a, $($T, )*> MatchTuple<'a> for ($($T, )*)
        where
            $(
                $T: FromStr2<'a>,
                <$T as FromStr2<'a>>::Err: std::error::Error + 'static,
            )*
        {
            fn len() -> usize {
                count_args!($($T )*)
            }

            fn parse_captures(_caps: &Captures<'a>) -> Result<Self, Box<dyn std::error::Error>> {
                Ok(parse_to_tuple!(_caps $($T )*))
            }
        }
    }
}

macro_rules! count_args {
    () => {
        0
    };
    ($head:ident $($tail:tt)*) => {
        count_args!($($tail)*) + 1
    };
}

macro_rules! parse_to_tuple {
    (@accum [$caps:ident][][$($n:tt)*] -> [$($body:tt)*]) => {
        as_expr!(($($body)*))
    };
    (@accum [$caps:ident][$head:ident $($tail:tt)*][$($n:tt)*] -> [$($body:tt)*]) => {
        parse_to_tuple!(@accum [$caps][$($tail)*][$($n)* + 1] -> [$($body)* FromStr2::from_str(&$caps.get($($n)*).ok_or("Missing capture")?.as_str())?,])
    };
    (@as_expr $e:expr) => {
        $e
    };
    ($caps:ident $($all:tt)*) => {
        parse_to_tuple!(@accum [$caps][$($all)*][1] -> [])
    }
}

macro_rules! as_expr {
    ($e:expr) => {
        $e
    };
}

impl_match_tuple!();
impl_match_tuple!(A);
impl_match_tuple!(A, B);
impl_match_tuple!(A, B, C);
impl_match_tuple!(A, B, C, D);
impl_match_tuple!(A, B, C, D, E);
impl_match_tuple!(A, B, C, D, E, F);
impl_match_tuple!(A, B, C, D, E, F, G);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_match() {
        let re = regex!(r"^(.+) (.+) stole (\d+) cakes.$");
        let (first_name, Str(last_name), count): (String, Str, usize) =
            parse_with_regex(re, "Lex Luthor stole 40 cakes.").unwrap();
        assert_eq!(first_name, "Lex".to_owned());
        assert_eq!(last_name, "Luthor");
        assert_eq!(count, 40);
    }
}
