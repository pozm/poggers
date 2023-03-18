/// Memory Protection Flags
pub enum Protections {
    /// If memory can execute in this page ?
    Execute,
    /// If memory can be read and executed
    ExecuteRead,
    /// if memory can be read, write, and executed
    ExecuteReadWrite,
    /// win32 specific
    ExecuteWriteCopy,
    /// No Permissions
    NoAccess,
    /// only read access
    ReadOnly,
    /// can read or write
    ReadWrite,
    /// win32 specific
    WriteCopy,
    /// win32 specific
    TargetInvalid,
    /// win32 specific
    TargerNoUpdate,
    /// invalid protection
    INVALID,
}
#[cfg(windows)]
use windows::Win32::System::Memory::PAGE_PROTECTION_FLAGS;
#[cfg(windows)]
impl Protections {
    /// convert into u32
    pub fn u32(&self) -> u32 {
        match &self {
            Protections::Execute => 0x10,
            Protections::ExecuteRead => 0x20,
            Protections::ExecuteReadWrite => 0x40,
            Protections::ExecuteWriteCopy => 0x80,
            Protections::NoAccess => 0x01,
            Protections::ReadOnly => 0x02,
            Protections::ReadWrite => 0x04,
            Protections::WriteCopy => 0x08,
            Protections::TargetInvalid => 0x40000000,
            Protections::TargerNoUpdate => 0x40000000,
            Protections::INVALID => 0x0,
        }
    }
    /// convert into native version 
    pub fn native(&self) -> PAGE_PROTECTION_FLAGS {
        PAGE_PROTECTION_FLAGS(self.u32())
    }
}
#[cfg(windows)]
impl From<u32> for Protections {
    fn from(value: u32) -> Self {
        match value {
            0x10 => Protections::Execute,
            0x20 => Protections::ExecuteRead,
            0x40 => Protections::ExecuteReadWrite,
            0x80 => Protections::ExecuteWriteCopy,
            0x01 => Protections::NoAccess,
            0x02 => Protections::ReadOnly,
            0x04 => Protections::ReadWrite,
            0x08 => Protections::WriteCopy,
            0x40000000 => Protections::TargetInvalid,
            _ => Protections::INVALID,
        }
    }
}
#[cfg(windows)]
impl Into<u32> for Protections {
    fn into(self) -> u32 {
        self.u32()
    }
}
