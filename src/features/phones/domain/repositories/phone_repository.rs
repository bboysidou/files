use crate::features::phones::domain::entities::phone::Phone;

pub trait PhoneRepository: Send + Sync {
    fn phones(&self) -> Vec<Phone>;
}
