use std::marker::PhantomData;

use crate::spaceship::blueprint::{
    module::{
        HasModule, No, Yes,
        bridge::{Bridge, BridgeConstraint},
        engine::{Engine, EngineConstraint},
        life_support::{LifeSupport, LifeSupportConstraint},
        reactor::{Reactor, ReactorConstraint},
        sensors::{Sensors, SensorsConstraint},
        shield::{Shield, ShieldConstraint},
    },
    slot::{SlotCheck, SlotIsAvailable},
    stage::{
        CoreModulesInstallationStage, FinalizationStage, InitialStage,
        OptionalModulesInstallationStage, Stage,
    },
};

pub mod module;
mod slot;
mod stage;

pub struct Blueprint<S: Stage, const N: usize, FR: HasModule, AR: HasModule> {
    reactors: Option<Vec<Reactor>>,
    engine: Option<Engine>,
    life_support: Option<LifeSupport>,
    bridge: Option<Bridge>,
    shield: Option<Shield>,
    sensors: Option<Sensors>,
    _stage: PhantomData<(S, FR, AR)>,
}

impl Blueprint<InitialStage, 0, No, No> {
    pub fn new() -> Self {
        Blueprint {
            reactors: None,
            engine: None,
            life_support: None,
            bridge: None,
            shield: None,
            sensors: None,
            _stage: PhantomData,
        }
    }
}

impl<const N: usize, FR: HasModule, AR: HasModule> Blueprint<InitialStage, N, FR, AR> {
    pub fn set_frame(self) -> Blueprint<CoreModulesInstallationStage<No, No, No, No>, N, FR, AR> {
        Blueprint {
            reactors: Some(Vec::new()),
            engine: None,
            life_support: None,
            bridge: None,
            shield: None,
            sensors: None,
            _stage: PhantomData,
        }
    }
}

impl<
    const N: usize,
    R: HasModule,
    E: HasModule,
    L: HasModule,
    B: HasModule,
    FR: HasModule,
    AR: HasModule,
> Blueprint<CoreModulesInstallationStage<R, E, L, B>, N, FR, AR>
{
    pub fn add_reactor<T>(
        mut self,
        reactor: T,
    ) -> Blueprint<CoreModulesInstallationStage<Yes, E, L, B>, { N + 3 }, T::NewFR, T::NewAR>
    where
        T: ReactorConstraint<N, FR, AR> + Into<Reactor>,
        SlotCheck<N, 3>: SlotIsAvailable,
    {
        if let Some(reactors) = self.reactors.as_mut() {
            reactors.push(reactor.into());
        }
        Blueprint {
            reactors: self.reactors,
            engine: self.engine,
            life_support: self.life_support,
            bridge: self.bridge,
            shield: self.shield,
            sensors: self.sensors,
            _stage: PhantomData,
        }
    }
}

impl<const N: usize, R: HasModule, L: HasModule, B: HasModule, FR: HasModule, AR: HasModule>
    Blueprint<CoreModulesInstallationStage<R, No, L, B>, N, FR, AR>
{
    pub fn add_engine<T>(
        self,
        engine: T,
    ) -> Blueprint<CoreModulesInstallationStage<R, Yes, L, B>, { N + 2 }, FR, AR>
    where
        T: EngineConstraint + Into<Engine>,
        SlotCheck<N, 2>: SlotIsAvailable,
    {
        Blueprint {
            reactors: self.reactors,
            engine: Some(engine.into()),
            life_support: self.life_support,
            bridge: self.bridge,
            shield: self.shield,
            sensors: self.sensors,
            _stage: PhantomData,
        }
    }
}

impl<const N: usize, R: HasModule, E: HasModule, B: HasModule, FR: HasModule, AR: HasModule>
    Blueprint<CoreModulesInstallationStage<R, E, No, B>, N, FR, AR>
{
    pub fn add_life_support<T>(
        self,
        life_support: T,
    ) -> Blueprint<CoreModulesInstallationStage<R, E, Yes, B>, { N + 2 }, FR, AR>
    where
        T: LifeSupportConstraint + Into<LifeSupport>,
        SlotCheck<N, 2>: SlotIsAvailable,
    {
        Blueprint {
            reactors: self.reactors,
            engine: self.engine,
            life_support: Some(life_support.into()),
            bridge: self.bridge,
            shield: self.shield,
            sensors: self.sensors,
            _stage: PhantomData,
        }
    }
}

impl<const N: usize, R: HasModule, E: HasModule, L: HasModule, FR: HasModule, AR: HasModule>
    Blueprint<CoreModulesInstallationStage<R, E, L, No>, N, FR, AR>
{
    pub fn add_bridge<T>(
        self,
        bridge: T,
    ) -> Blueprint<CoreModulesInstallationStage<R, E, L, Yes>, { N + 1 }, FR, AR>
    where
        T: BridgeConstraint + Into<Bridge>,
        SlotCheck<N, 1>: SlotIsAvailable,
    {
        Blueprint {
            reactors: self.reactors,
            engine: self.engine,
            life_support: self.life_support,
            bridge: Some(bridge.into()),
            shield: self.shield,
            sensors: self.sensors,
            _stage: PhantomData,
        }
    }
}

