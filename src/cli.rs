use ethcontract::prelude::*;
use std::str::FromStr;

/// Trait for CLI arguments that represent an amount of some currency.
pub trait Currency: FromStr {
    /// Get the underlying int type.
    fn as_inner(&self) -> U256;
}

/// For CLI arguments that take amount of ether.
///
/// Supports parsing ether values in human-readable form,
/// i.e. `1eth` or `100gwei` or others.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Eth(U256);

impl Currency for Eth {
    fn as_inner(&self) -> U256 {
        self.0
    }
}

impl FromStr for Eth {
    type Err = uint::FromStrRadixErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_ascii_lowercase();

        let (s, modifier) = if let Some(s) = s.strip_suffix("ether") {
            (s, U256::exp10(18))
        } else if let Some(s) = s.strip_suffix("eth") {
            (s, U256::exp10(18))
        } else if let Some(s) = s.strip_suffix("pwei") {
            (s, U256::exp10(15))
        } else if let Some(s) = s.strip_suffix("twei") {
            (s, U256::exp10(12))
        } else if let Some(s) = s.strip_suffix("gwei") {
            (s, U256::exp10(9))
        } else if let Some(s) = s.strip_suffix("mwei") {
            (s, U256::exp10(6))
        } else if let Some(s) = s.strip_suffix("kwei") {
            (s, U256::exp10(3))
        } else if let Some(s) = s.strip_suffix("wei") {
            (s, U256::exp10(0))
        } else {
            (s.as_str(), U256::exp10(0))
        };

        if s.starts_with("0x") {
            Ok(Eth(U256::from_str_radix(s, 16)? * modifier))
        } else {
            Ok(Eth(U256::from_str_radix(s, 10)? * modifier))
        }
    }
}

#[cfg(test)]
mod test_eth {
    use super::*;

    #[test]
    fn eth_from_str() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(Eth::from_str("0")?.0, U256::from(0));
        assert_eq!(Eth::from_str("1")?.0, U256::from_dec_str("1")?);
        assert_eq!(Eth::from_str("15")?.0, U256::from_dec_str("15")?);
        assert_eq!(Eth::from_str("0x1")?.0, U256::from_dec_str("1")?);
        assert_eq!(Eth::from_str("0x15")?.0, U256::from_dec_str("21")?);

        assert_eq!(Eth::from_str("5wei")?.0, U256::from_dec_str("5")?);
        assert_eq!(Eth::from_str("5kwei")?.0, U256::from_dec_str("5000")?);
        assert_eq!(Eth::from_str("5mwei")?.0, U256::from_dec_str("5000000")?);
        assert_eq!(Eth::from_str("5gwei")?.0, U256::from_dec_str("5000000000")?);
        assert_eq!(Eth::from_str("5twei")?.0, U256::from_dec_str("5000000000000")?);
        assert_eq!(Eth::from_str("5pwei")?.0, U256::from_dec_str("5000000000000000")?);
        assert_eq!(Eth::from_str("5eth")?.0, U256::from_dec_str("5000000000000000000")?);
        assert_eq!(Eth::from_str("5ether")?.0, U256::from_dec_str("5000000000000000000")?);

        Ok(())
    }
}

/// For CLI arguments that take amount of SCM tokens.
///
/// Supports parsing ether values in human-readable form,
/// i.e. `1scm` or `100asc` or others.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Scm(U256);

impl Currency for Scm {
    fn as_inner(&self) -> U256 {
        self.0
    }
}

impl FromStr for Scm {
    type Err = uint::FromStrRadixErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_ascii_lowercase();

        let (s, modifier) = if let Some(s) = s.strip_suffix("scam") {
            (s, U256::exp10(18))
        } else if let Some(s) = s.strip_suffix("scm") {
            (s, U256::exp10(18))
        } else if let Some(s) = s.strip_suffix("msc") {
            (s, U256::exp10(15))
        } else if let Some(s) = s.strip_suffix("usc") {
            (s, U256::exp10(12))
        } else if let Some(s) = s.strip_suffix("nsc") {
            (s, U256::exp10(9))
        } else if let Some(s) = s.strip_suffix("psc") {
            (s, U256::exp10(6))
        } else if let Some(s) = s.strip_suffix("fsc") {
            (s, U256::exp10(3))
        } else if let Some(s) = s.strip_suffix("asc") {
            (s, U256::exp10(0))
        } else {
            (s.as_str(), U256::exp10(0))
        };

        if s.starts_with("0x") {
            Ok(Scm(U256::from_str_radix(s, 16)? * modifier))
        } else {
            Ok(Scm(U256::from_str_radix(s, 10)? * modifier))
        }
    }
}

#[cfg(test)]
mod test_scm {
    use super::*;

    #[test]
    fn eth_from_str() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(Scm::from_str("0")?.0, U256::from(0));
        assert_eq!(Scm::from_str("1")?.0, U256::from_dec_str("1")?);
        assert_eq!(Scm::from_str("15")?.0, U256::from_dec_str("15")?);
        assert_eq!(Scm::from_str("0x1")?.0, U256::from_dec_str("1")?);
        assert_eq!(Scm::from_str("0x15")?.0, U256::from_dec_str("21")?);

        assert_eq!(Scm::from_str("5asc")?.0, U256::from_dec_str("5")?);
        assert_eq!(Scm::from_str("5fsc")?.0, U256::from_dec_str("5000")?);
        assert_eq!(Scm::from_str("5psc")?.0, U256::from_dec_str("5000000")?);
        assert_eq!(Scm::from_str("5nsc")?.0, U256::from_dec_str("5000000000")?);
        assert_eq!(Scm::from_str("5usc")?.0, U256::from_dec_str("5000000000000")?);
        assert_eq!(Scm::from_str("5msc")?.0, U256::from_dec_str("5000000000000000")?);
        assert_eq!(Scm::from_str("5scm")?.0, U256::from_dec_str("5000000000000000000")?);
        assert_eq!(Scm::from_str("5scam")?.0, U256::from_dec_str("5000000000000000000")?);

        Ok(())
    }
}
