use rust_i18n::t;

pub fn item_count(count: usize) -> String {
    if count == 1 {
        t!("list.items_one").to_string()
    } else {
        t!("list.items_other", count = count).to_string()
    }
}
