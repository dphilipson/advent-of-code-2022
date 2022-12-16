use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::str;
use std::str::FromStr;

#[derive(Copy, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SizedStr<const SIZE: usize>([u8; SIZE]);

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SizedStrError {
    InputTooLong,
}

impl<const SIZE: usize> FromStr for SizedStr<SIZE> {
    type Err = SizedStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s.as_bytes();
        if bytes.len() > SIZE {
            return Err(Self::Err::InputTooLong);
        }
        let mut out = [0_u8; SIZE];
        out[..bytes.len()].copy_from_slice(bytes);
        Ok(Self(out))
    }
}

impl<const SIZE: usize> Debug for SizedStr<SIZE> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("s").unwrap();
        Debug::fmt(self.as_str(), f)
    }
}

impl<const SIZE: usize> Display for SizedStr<SIZE> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self.as_str(), f)
    }
}

impl<const SIZE: usize> SizedStr<SIZE> {
    fn len(&self) -> usize {
        self.0.iter().position(|&b| b == 0).unwrap_or(SIZE)
    }

    fn as_str(&self) -> &str {
        str::from_utf8(&self.0[..self.len()]).unwrap()
    }

    fn as_bytes(&self) -> &[u8] {
        &self.0[..self.len()]
    }
}

impl Display for SizedStrError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt("input str too large to fit in fixed size str", f)
    }
}

impl Error for SizedStrError {}

#[cfg(test)]
mod tests {
    use super::*;

    type Str5 = SizedStr<5>;

    #[test]
    fn test_full_length_str() {
        test_str("hello");
    }

    #[test]
    fn test_part_length_str() {
        test_str("hi");
    }

    fn test_str(s: &str) {
        let sized_s = Str5::from_str(s).unwrap();
        assert_eq!(sized_s.len(), s.len());
        assert_eq!(sized_s.as_str(), s);
        assert_eq!(sized_s.as_bytes(), s.as_bytes());
        assert_eq!(format!("{sized_s}"), s);
        assert_eq!(format!("{sized_s:?}"), format!("s\"{s}\""));
    }
}
