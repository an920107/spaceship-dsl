use spaceship::spaceship::blueprint::{
    Blueprint,
    module::{
        bridge::CommandBridge, engine::IonEngine, life_support::AdvancedLifeSupport,
        reactor::AntimatterReactor, sensors::BasicSensors, shield::PhaseShield,
    },
};

fn main() {
    let bp = Blueprint::new();
    let bp = bp.set_frame();
    let bp = bp.add_reactor(AntimatterReactor::new());
    let bp = bp.add_engine(IonEngine::new());
    let bp = bp.add_life_support(AdvancedLifeSupport::new());
    // let bp = bp.add_reactor(FusionReactor::new());
    let bp = bp.add_bridge(CommandBridge::new());
    let bp = bp.lock_core_modules();
    let bp = bp.add_sensors(BasicSensors::new());
    let bp = bp.add_shield(PhaseShield::new());
    // let bp = bp.add_shield(MagneticShield::new());
    let bp = bp.finalize();

    bp.print_spec();
}
