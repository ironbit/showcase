use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Default, Hash, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Task {
    #[default]
    Halt,
    Jump,
    Push,
    Pop,
    Dupe,
    Swap,
    Add,
    Equal,
}
