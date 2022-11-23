use super::entity;
use super::state;
use mongodb;
use rocket::futures::StreamExt;
use rocket::serde::json;

#[get("/")]
pub async fn index(db: &rocket::State<mongodb::Client>) -> json::Json<Vec<entity::Object>> {
    let mut cursor = db.database("objects")
    .collection::<entity::Object>("objects")
    .find(None, None).await.expect("failed querying objects");
    let mut result_list = Vec::<entity::Object>::new();

    while let Some(object) = cursor.next().await {
        match object {
            Ok(retrieved) => {
                result_list.push(retrieved);
            }
            Err(_) => {}
        };
    }

    return json::Json(result_list);
}

#[post("/", data = "<object>")]
pub async fn post_object<'a>(
    db: &rocket::State<mongodb::Client>,
    data_holder: &rocket::State<state::DataHolder>,
    object: json::Json<entity::Object>,
) -> Result<json::Json<Vec<entity::Object>>, String> {
    db.database("objects")
    .collection::<entity::DbObject>("objects")
    .insert_one(entity::DbObject::from_object(&object.0), None)
    .await.expect("error when trying to insert object into db");

    match data_holder.data.write() {
        Ok(mut mutable) => {
            mutable.push(object.0);
        }
        Err(err) => return Err(err.to_string()),
    };

    match data_holder.data.read() {
        Ok(mutable) => {
           return Ok(json::Json(mutable.to_owned())); 
        }
        Err(err) => return Err(err.to_string()),
    };
}
