pub mod bridge;
pub mod engine;
pub mod life_support;
pub mod reactor;
pub mod sensors;
pub mod shield;

#[derive(Debug, Clone)]
pub struct Yes;

#[derive(Debug, Clone)]
pub struct No;

pub trait HasModule {}
impl HasModule for Yes {}
impl HasModule for No {}
