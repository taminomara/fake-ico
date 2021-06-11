use ethcontract::prelude::*;
use std::str::FromStr;

/// For CLI arguments that take amount of ether.
///
/// Supports parsing ether values in human-readable form,
/// i.e. `1eth` or `100gwei` or others.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Eth(U256);

impl Eth {
    /// Get the underlying int type.
    pub fn as_inner(self) -> U256 {
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
            (s.as_str(), U256::exp10(18))
        };

        if s.starts_with("0x") {
            Ok(Eth(U256::from_str_radix(s, 16)? * modifier))
        } else {
            Ok(Eth(U256::from_str_radix(s, 10)? * modifier))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn eth_from_str() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(Eth::from_str("0")?.0, U256::from(0));
        assert_eq!(Eth::from_str("1")?.0, U256::from_dec_str("1000000000000000000")?);
        assert_eq!(Eth::from_str("15")?.0, U256::from_dec_str("15000000000000000000")?);
        assert_eq!(Eth::from_str("0x1")?.0, U256::from_dec_str("1000000000000000000")?);
        assert_eq!(Eth::from_str("0x15")?.0, U256::from_dec_str("21000000000000000000")?);

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
