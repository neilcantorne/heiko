pub(crate) trait Backend {
    fn is_installed() -> bool;
    fn load() -> Result<Self, crate::library::LoadError> where Self: Sized;
}