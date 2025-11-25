use std::marker::PhantomData;

#[derive(Clone, Copy)]
pub struct StandardLifeSupport(PhantomData<()>);

#[derive(Clone, Copy)]
pub struct AdvancedLifeSupport(PhantomData<()>);

impl StandardLifeSupport {
    pub fn new() -> Self {
        StandardLifeSupport(PhantomData)
    }
}

impl AdvancedLifeSupport {
    pub fn new() -> Self {
        AdvancedLifeSupport(PhantomData)
    }
}

pub trait LifeSupportConstraint {}

impl LifeSupportConstraint for StandardLifeSupport {}

impl LifeSupportConstraint for AdvancedLifeSupport {}

pub enum LifeSupport {
    Standard(StandardLifeSupport),
    Advanced(AdvancedLifeSupport),
}

impl Into<LifeSupport> for StandardLifeSupport {
    fn into(self) -> LifeSupport {
        LifeSupport::Standard(self)
    }
}

impl Into<LifeSupport> for AdvancedLifeSupport {
    fn into(self) -> LifeSupport {
        LifeSupport::Advanced(self)
    }
}

impl LifeSupport {
    pub fn get_mass(&self) -> i32 {
        match self {
            LifeSupport::Standard(_) => 80,
            LifeSupport::Advanced(_) => 70,
        }
    }

    pub fn get_power_draw(&self) -> i32 {
        match self {
            LifeSupport::Standard(_) => 50,
            LifeSupport::Advanced(_) => 50,
        }
    }
}
