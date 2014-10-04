pub struct DbEntry {
    pub sinogram: String,
    pub frequency: uint
}

impl DbEntry {
    pub fn new(sinogram:String, frequency:uint) -> DbEntry {
        DbEntry {
            sinogram: sinogram,
            frequency: frequency
        }
    }
}
