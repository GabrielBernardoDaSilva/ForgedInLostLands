use crate::lost_realm::LostRealm;

pub trait ArcaneWeft {
    fn craft(self, lost_realm: &mut LostRealm);
}
