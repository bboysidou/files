use crate::core::constants::operations::BULK_NUMBER_TOKEN;

pub fn plan_bulk_rename(names: &[String], find: &str, template: &str) -> Vec<String> {
    let width = names.len().to_string().len();
    names
        .iter()
        .enumerate()
        .map(|(index, name)| {
            let number = format!("{:0width$}", index + 1);
            let replacement = template.replace(BULK_NUMBER_TOKEN, &number);
            if !find.is_empty() {
                return name.replace(find, &replacement);
            }
            if template.is_empty() {
                return name.clone();
            }
            format!("{replacement}{}", extension_of(name))
        })
        .collect()
}

fn extension_of(name: &str) -> &str {
    match name.rfind('.') {
        Some(dot) if dot > 0 => &name[dot..],
        _ => "",
    }
}
