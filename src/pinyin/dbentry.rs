#[deriving(Clone, Eq, PartialEq, PartialOrd)]
pub struct DbEntry {
    pub frequency: uint,
    pub sinogram: String
}

impl DbEntry {
    pub fn new(sinogram:String, frequency:uint) -> DbEntry {
        DbEntry {
            frequency: frequency,
            sinogram: sinogram
        }
    }
}

impl Ord for DbEntry {
    fn cmp(&self,other: &DbEntry) -> Ordering {
        return self.frequency.cmp(&other.frequency);
    }
}
