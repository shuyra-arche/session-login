//! Startup context delivery and issuer lifetime.

mod context;
pub(crate) mod issuer;

pub(crate) use issuer::deliver;
