use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Describe {
    pub columns: Vec<Column>,
    pub nullable: Vec<Option<bool>>,
    pub parameters: Parameters,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Column {
    pub name: String,
    pub ordinal: usize,
    pub type_info: TypeInfoUnion,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Parameters {
    pub left: Vec<LeftParameter>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum LeftParameter {
    Text(String),
    Obj(LeftParameterObj),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct LeftParameterObj {
    custom: CustomObj,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CustomObj {
    name: String,
    kind: CustomKind,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CustomKind {
    #[serde(rename = "Enum")]
    kind_enum: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum TypeInfoUnion {
    Other(String),
    TypeInfoClass(TypeInfoClass),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct TypeInfoClass {
    custom: Custom,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Custom {
    name: String,
    kind: Kind,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Kind {
    #[serde(rename = "Enum")]
    kind_enum: Vec<String>,
}
