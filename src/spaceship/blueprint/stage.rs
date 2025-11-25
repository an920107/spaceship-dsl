use std::marker::PhantomData;

use crate::spaceship::blueprint::module::HasModule;

pub struct InitialStage;
pub struct CoreModulesInstallationStage<R: HasModule, E: HasModule, L: HasModule, B: HasModule>(
    PhantomData<(R, E, L, B)>,
);
pub struct OptionalModulesInstallationStage<H: HasModule, E: HasModule>(PhantomData<(H, E)>);
pub struct FinalizationStage;

pub trait Stage {}

impl Stage for InitialStage {}

impl<R: HasModule, E: HasModule, L: HasModule, B: HasModule> Stage
    for CoreModulesInstallationStage<R, E, L, B>
{
}

impl<H: HasModule, E: HasModule> Stage for OptionalModulesInstallationStage<H, E> {}

impl Stage for FinalizationStage {}
