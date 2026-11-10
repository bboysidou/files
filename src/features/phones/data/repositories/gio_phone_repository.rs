use crate::features::network::data::datasources::gio_local_path::gio_local_path;
use crate::features::phones::data::datasources::gio_volumes::gio_volumes;
use crate::features::phones::domain::entities::phone::Phone;
use crate::features::phones::domain::repositories::phone_repository::PhoneRepository;

pub struct GioPhoneRepository;

impl PhoneRepository for GioPhoneRepository {
    fn phones(&self) -> Vec<Phone> {
        gio_volumes()
            .into_iter()
            .map(|phone| Phone {
                mount: gio_local_path(&phone.uri),
                ..phone
            })
            .collect()
    }
}
