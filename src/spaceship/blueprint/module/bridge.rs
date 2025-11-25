use std::marker::PhantomData;

#[derive(Clone, Copy)]
pub struct ExplorerBridge(PhantomData<()>);

#[derive(Clone, Copy)]
pub struct CommandBridge(PhantomData<()>);

impl ExplorerBridge {
    pub fn new() -> Self {
        ExplorerBridge(PhantomData)
    }
}

impl CommandBridge {
    pub fn new() -> Self {
        CommandBridge(PhantomData)
    }
}

pub trait BridgeConstraint {}

impl BridgeConstraint for ExplorerBridge {}

impl BridgeConstraint for CommandBridge {}

pub enum Bridge {
    Explorer(ExplorerBridge),
    Command(CommandBridge),
}

impl Into<Bridge> for ExplorerBridge {
    fn into(self) -> Bridge {
        Bridge::Explorer(self)
    }
}
impl Into<Bridge> for CommandBridge {
    fn into(self) -> Bridge {
        Bridge::Command(self)
    }
}

impl Bridge {
    pub fn get_mass(&self) -> i32 {
        match self {
            Bridge::Explorer(_) => 50,
            Bridge::Command(_) => 60,
        }
    }

    pub fn get_power_draw(&self) -> i32 {
        match self {
            Bridge::Explorer(_) => 75,
            Bridge::Command(_) => 75,
        }
    }
}
