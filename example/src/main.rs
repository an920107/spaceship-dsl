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
            reactor AntimatterReactor,
            engine IonEngine,
            life_support AdvancedLifeSupport,
            bridge CommandBridge,
        }
        optional {
            shield PhaseShield,
        }
    );

    spaceship.print_spec();
}
