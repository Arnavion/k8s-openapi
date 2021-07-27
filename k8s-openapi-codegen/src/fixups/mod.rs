#![deny(unused)]

pub(crate) trait Fixup {
    fn fixup(&self, spec: &mut crate::swagger20::Spec) -> Result<(), crate::Error>;

    fn requires_client_generation(&self) -> bool;
}

pub(crate) mod special;

pub(crate) mod upstream_bugs;
