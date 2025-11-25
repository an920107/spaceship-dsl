use spaceship::{
    blueprint::module::{
        bridge::CommandBridge, engine::IonEngine, life_support::AdvancedLifeSupport,
        reactor::AntimatterReactor, shield::PhaseShield,
    },
    create_spaceship,
};

fn main() {
    let spaceship = create_spaceship!(
        core {
            add_reactor(AntimatterReactor::new());
            add_bridge(CommandBridge::new());
            add_life_support(AdvancedLifeSupport::new());
            add_engine(IonEngine::new());
        }
        optional {
            add_shield(PhaseShield::new());
        }
    );

    spaceship.print_spec();
}
