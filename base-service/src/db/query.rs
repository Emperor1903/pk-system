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
    (obj: &T) -> Result<Bson, mongodb::error::Error>
{
    let collection = get_collection::<T>();
    let serialized_data = bson::to_bson(&obj)?;
    let document = serialized_data.as_document().unwrap();
    let rs = collection.insert_one(document.to_owned(), None)?;
    Ok(rs.inserted_id)
}

fn magic_update(document: &mongodb::bson::Document) ->  mongodb::bson::Document {
    doc!{"$set": document}
}

pub fn update
    <T: Clone + Serialize + DeserializeOwned + Unpin + Debug+ Sync + std::marker::Send,
     U: Clone>
    (id: U, obj: &T)
     ->  Result<U, mongodb::error::Error> where Bson: From<U>
{
    let collection = get_collection::<T>();
    let serialized_data = bson::to_bson(&obj)?;
    let document = serialized_data.as_document().unwrap();
    collection.update_one(doc!{"_id": id.clone()}, magic_update(document), None)?;
    Ok(id)
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

pub fn search
    <T: Serialize + DeserializeOwned + Unpin + Debug+ Sync + std::marker::Send>
    (keyword: Option<String>,
     ids: Option<Vec<ObjectId>>,
     fields: Option<Vec<String>>,
     skip: Option<i64>,
     limit: Option<i64>,
     start_time: Option<i64>,
     end_time: Option<i64>)
     -> Result<bson::Document, mongodb::error::Error>
{
    let collection = get_collection::<T>();
    let mut pipelines = Vec::new();
    if let Some(v) = ids {
        if let Some(u) = fields {
            for i in 0..v.len() {
                pipelines.push(doc!{
                    "$match": {
                        u[i].clone(): v[i].clone()
                    }
                });
            }
        }
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
    let mut page_v = Vec::new();
    if let Some(v) = skip { 
        let stage = doc! {"$skip": v};
        page_v.push(stage);
    }
    if let Some(mut v) = limit {
        if v > 1000 {
            v = 1000;
        }
        let stage = doc! {"$limit": v};
        page_v.push(stage);
    } else {
        let stage = doc! {"$limit": 1000};
        page_v.push(stage);
    }
    
    let facet = doc! {
        "$facet": {
            "total": [{"$count": "count"}],
            "data": page_v
        }
    };

    pipelines.push(facet);
    
    let cursor = collection.aggregate(pipelines, None)?;

    let documents: Vec<_> = cursor.map(|doc| doc.unwrap()).collect();
    Ok(documents[0].clone())
}
