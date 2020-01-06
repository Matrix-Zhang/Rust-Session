use crate::mongo::*;

#[derive(Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Sex {
    Male,
    Female,
}

#[derive(Deserialize, Serialize)]
pub struct Person {
    name: String,
    age: i32,
    sex: Sex,
}

impl Person {
    pub fn new(name: impl Into<String>, age: i32, sex: Sex) -> Self {
        Person {
            name: name.into(),
            age,
            sex,
        }
    }
}

impl MongoModel for Person {
    const COLLECTION_NAME: &'static str = "person";
}
