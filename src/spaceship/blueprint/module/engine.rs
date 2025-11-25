use std::marker::PhantomData;

#[derive(Clone, Copy)]
pub struct IonEngine(PhantomData<()>);

#[derive(Clone, Copy)]
pub struct PlasmaEngine(PhantomData<()>);

impl IonEngine {
    pub fn new() -> Self {
        IonEngine(PhantomData)
    }
}

impl PlasmaEngine {
    pub fn new() -> Self {
        PlasmaEngine(PhantomData)
    }
}

pub trait Engine {
    fn get_type(&self) -> &'static str;

    fn get_slot_cost(&self) -> i32 {
        2
    }

    fn get_mass(&self) -> i32;

    fn get_power_draw(&self) -> i32 {
        250
    }

    fn get_thrust(&self) -> i32;
}

impl Engine for IonEngine {
    fn get_type(&self) -> &'static str {
        "Ion"
    }

    fn get_mass(&self) -> i32 {
        100
    }

    fn get_thrust(&self) -> i32 {
        500
    }
}

impl Engine for PlasmaEngine {
    fn get_type(&self) -> &'static str {
        "Plasma"
    }

    fn get_mass(&self) -> i32 {
        120
    }

    fn get_thrust(&self) -> i32 {
        750
    }
}

pub enum EngineType {
    Ion(IonEngine),
    Plasma(PlasmaEngine),
}

impl From<IonEngine> for EngineType {
    fn from(engine: IonEngine) -> Self {
        EngineType::Ion(engine)
    }
}

impl From<PlasmaEngine> for EngineType {
    fn from(engine: PlasmaEngine) -> Self {
        EngineType::Plasma(engine)
    }
}
