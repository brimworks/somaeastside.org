use std::fmt;

#[derive(Clone, PartialEq, Eq, Hash)]
pub(crate) struct SecretStr(String);

impl fmt::Debug for SecretStr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0.len() < 8 {
            write!(f, "Secret(len={})", self.0.len())
        } else {
            write!(f, "Secret(len={}, {}…)", self.0.len(), &self.0[0..4])
        }
    }
}

impl From<String> for SecretStr {
    fn from(value: String) -> Self {
        SecretStr(value)
    }
}

impl From<SecretStr> for String {
    fn from(value: SecretStr) -> Self {
        value.0
    }
}

impl AsRef<str> for SecretStr {
    #[inline(always)]
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl AsMut<str> for SecretStr {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut str {
        &mut self.0
    }
}

