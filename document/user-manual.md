# User Manual: Spaceship Design eDSL

Welcome, future spaceship engineers! This manual will guide you through using our
Domain-Specific Language (DSL) to design and validate your spaceships. Our DSL
is embedded within Rust, providing a natural and powerful way to construct your
blueprints while ensuring safety requirements are met.

## 1. Getting Started: Setup and Installation

To use our spaceship design DSL, you'll need a Rust development environment set
up on your machine. We also recommend using Visual Studio Code (VS Code) with the
Rust Analyzer extension for the best development experience, including compile-time
error highlighting.

### 1.1 Installing Rust and Cargo

If you don't already have Rust and Cargo installed, you can do so using
`rustup`, the Rust toolchain installer:

1. Open your terminal or command prompt.
2. Run the following command:

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

3. Follow the on-screen instructions. This will install `rustc` (the Rust
   compiler) and `cargo` (the Rust package manager).
4. After the installation, you might need to restart your terminal to ensure Cargo is in your PATH.
5. You can verify the installation by checking their versions:

   ```bash
    rustc --version
    cargo --version
   ```

### 1.2 Recommended Editor: Visual Studio Code

For an optimal development experience, we highly recommend using Visual Studio
Code.

1. Download and install VS Code from the official website:
   [https://code.visualstudio.com/](https://code.visualstudio.com/)

### 1.3 Installing the Rust Extension

This extension provide the bundle of rust extensions for better development.

## 2. How to Run Your Program

Once Rust is installed, navigating to your project directory and running your
spaceship design is straightforward:

1. Open your terminal or command prompt.
2. Navigate to the root directory of your Rust project where your `Cargo.toml`
   file is located.
3. Execute your program using Cargo:

   ```bash
   cargo run
   ```

   This command will compile your code (if necessary) and then run the
   executable.

## 3. How to Use Our DSL to Design a Spaceship

Our DSL allows you to define your spaceship's core and optional modules in a
clear and declarative manner. It's designed to feel natural and intuitive for
domain experts, even those without extensive programming backgrounds.

### 3.1 Basic Blueprint Creation

To begin designing a spaceship, you use the `create_spaceship!` macro. This
macro acts as the entry point for your blueprint definition.

You will specify your core and optional modules within two distinct blocks:
`core { ... }` and `optional { ... }`.

Here's the basic structure:

```rust
use spaceship::blueprint::module::{
    // Import all necessary module types
    bridge::CommandBridge, engine::IonEngine, life_support::AdvancedLifeSupport,
    reactor::AntimatterReactor, sensors::AdvancedSensors, shield::PhaseShield,
};
use spaceship::create_spaceship;

fn main() {
    let spaceship = create_spaceship!(
        // Define core modules here
        core {
            engine IonEngine,
            reactor AntimatterReactor,
            life_support AdvancedLifeSupport,
            bridge CommandBridge,
        }
        // Define optional modules here
        optional {
            shield PhaseShield,
            sensors AdvancedSensors,
        }
    );

    spaceship.print_spec();
}
```

### 3.2 Compile-Time Safety Checks (Correct by Construction - CBC)

A key feature of our DSL, achieved through Rust's powerful type system and
macros, is **Correct by Construction (CBC)**. This means that many safety
requirements are checked _at compile-time_ rather than runtime.

**What does this mean for you?**

- **Immediate Feedback**: If your spaceship design violates a safety
  regulation (e.g., installing modules in the wrong order, missing a core
  module, or installing incompatible equipment), your editor (with Rust
  Analyzer) will immediately highlight the error with a red underline.
  The code will _fail to compile_, preventing you from even running an
  illegal blueprint.
- **Reduced Errors**: This approach drastically reduces the chances of shipping
  a faulty design, as critical errors are caught before the program even
  starts.
- **Editor Support**: With the Rust Analyzer extension in VS Code, you'll see
  these errors as you type, providing real-time guidance and preventing
  mistakes. For example, trying to define a blueprint without all core modules
  (engine, reactor, life support, bridge) will result in a clear compile error
  message right in your editor, preventing compilation. Similarly, rules like
  equipment dependency logic (e.g., specific reactor types incompatible with
  certain shield types) are enforced at compile time.

**Example of an illegal design causing a compile error:**

If you were to, for instance, define a `core` block but forget to include an
`engine`, the Rust compiler (and Rust Analyzer) would immediately indicate a
type error because the `create_spaceship!` macro expects a complete set of
core components before allowing an `optional` block.

This compile-time enforcement helps ensure adherence to regulations like:

- `[A-103] Frame Must Be Set First` (handled implicitly by the macro's design)
- `[A-305] Build Phase Integrity` (enforced by the `core` then `optional` structure)
- `[B-209] Core System Integrity` (ensuring all required core modules are present)
- `[B-440] Equipment Dependency Logic` (e.g., preventing incompatible reactor
  and shield types from compiling together).

### 3.3 Dealing with Compile Errors

When you encounter a compile error:

1. **Read the Error Message**: We implment our own error handling logic for clear error message.
2. **Check VS Code**: The red/yellow underlines provided by Rust Analyzer will quickly
   guide you to problematic areas in your `create_spaceship!` definition.
3. **Consult Documentation**: Refer back to the module specifications and safety
   regulations (if provided separately) to understand the constraints.

## 4. How to Use `print_spec` with the Finalized Blueprint

After successfully defining your spaceship blueprint using `create_spaceship!`,
the macro returns a finalized spaceship object. This object contains all the
compiled information about your design. To view the detailed specifications of
your newly designed spaceship, you can call the `print_spec()` method on it.

```rust
fn main() {
    let spaceship = create_spaceship!(
        core { /* ... */ }
        optional { /* ... */ }
    );

    // Call print_spec() on the finalized spaceship object
    spaceship.print_spec();
}
```

### 4.1 `print_spec` Output

The `print_spec()` method will output a text-based (ASCII) visualization of
your spaceship's key specifications to the console. This output is designed to
be clear and easy for engineers to interpret. The specific format follows:

```text
Spaceship Blueprint Specification:
====== Slots ======
Total slots: 10
Used slots: N
Available slots: 10 - N
====== Power ======
Total Power Output: (sum of all reactor power_output)
Total Power Consumption: (sum of all non-reactor module power_draw)
Power Balance: (Total Power Output - Total Power Consumption)
====== Load ======
Total Mass: (sum of frame mass and all module masses)
Total Thrust: (sum of all engine thrust)
Thrust-to-Weight Ratio: (Total Thrust / Total Mass)

```

_Note: `N` in the "Slots" section represents the calculated total number of
slots consumed by your installed modules._

This output provides a comprehensive overview of your spaceship's capabilities
and resource utilization, allowing for quick validation of the design against
performance targets.
