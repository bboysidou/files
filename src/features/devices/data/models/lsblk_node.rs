use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct LsblkReport {
    pub blockdevices: Vec<LsblkNode>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct LsblkNode {
    pub path: String,
    pub pkname: Option<String>,
    pub label: Option<String>,
    pub mountpoint: Option<String>,
    pub size: Option<String>,
    pub fstype: Option<String>,
    #[serde(default)]
    pub hotplug: bool,
    #[serde(default)]
    pub rm: bool,
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(default)]
    pub children: Vec<LsblkNode>,
}
