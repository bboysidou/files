use crate::core::constants::applications::PATH_SEPARATOR;
use crate::core::constants::phones::{MTP_HOST_SEPARATOR, MTP_LABEL_SEPARATOR};

pub fn mtp_label(host: &str) -> String {
    let host = host.trim_matches(PATH_SEPARATOR);
    let named = host
        .rsplit_once(MTP_HOST_SEPARATOR)
        .map_or(host, |(named, _)| named);
    named.replace(MTP_HOST_SEPARATOR, MTP_LABEL_SEPARATOR)
}
