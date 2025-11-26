use crate::blueprint::module::{No, Yes};

#[diagnostic::on_unimplemented(
    message = "cannot lock core modules: not all core modules installed"
)]
pub trait AllCoreModulesInstalled<R, E, L, B> {}
impl AllCoreModulesInstalled<Yes, Yes, Yes, Yes> for () {}

#[diagnostic::on_unimplemented(message = "cannot add engine: engine already installed")]
pub trait NoEngineInstalled<E> {}
impl NoEngineInstalled<No> for () {}

#[diagnostic::on_unimplemented(message = "cannot add life support: life support already installed")]
pub trait NoLifeSupportInstalled<L> {}
impl NoLifeSupportInstalled<No> for () {}

#[diagnostic::on_unimplemented(message = "cannot add bridge: bridge already installed")]
pub trait NoBridgeInstalled<B> {}
impl NoBridgeInstalled<No> for () {}

#[diagnostic::on_unimplemented(message = "cannot add shield: shield already installed")]
pub trait NoShieldInstalled<H> {}
impl NoShieldInstalled<No> for () {}

#[diagnostic::on_unimplemented(message = "cannot add sensors: sensors already installed")]
pub trait NoSensorsInstalled<E> {}
impl NoSensorsInstalled<No> for () {}
