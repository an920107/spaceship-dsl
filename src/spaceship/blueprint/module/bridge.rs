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

pub trait Bridge {
    fn get_type(&self) -> &'static str;

    fn get_slot_cost(&self) -> i32 {
        1
    }

    fn get_mass(&self) -> i32;

    fn get_power_draw(&self) -> i32 {
        75
    }
}

impl Bridge for ExplorerBridge {
    fn get_type(&self) -> &'static str {
        "Explorer"
    }

    fn get_mass(&self) -> i32 {
        50
    }
}

impl Bridge for CommandBridge {
    fn get_type(&self) -> &'static str {
        "Command"
    }

    fn get_mass(&self) -> i32 {
        60
    }
}

pub enum BridgeType {
    Explorer(ExplorerBridge),
    Command(CommandBridge),
}

impl Into<BridgeType> for ExplorerBridge {
    fn into(self) -> BridgeType {
        BridgeType::Explorer(self)
    }
}
impl Into<BridgeType> for CommandBridge {
    fn into(self) -> BridgeType {
        BridgeType::Command(self)
    }
}
