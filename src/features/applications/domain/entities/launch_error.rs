#[derive(Debug)]
pub enum LaunchError {
    NoTerminal,
    BadCommand,
    Spawn(String),
}
