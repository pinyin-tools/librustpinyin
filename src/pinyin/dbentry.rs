pub struct DBEntry {
    pub sinogram: String,
    pub frequency: uint
}

impl DBEntry {
    pub fn new(sinogram:String, frequency:uint) -> DBEntry {
        DBEntry {
            sinogram: sinogram,
            frequency: frequency
        }
    }
}
