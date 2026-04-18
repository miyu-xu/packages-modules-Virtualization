//! Stub device tree types for host targets (Unix implementation uses full libfdt in AOSP).

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FdtError {
    NotFound,
    Internal,
}

impl std::fmt::Display for FdtError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for FdtError {}

pub struct Fdt;

impl Fdt {
    pub fn create_empty_tree(_buf: &mut [u8]) -> Result<&mut Fdt, FdtError> {
        Err(FdtError::Internal)
    }

    pub fn from_mut_slice(_buf: &mut [u8]) -> Result<&mut Fdt, FdtError> {
        Err(FdtError::Internal)
    }

    pub unsafe fn unchecked_from_slice(_buf: &[u8]) -> &Fdt {
        static F: Fdt = Fdt;
        &F
    }
}
