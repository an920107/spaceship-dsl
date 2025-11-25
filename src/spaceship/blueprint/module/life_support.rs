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

pub trait LifeSupport {
    fn get_type(&self) -> &'static str;

    fn get_slot_cost(&self) -> i32 {
        2
    }

    fn get_mass(&self) -> i32;

    fn get_power_draw(&self) -> i32 {
        75
    }
}

impl LifeSupport for StandardLifeSupport {
    fn get_type(&self) -> &'static str {
        "Standard"
    }

    fn get_mass(&self) -> i32 {
        80
    }
}

impl LifeSupport for AdvancedLifeSupport {
    fn get_type(&self) -> &'static str {
        "Advanced"
    }

    fn get_mass(&self) -> i32 {
        70
    }
}

pub enum LifeSupportType {
    Standard(StandardLifeSupport),
    Advanced(AdvancedLifeSupport),
}

impl Into<LifeSupportType> for StandardLifeSupport {
    fn into(self) -> LifeSupportType {
        LifeSupportType::Standard(self)
    }
}

impl Into<LifeSupportType> for AdvancedLifeSupport {
    fn into(self) -> LifeSupportType {
        LifeSupportType::Advanced(self)
    }
}
