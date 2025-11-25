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

pub trait ReactorConstraint<const N: usize, FR: HasModule, AR: HasModule> {
    type NewFR: HasModule;
    type NewAR: HasModule;
}

impl<const N: usize, FR: HasModule, AR: HasModule> ReactorConstraint<N, FR, AR> for FusionReactor {
    type NewFR = Yes;
    type NewAR = AR;
}

impl<const N: usize, FR: HasModule, AR: HasModule> ReactorConstraint<N, FR, AR>
    for AntimatterReactor
{
    type NewFR = FR;
    type NewAR = Yes;
}

#[derive(Clone, Copy)]
pub enum Reactor {
    Fusion(FusionReactor),
    Antimatter(AntimatterReactor),
}

impl Into<Reactor> for FusionReactor {
    fn into(self) -> Reactor {
        Reactor::Fusion(self)
    }
}

impl Into<Reactor> for AntimatterReactor {
    fn into(self) -> Reactor {
        Reactor::Antimatter(self)
    }
}

impl Reactor {
    pub fn get_mass(&self) -> i32 {
        match self {
            Reactor::Fusion(_) => 300,
            Reactor::Antimatter(_) => 450,
        }
    }

    pub fn get_power_output(&self) -> i32 {
        match self {
            Reactor::Fusion(_) => 1000,
            Reactor::Antimatter(_) => 1000,
        }
    }
}
