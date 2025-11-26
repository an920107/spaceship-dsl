use std::marker::PhantomData;

use crate::blueprint::module::{HasModule, No};

#[derive(Clone, Copy)]
pub struct MagneticShield(PhantomData<()>);

#[derive(Clone, Copy)]
pub struct PhaseShield(PhantomData<()>);

impl MagneticShield {
    pub fn new() -> Self {
        MagneticShield(PhantomData)
    }
}

impl PhaseShield {
    pub fn new() -> Self {
        PhaseShield(PhantomData)
    }
}

#[diagnostic::on_unimplemented(
    message = "cannot add shield: shield type is incompatible with the current reactor"
)]
pub trait ShieldConstraint<FR: HasModule, AR: HasModule> {}

impl<FR: HasModule> ShieldConstraint<FR, No> for MagneticShield {}

impl<AR: HasModule> ShieldConstraint<No, AR> for PhaseShield {}

pub enum Shield {
    Magnetic(MagneticShield),
    Phase(PhaseShield),
}

impl Into<Shield> for MagneticShield {
    fn into(self) -> Shield {
        Shield::Magnetic(self)
    }
}

impl Into<Shield> for PhaseShield {
    fn into(self) -> Shield {
        Shield::Phase(self)
    }
}

impl Shield {
    pub fn get_mass(&self) -> i32 {
        match self {
            Shield::Magnetic(_) => 40,
            Shield::Phase(_) => 30,
        }
    }

    pub fn get_power_draw(&self) -> i32 {
        match self {
            Shield::Magnetic(_) => 100,
            Shield::Phase(_) => 120,
        }
    }
}
