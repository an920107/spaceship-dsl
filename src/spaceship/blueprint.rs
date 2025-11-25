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
    stage::{
        CoreModulesInstallationStage, FinalizationStage, InitialStage,
        OptionalModulesInstallationStage, Stage,
    },
};

pub mod module;
mod stage;

pub struct Blueprint<S: Stage, FR: HasModule, AR: HasModule> {
    reactors: Option<Vec<ReactorType>>,
    engine: Option<EngineType>,
    life_support: Option<LifeSupportType>,
    bridge: Option<BridgeType>,
    shield: Option<ShieldType>,
    sensors: Option<SensorsType>,
    _state: PhantomData<(S, FR, AR)>,
}

impl Blueprint<InitialStage, No, No> {
    pub fn new() -> Self {
        Blueprint {
            reactors: None,
            engine: None,
            life_support: None,
            bridge: None,
            shield: None,
            sensors: None,
            _state: PhantomData,
        }
    }

    pub fn set_frame(self) -> Blueprint<CoreModulesInstallationStage<No, No, No, No>, No, No> {
        Blueprint {
            reactors: Some(Vec::new()),
            engine: None,
            life_support: None,
            bridge: None,
            shield: None,
            sensors: None,
            _state: PhantomData,
        }
    }
}

impl<R: HasModule, E: HasModule, L: HasModule, B: HasModule, FR: HasModule, AR: HasModule>
    Blueprint<CoreModulesInstallationStage<R, E, L, B>, FR, AR>
{
    pub fn add_reactor<T>(
        mut self,
        reactor: T,
    ) -> Blueprint<CoreModulesInstallationStage<Yes, E, L, B>, T::NewFR, T::NewAR>
    where
        T: Reactor<FR, AR> + Into<ReactorType>,
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
            _state: PhantomData,
        }
    }
}

impl<R: HasModule, L: HasModule, B: HasModule, FR: HasModule, AR: HasModule>
    Blueprint<CoreModulesInstallationStage<R, No, L, B>, FR, AR>
{
    pub fn add_engine<T>(
        self,
        engine: T,
    ) -> Blueprint<CoreModulesInstallationStage<R, Yes, L, B>, FR, AR>
    where
        T: Engine + Into<EngineType>,
    {
        Blueprint {
            reactors: self.reactors,
            engine: Some(engine.into()),
            life_support: self.life_support,
            bridge: self.bridge,
            shield: self.shield,
            sensors: self.sensors,
            _state: PhantomData,
        }
    }
}

impl<R: HasModule, E: HasModule, B: HasModule, FR: HasModule, AR: HasModule>
    Blueprint<CoreModulesInstallationStage<R, E, No, B>, FR, AR>
{
    pub fn add_life_support<T>(
        self,
        life_support: T,
    ) -> Blueprint<CoreModulesInstallationStage<R, E, Yes, B>, FR, AR>
    where
        T: LifeSupport + Into<LifeSupportType>,
    {
        Blueprint {
            reactors: self.reactors,
            engine: self.engine,
            life_support: Some(life_support.into()),
            bridge: self.bridge,
            shield: self.shield,
            sensors: self.sensors,
            _state: PhantomData,
        }
    }
}

impl<R: HasModule, E: HasModule, L: HasModule, FR: HasModule, AR: HasModule>
    Blueprint<CoreModulesInstallationStage<R, E, L, No>, FR, AR>
{
    pub fn add_bridge<T>(
        self,
        bridge: T,
    ) -> Blueprint<CoreModulesInstallationStage<R, E, L, Yes>, FR, AR>
    where
        T: Bridge + Into<BridgeType>,
    {
        Blueprint {
            reactors: self.reactors,
            engine: self.engine,
            life_support: self.life_support,
            bridge: Some(bridge.into()),
            shield: self.shield,
            sensors: self.sensors,
            _state: PhantomData,
        }
    }
}

impl<FR: HasModule, AR: HasModule>
    Blueprint<CoreModulesInstallationStage<Yes, Yes, Yes, Yes>, FR, AR>
{
    pub fn lock_core_modules(self) -> Blueprint<OptionalModulesInstallationStage<No, No>, FR, AR> {
        Blueprint {
            reactors: self.reactors,
            engine: self.engine,
            life_support: self.life_support,
            bridge: self.bridge,
            shield: self.shield,
            sensors: self.sensors,
            _state: PhantomData,
        }
    }
}

impl<E: HasModule, FR: HasModule, AR: HasModule>
    Blueprint<OptionalModulesInstallationStage<No, E>, FR, AR>
{
    pub fn add_shield<T>(
        self,
        shield: T,
    ) -> Blueprint<OptionalModulesInstallationStage<Yes, E>, FR, AR>
    where
        T: Shield<FR, AR> + Into<ShieldType>,
    {
        Blueprint {
            reactors: self.reactors,
            engine: self.engine,
            life_support: self.life_support,
            bridge: self.bridge,
            shield: Some(shield.into()),
            sensors: self.sensors,
            _state: PhantomData,
        }
    }
}

impl<H: HasModule, FR: HasModule, AR: HasModule>
    Blueprint<OptionalModulesInstallationStage<H, No>, FR, AR>
{
    pub fn add_sensors<T>(
        self,
        sensors: T,
    ) -> Blueprint<OptionalModulesInstallationStage<H, Yes>, FR, AR>
    where
        T: Sensors + Into<SensorsType>,
    {
        Blueprint {
            reactors: self.reactors,
            engine: self.engine,
            life_support: self.life_support,
            bridge: self.bridge,
            shield: self.shield,
            sensors: Some(sensors.into()),
            _state: PhantomData,
        }
    }
}

impl<H: HasModule, E: HasModule, FR: HasModule, AR: HasModule>
    Blueprint<OptionalModulesInstallationStage<H, E>, FR, AR>
{
    pub fn finalize(self) -> Blueprint<FinalizationStage, FR, AR> {
        Blueprint {
            reactors: self.reactors,
            engine: self.engine,
            life_support: self.life_support,
            bridge: self.bridge,
            shield: self.shield,
            sensors: self.sensors,
            _state: PhantomData,
        }
    }
}
