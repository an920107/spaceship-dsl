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

pub trait EngineConstraint {}

impl EngineConstraint for IonEngine {}

impl EngineConstraint for PlasmaEngine {}

pub enum Engine {
    Ion(IonEngine),
    Plasma(PlasmaEngine),
}

impl From<IonEngine> for Engine {
    fn from(engine: IonEngine) -> Self {
        Engine::Ion(engine)
    }
}

impl From<PlasmaEngine> for Engine {
    fn from(engine: PlasmaEngine) -> Self {
        Engine::Plasma(engine)
    }
}

impl Engine {
    pub fn get_mass(&self) -> i32 {
        match self {
            Engine::Ion(_) => 100,
            Engine::Plasma(_) => 120,
        }
    }

    pub fn get_power_draw(&self) -> i32 {
        match self {
            Engine::Ion(_) => 250,
            Engine::Plasma(_) => 250,
        }
    }

    pub fn get_thrust(&self) -> i32 {
        match self {
            Engine::Ion(_) => 500,
            Engine::Plasma(_) => 750,
        }
    }
}
