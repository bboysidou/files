pub fn tokenize_exec(exec: &str) -> Option<Vec<String>> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let (mut quoted, mut started) = (false, false);
    let mut chars = exec.chars();
    while let Some(character) = chars.next() {
        match (quoted, character) {
            (true, '"') => quoted = false,
            (true, '\\') => current.push(chars.next()?),
            (false, '"') => (quoted, started) = (true, true),
            (false, ' ' | '\t') if started => {
                tokens.push(std::mem::take(&mut current));
                started = false;
            }
            (false, ' ' | '\t') => {}
            _ => {
                current.push(character);
                started = true;
            }
        }
    }
    if quoted {
        return None;
    }
    if started {
        tokens.push(current);
    }
    Some(tokens)
}
