use serde::de::DeserializeOwned;
use serde::Serialize;
use core::fmt::Debug;
use mongodb::bson;
use mongodb::bson::{doc, Bson, oid::ObjectId};
use crate::utils::get_struct_name;
use super::DB;

fn get_collection<T>() -> mongodb::sync::Collection<mongodb::bson::Document> {
    let name = &get_struct_name::<T>()[..];
    DB.collection(name)
}

pub fn create
    <T: Serialize + DeserializeOwned + Unpin + Debug+ Sync + std::marker::Send + Clone>
    (obj: &T) -> Result<T, mongodb::error::Error>
{
    let collection = get_collection::<T>();
    let serialized_data = bson::to_bson(&obj)?;
    let document = serialized_data.as_document().unwrap();
    collection.insert_one(document.to_owned(), None)?;
    Ok((*obj).clone())
}

pub fn search
    <T: Serialize + DeserializeOwned + Unpin + Debug+ Sync + std::marker::Send>
    (keyword: Option<String>,
     start: Option<u32>,
     limit: Option<i32>)
     -> Result<Vec<T>, mongodb::error::Error>
{
    let collection = get_collection::<T>();
    let mut pipelines = Vec::new();
    if let Some(v) = keyword {
        let stage = doc! {
            "$match": {
                "$text": {
                    "$search": v
                }                
            }
        };
        pipelines.push(stage);
    }
    if let Some(v) = start {
        let stage = doc! {"$skip": v};
        pipelines.push(stage);
    }
    if let Some(mut v) = limit {
        if v > 20 {
            v = 20;
        }
        let stage = doc! {"$limit": v};
        pipelines.push(stage);
    } else {
        let stage = doc! {"$limit": 10};
        pipelines.push(stage);
    }
    let cursor = collection.aggregate(pipelines, None)?;
    let bson_to_t = |d| bson::from_bson(Bson::Document(d)).unwrap();
    let documents: Vec<_> = cursor.map(|doc| bson_to_t(doc.unwrap())).collect();

    Ok(documents)
}

pub fn update
    <T: Clone + Serialize + DeserializeOwned + Unpin + Debug+ Sync + std::marker::Send>
    (obj: &T)
     ->  Result<T, mongodb::error::Error>
{
    let collection = get_collection::<T>();
    let serialized_data = bson::to_bson(&obj)?;
    let document = serialized_data.as_document().unwrap();
    let update_doc = doc! {
        "$set": document
    };
    collection.update_one((*document).clone(), update_doc, None)?;
    Ok((*obj).clone())
}

pub fn delete
    <T: Clone + Serialize + DeserializeOwned + Unpin + Debug+ Sync + std::marker::Send>
    (id: &ObjectId)
     ->  Result<ObjectId, mongodb::error::Error>
{
    let collection = get_collection::<T>();
    collection.delete_one(doc! {"_id": id}, None)?;
    Ok((*id).clone())
}

pub fn get
    <T: Clone + Serialize + DeserializeOwned + Unpin + Debug+ Sync + std::marker::Send,
     U: Clone + From<U>>
    (id: U)
     ->  Result<T, mongodb::error::Error> where Bson: From<U>
{
    let collection = get_collection::<T>();
    let rs = collection.find_one(doc! {"_id": id}, None)?;
    Ok(bson::from_bson(Bson::Document(rs.unwrap())).unwrap())
}

pub fn search_relate
    <T: Serialize + DeserializeOwned + Unpin + Debug+ Sync + std::marker::Send>
    (id: ObjectId, field: String, skip: Option<u32>, limit: Option<i32>)
     -> Result<Vec<bson::Document>, mongodb::error::Error>
{
    let collection = get_collection::<T>();
    let match_field = format!("${}", field);
    let mut pipelines = Vec::new();
    pipelines.push(doc!{
        "$match": {
             match_field: id
        }
    });
    if let Some(v) = skip { 
        let stage = doc! {"$skip": v};
        pipelines.push(stage);
    }
    if let Some(mut v) = limit {
        if v > 20 {
            v = 20;
        }
        let stage = doc! {"$limit": v};
        pipelines.push(stage);
    } else {
        let stage = doc! {"$limit": 10};
        pipelines.push(stage);
    }
    
    let cursor = collection.aggregate(pipelines, None)?;
    let documents: Vec<_> = cursor.map(|doc| doc.unwrap()).collect();
    Ok(documents)
}
