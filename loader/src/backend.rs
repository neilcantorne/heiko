pub(crate) trait Backend {
    type Handle;
    fn is_installed() -> bool;
    unsafe fn load() -> Result<Self::Handle, crate::library::LoadError> 
        where Self::Handle: Sized + Drop + Sync;
}