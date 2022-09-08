use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

/// Price level of the business.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum PriceType {
    #[serde(rename = "$")]
    OneDollar,

    #[serde(rename = "$$")]
    TwoDollar,

    #[serde(rename = "$$$")]
    ThreeDollar,

    #[serde(rename = "$$$$")]
    FourDollar,
}

impl PriceType {
    #[must_use]
    pub const fn as_str(&self) -> &str {
        match self {
            Self::OneDollar => "$",
            Self::TwoDollar => "$$",
            Self::ThreeDollar => "$$$",
            Self::FourDollar => "$$$$",
        }
    }

    #[must_use]
    pub const fn as_usize(&self) -> usize {
        match self {
            Self::OneDollar => 1,
            Self::TwoDollar => 2,
            Self::ThreeDollar => 3,
            Self::FourDollar => 4,
        }
    }
}

impl Display for PriceType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