impl<const N: usize, FR: HasModule, AR: HasModule>
    Blueprint<CoreModulesInstallationStage<Yes, Yes, Yes, Yes>, N, FR, AR>
{
    pub fn lock_core_modules(
        self,
    ) -> Blueprint<OptionalModulesInstallationStage<No, No>, N, FR, AR> {
        Blueprint {
            reactors: self.reactors,
            engine: self.engine,
            life_support: self.life_support,
            bridge: self.bridge,
            shield: self.shield,
            sensors: self.sensors,
            _stage: PhantomData,
        }
    }
}

impl<const N: usize, E: HasModule, FR: HasModule, AR: HasModule>
    Blueprint<OptionalModulesInstallationStage<No, E>, N, FR, AR>
{
    pub fn add_shield<T>(
        self,
        shield: T,
    ) -> Blueprint<OptionalModulesInstallationStage<Yes, E>, { N + 1 }, FR, AR>
    where
        T: ShieldConstraint<FR, AR> + Into<Shield>,
        SlotCheck<N, 1>: SlotIsAvailable,
    {
        Blueprint {
            reactors: self.reactors,
            engine: self.engine,
            life_support: self.life_support,
            bridge: self.bridge,
            shield: Some(shield.into()),
            sensors: self.sensors,
            _stage: PhantomData,
        }
    }
}

impl<const N: usize, H: HasModule, FR: HasModule, AR: HasModule>
    Blueprint<OptionalModulesInstallationStage<H, No>, N, FR, AR>
{
    pub fn add_sensors<T>(
        self,
        sensors: T,
    ) -> Blueprint<OptionalModulesInstallationStage<H, Yes>, { N + 1 }, FR, AR>
    where
        T: SensorsConstraint + Into<Sensors>,
        SlotCheck<N, 1>: SlotIsAvailable,
    {
        Blueprint {
            reactors: self.reactors,
            engine: self.engine,
            life_support: self.life_support,
            bridge: self.bridge,
            shield: self.shield,
            sensors: Some(sensors.into()),
            _stage: PhantomData,
        }
    }
}

impl<const N: usize, H: HasModule, E: HasModule, FR: HasModule, AR: HasModule>
    Blueprint<OptionalModulesInstallationStage<H, E>, N, FR, AR>
{
    pub fn finalize(self) -> Blueprint<FinalizationStage, N, FR, AR> {
        Blueprint {
            reactors: self.reactors,
            engine: self.engine,
            life_support: self.life_support,
            bridge: self.bridge,
            shield: self.shield,
            sensors: self.sensors,
            _stage: PhantomData,
        }
    }
}

impl<const N: usize, FR: HasModule, AR: HasModule> Blueprint<FinalizationStage, N, FR, AR> {
    pub fn print_spec(&self) {
        println!("Spaceship Blueprint Specification:");
        println!("====== Slots ======");
        println!("Total slots: {}", slot::TOTAL_SLOTS);
        println!("Used slots: {}", N);
        println!("Available slots: {}", slot::TOTAL_SLOTS - N);
        println!("====== Power ======");
        println!("Total Power Output: {}", self.get_power_output());
        println!("Total Power Consumption: {}", self.get_power_draw());
        println!(
            "Power Balance: {}",
            self.get_power_output() - self.get_power_draw()
        );
        println!("====== Load ======");
        println!(
            "Thrust-to-Weight Ratio: {}",
            if self.get_mass() > 0 {
                self.get_thrust() as f32 / self.get_mass() as f32
            } else {
                0.0
            }
        );
        println!();
    }

    fn get_power_output(&self) -> i32 {
        match &self.reactors {
            Some(reactors) => reactors.iter().map(|r| r.get_power_output()).sum(),
            None => 0,
        }
    }

    fn get_power_draw(&self) -> i32 {
        self.engine.as_ref().map_or(0, |e| e.get_power_draw())
            + self
                .life_support
                .as_ref()
                .map_or(0, |ls| ls.get_power_draw())
            + self.bridge.as_ref().map_or(0, |b| b.get_power_draw())
            + self.shield.as_ref().map_or(0, |s| s.get_power_draw())
            + self.sensors.as_ref().map_or(0, |s| s.get_power_draw())
    }

    fn get_mass(&self) -> i32 {
        self.engine.as_ref().map_or(0, |e| e.get_mass())
            + self.life_support.as_ref().map_or(0, |ls| ls.get_mass())
            + self.bridge.as_ref().map_or(0, |b| b.get_mass())
            + self.shield.as_ref().map_or(0, |s| s.get_mass())
            + self.sensors.as_ref().map_or(0, |s| s.get_mass())
            + match &self.reactors {
                Some(reactors) => reactors.iter().map(|r| r.get_mass()).sum(),
                None => 0,
            }
    }

    fn get_thrust(&self) -> i32 {
        self.engine.as_ref().map_or(0, |e| e.get_thrust())
    }
}
