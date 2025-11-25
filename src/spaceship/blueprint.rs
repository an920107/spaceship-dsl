use std::marker::PhantomData;

use crate::spaceship::blueprint::{
    module::{
        HasModule, No, Yes,
        bridge::{Bridge, BridgeType},
        engine::{Engine, EngineType},
        life_support::{LifeSupport, LifeSupportType},
        reactor::{Reactor, ReactorType},
        sensors::{Sensors, SensorsType},
        shield::{Shield, ShieldType},
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
    reactors: Option<Vec<ReactorType>>,
    engine: Option<EngineType>,
    life_support: Option<LifeSupportType>,
    bridge: Option<BridgeType>,
    shield: Option<ShieldType>,
    sensors: Option<SensorsType>,
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
        T: Reactor<N, FR, AR> + Into<ReactorType>,
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
        T: Engine + Into<EngineType>,
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
        T: LifeSupport + Into<LifeSupportType>,
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
        T: Bridge + Into<BridgeType>,
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
        T: Shield<FR, AR> + Into<ShieldType>,
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
        T: Sensors + Into<SensorsType>,
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
