# Spaceship DSL

A type-safe Domain Specific Language (DSL) for building spaceships in Rust with compile-time validation.

## Environment

> [!NOTE]
> This project is using nightly Rust features (for compile-time slot checking). Make sure to use a nightly toolchain. Although there is a `rust-toolchain.toml` file included in the project, you can also set the override manually by running:
>
> ```bash
> rustup override set nightly
> ```

## Usage

### The `create_spaceship!` Macro

The `create_spaceship!` macro provides a declarative way to build spaceship blueprints with compile-time safety guarantees. It ensures that all required modules are installed and validates slot availability and module compatibility.

### Syntax

```rust
let spaceship = create_spaceship!(
    core {
        <slot_name> <ModuleImpl>,
        ...
    }
    optional {
        <slot_name> <ModuleImpl>,
        ...
    }
)
```

Or without optional modules:

```rust
let spaceship = create_spaceship!(
    core {
        <slot_name> <ModuleImpl>,
        ...
    }
)
```

### Module Types

#### Core Modules (Required)

All core modules must be specified for a valid spaceship:

- **`reactor`** - Power generation system (occupies 3 slots)
  - Example: `AntimatterReactor`, `FusionReactor`
  
- **`engine`** - Propulsion system (occupies 2 slots)
  - Example: `IonEngine`, `WarpDrive`
  
- **`life_support`** - Life support system (occupies 2 slots)
  - Example: `AdvancedLifeSupport`, `BasicLifeSupport`
  
- **`bridge`** - Command and control center (occupies 1 slot)
  - Example: `CommandBridge`, `NavigationBridge`

#### Optional Modules

These modules can be added to enhance spaceship capabilities:

- **`shield`** - Defensive system (occupies 1 slot)
  - Example: `PhaseShield`, `EnergyShield`
  - Note: May require specific reactor types
  
- **`sensors`** - Detection and scanning system (occupies 1 slot)
  - Example: `AdvancedSensors`, `BasicSensors`

### Available Modules

#### Reactors

- `AntimatterReactor`
- `FusionReactor`

#### Engines

- `IonEngine`
- `WarpDrive`

#### Life Support Systems

- `AdvancedLifeSupport`
- `BasicLifeSupport`

#### Bridges

- `CommandBridge`
- `NavigationBridge`

#### Shields

- `PhaseShield`
- `EnergyShield`

#### Sensors

- `AdvancedSensors`
- `BasicSensors`

### Example

```rust
use spaceship::{
    blueprint::module::{
        bridge::CommandBridge, 
        engine::IonEngine, 
        life_support::AdvancedLifeSupport,
        reactor::AntimatterReactor, 
        shield::PhaseShield,
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
```

### Features

- **Type-Safe**: The macro enforces module installation order and validates compatibility at compile time
- **Slot Management**: Automatically tracks and validates slot usage across modules
- **Power Balance**: Ensures power generation meets consumption requirements
- **Staged Construction**: Uses a type-state pattern to prevent invalid configurations

### Blueprint Specification

After creating a spaceship, you can call `.print_spec()` to view:

- Total, used, and available slots
- Power output vs power consumption
- Total mass and thrust
- Thrust-to-weight ratio

### Compile-Time Guarantees

The macro provides several compile-time guarantees:

1. **All core modules must be installed** - Missing any core module will result in a compile error
2. **Slot availability** - Exceeding slot limits is caught at compile time
3. **Module compatibility** - Certain modules may require specific reactors or other dependencies
4. **Installation order** - Core modules must be installed before optional modules
