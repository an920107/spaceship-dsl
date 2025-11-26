use spaceship::{
    blueprint::module::{
        bridge::CommandBridge, engine::IonEngine, life_support::AdvancedLifeSupport,
        reactor::AntimatterReactor, sensors::AdvancedSensors, shield::PhaseShield,
    },
    create_spaceship,
};

fn main() {
    let spaceship = create_spaceship!(
        core {
            engine IonEngine,
            reactor AntimatterReactor,
            life_support AdvancedLifeSupport,
            bridge CommandBridge,
        }
        optional {
            shield PhaseShield,
            sensors AdvancedSensors,
        }
    );

    spaceship.print_spec();
}
