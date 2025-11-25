use std::marker::PhantomData;

use crate::spaceship::blueprint::module::{HasModule, No};

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

pub trait Shield<FR: HasModule, AR: HasModule> {
    fn get_type(&self) -> &'static str;

    fn get_slot_cost(&self) -> i32 {
        1
    }

    fn get_mass(&self) -> i32 {
        40
    }

    fn get_power_draw(&self) -> i32 {
        100
    }
}

impl<FR: HasModule> Shield<FR, No> for MagneticShield {
    fn get_type(&self) -> &'static str {
        "Magnetic"
    }
}

impl<AR: HasModule> Shield<No, AR> for PhaseShield {
    fn get_type(&self) -> &'static str {
        "Phase"
    }
}

pub enum ShieldType {
    Magnetic(MagneticShield),
    Phase(PhaseShield),
}

impl Into<ShieldType> for MagneticShield {
    fn into(self) -> ShieldType {
        ShieldType::Magnetic(self)
    }
}

impl Into<ShieldType> for PhaseShield {
    fn into(self) -> ShieldType {
        ShieldType::Phase(self)
    }
}
