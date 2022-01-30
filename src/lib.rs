#![cfg_attr(feature = "bench", feature(test))]
#[cfg(feature = "bench")]
mod bench;
mod r#impl;
#[cfg(test)]
mod test;

pub trait TrimOwned {
    fn trim_owned(self) -> Self;
}

impl TrimOwned for String {
    fn trim_owned(self) -> Self {
        r#impl::r#drain(self)
    }
}
