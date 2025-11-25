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

pub trait SensorsConstraint {}

impl SensorsConstraint for BasicSensors {}

impl SensorsConstraint for AdvancedSensors {}

pub enum Sensors {
    Basic(BasicSensors),
    Advanced(AdvancedSensors),
}

impl Into<Sensors> for BasicSensors {
    fn into(self) -> Sensors {
        Sensors::Basic(self)
    }
}

impl Into<Sensors> for AdvancedSensors {
    fn into(self) -> Sensors {
        Sensors::Advanced(self)
    }
}

impl Sensors {
    pub fn get_mass(&self) -> i32 {
        match self {
            Sensors::Basic(_) => 30,
            Sensors::Advanced(_) => 35,
        }
    }

    pub fn get_power_draw(&self) -> i32 {
        match self {
            Sensors::Basic(_) => 50,
            Sensors::Advanced(_) => 50,
        }
    }
}
