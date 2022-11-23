use rocket::serde::{Serialize, Deserialize};

#[derive(Deserialize)]
#[derive(Serialize)]
#[derive(Clone)]
#[serde(crate="rocket::serde")]
pub struct Object {
    pub name: String,
    pub sub: SubObject,
}

#[derive(Deserialize)]
#[derive(Serialize)]
#[derive(Clone)]
#[serde(crate="rocket::serde")]
pub struct SubObject {
    pub name: String,
    pub data: f64,
}


#[derive(Deserialize, Serialize, Clone, Copy)]
#[serde(crate="rocket::serde")]
pub struct DbObject<'a> {
    pub name: &'a str,
    pub sub: DbSubObject<'a>,
}

#[derive(Deserialize, Serialize, Clone, Copy)]
#[serde(crate="rocket::serde")]
pub struct DbSubObject<'a> {
    pub name: &'a str,
    pub data: f64,
}

impl DbObject<'_> {
    pub fn from_object(object: &Object) -> DbObject {
        return DbObject { 
            name: object.name.as_str(),
            sub: DbSubObject::from_sub_object(&object.sub),
        }
    }
}

impl DbSubObject<'_> {
    pub fn from_sub_object(sub_object: &SubObject) -> DbSubObject {
        return DbSubObject { 
            name: sub_object.name.as_str(), 
            data: sub_object.data,
        }
    }
}
