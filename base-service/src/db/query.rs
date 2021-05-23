use serde::de::DeserializeOwned;
use serde::Serialize;
use core::fmt::Debug;
use mongodb::bson;
use mongodb::bson::{doc, Bson, oid::ObjectId};

use crate::api::form::{SearchQuery, RelateSearchQuery};
use crate::utils::get_struct_name;
use super::DB;

fn get_collection<T>() -> mongodb::sync::Collection<mongodb::bson::Document> {
    let name = &get_struct_name::<T>()[..];
    DB.collection(name)
}

pub fn create
    <T: Serialize + DeserializeOwned + Unpin + Debug+ Sync + std::marker::Send + Clone>
    (obj: &T) -> Result<Bson, mongodb::error::Error>
{
    let collection = get_collection::<T>();
    let serialized_data = bson::to_bson(&obj)?;
    let document = serialized_data.as_document().unwrap();
    let rs = collection.insert_one(document.to_owned(), None)?;
    Ok(rs.inserted_id)
}

pub fn search
    <T: Serialize + DeserializeOwned + Unpin + Debug+ Sync + std::marker::Send>
    (keyword: Option<String>,
     start: Option<u64>,
     limit: Option<i64>)
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
    <T: Clone + Serialize + DeserializeOwned + Unpin + Debug+ Sync + std::marker::Send,
     U: Clone + From<U>>
    (id: U) -> Result<U, mongodb::error::Error> where Bson: From<U>
{
    let collection = get_collection::<T>();
    collection.delete_one(doc! {"_id": id.clone()}, None)?;
    Ok(id)
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
    (ids: Vec<ObjectId>, fields: Vec<String>, skip: Option<u64>, limit: Option<i64>, start_time: Option<u64>, end_time: Option<u64>)
     -> Result<Vec<bson::Document>, mongodb::error::Error>
{
    let collection = get_collection::<T>();
    let mut pipelines = Vec::new();
    for i in 0..ids.len() {
        pipelines.push(doc!{
            "$match": {
                fields[i].clone(): ids[i].clone()
            }
        });
    }

    if let Some(v) = skip { 
        let stage = doc! {"$skip": v};
        pipelines.push(stage);
    }
    if let Some(mut v) = limit {
        if v > 1000 {
            v = 1000;
        }
        let stage = doc! {"$limit": v};
        pipelines.push(stage);
    } else {
        let stage = doc! {"$limit": 1000};
        pipelines.push(stage);
    }

    if let Some(v) = start_time {
        pipelines.push(doc!{
            "$match": {
                "start_time": {
                    "$gte": v
                }
            }
        });
    }

    if let Some(v) = end_time {
        pipelines.push(doc!{
            "$match": {
                "end_time": {
                    "$lte": v
                }
            }
        });
    }

    
    
    let cursor = collection.aggregate(pipelines, None)?;

    let documents: Vec<_> = cursor.map(|doc| doc.unwrap()).collect();
    Ok(documents)
}
