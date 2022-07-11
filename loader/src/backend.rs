pub(crate) trait Backend {
    type Error;

    fn is_installed() -> bool;
    fn load() -> Result<Self, Self::Error> where Self: Sized;
}