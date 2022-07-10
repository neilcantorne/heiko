pub(crate) struct Library {

}

impl Library {
    pub unsafe fn load(name: &str) -> Result<Self, crate::library::LibraryError> {
        todo!();
    }

    pub unsafe fn get_fn<T>(&self, symbol: &str) -> Option<T>{
        todo!();
    }
}

