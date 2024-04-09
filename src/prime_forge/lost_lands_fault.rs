#[derive(Debug)]
pub enum LostLostLandsFaultForgedObject {
    TraitNotFound(String),
}

impl std::error::Error for LostLostLandsFaultForgedObject {}

impl std::fmt::Display for LostLostLandsFaultForgedObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LostLostLandsFaultForgedObject::TraitNotFound(forged_trait) => {
                write!(f, "Trait {} not found", forged_trait)
            }
        }
    }
}
