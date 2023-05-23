use serde::{Deserialize, Serialize};

use crate::ItemPathBuf;

#[derive(Deserialize, Clone, Debug, PartialEq, Serialize)]
pub struct Component {
    pub name: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "type")]
    pub type_: ComponentType,
    #[serde(default)]
    pub attributes: Vec<ItemPathBuf>,
    #[serde(default)]
    pub default: Option<toml::Value>,
}

#[derive(Deserialize, Clone, Debug, PartialEq, Eq, Serialize)]
pub enum ContainerType {
    Vec,
    Option,
}

#[derive(Deserialize, Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(untagged)]
pub enum ComponentType {
    Item(ItemPathBuf),
    Contained {
        #[serde(rename = "type")]
        #[serde(alias = "container_type")]
        type_: ContainerType,
        element_type: ItemPathBuf,
    },
}
