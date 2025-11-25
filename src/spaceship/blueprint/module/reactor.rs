use std::marker::PhantomData;

use super::{HasModule, Yes};

#[derive(Clone, Copy)]
pub struct FusionReactor(PhantomData<()>);

#[derive(Clone, Copy)]
pub struct AntimatterReactor(PhantomData<()>);

impl FusionReactor {
    pub fn new() -> Self {
        FusionReactor(PhantomData)
    }
}

impl AntimatterReactor {
    pub fn new() -> Self {
        AntimatterReactor(PhantomData)
    }
}

pub trait Reactor<const N: usize, FR: HasModule, AR: HasModule> {
    type NewFR: HasModule;
    type NewAR: HasModule;

    fn get_type(&self) -> &'static str;

    fn get_mass(&self) -> i32;

    fn get_power_output(&self) -> i32 {
        1000
    }
}

impl<const N: usize, FR: HasModule, AR: HasModule> Reactor<N, FR, AR> for FusionReactor {
    type NewFR = Yes;
    type NewAR = AR;

    fn get_type(&self) -> &'static str {
        "Fusion"
    }

    fn get_mass(&self) -> i32 {
        300
    }
}

impl<const N: usize, FR: HasModule, AR: HasModule> Reactor<N, FR, AR> for AntimatterReactor {
    type NewFR = FR;
    type NewAR = Yes;

    fn get_type(&self) -> &'static str {
        "Antimatter"
    }

    fn get_mass(&self) -> i32 {
        450
    }
}

#[derive(Clone, Copy)]
pub enum ReactorType {
    Fusion(FusionReactor),
    Antimatter(AntimatterReactor),
}

impl Into<ReactorType> for FusionReactor {
    fn into(self) -> ReactorType {
        ReactorType::Fusion(self)
    }
}

impl Into<ReactorType> for AntimatterReactor {
    fn into(self) -> ReactorType {
        ReactorType::Antimatter(self)
    }
}
