use std::marker::PhantomData;

#[derive(Clone, Copy)]
pub struct BasicSensors(PhantomData<()>);

#[derive(Clone, Copy)]
pub struct AdvancedSensors(PhantomData<()>);

impl BasicSensors {
    pub fn new() -> Self {
        BasicSensors(PhantomData)
    }
}

impl AdvancedSensors {
    pub fn new() -> Self {
        AdvancedSensors(PhantomData)
    }
}

pub trait Sensors {
    fn get_type(&self) -> &'static str;

    fn get_slot_cost(&self) -> i32 {
        1
    }

    fn get_mass(&self) -> i32;

    fn get_power_draw(&self) -> i32 {
        50
    }
}

impl Sensors for BasicSensors {
    fn get_type(&self) -> &'static str {
        "Basic"
    }

    fn get_mass(&self) -> i32 {
        30
    }
}

impl Sensors for AdvancedSensors {
    fn get_type(&self) -> &'static str {
        "Advanced"
    }

    fn get_mass(&self) -> i32 {
        35
    }
}

pub enum SensorsType {
    Basic(BasicSensors),
    Advanced(AdvancedSensors),
}

impl Into<SensorsType> for BasicSensors {
    fn into(self) -> SensorsType {
        SensorsType::Basic(self)
    }
}

impl Into<SensorsType> for AdvancedSensors {
    fn into(self) -> SensorsType {
        SensorsType::Advanced(self)
    }
}
