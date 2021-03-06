mod struct_scheme;

use mongodb::{
    Client, 
    Collection,
    results::{
        DeleteResult
    },
    error::Error
};


// collection Name
const Rooms:&str = "rooms";

// Mongo DB Conection
fn mongo_connection(coll:&str)->Result<Collection, Error>{
    let client = Client::with_uri_str("mongodb://localhost:27017")?;
    let db = client.database("edge");
    let collection = db.collection(coll);
    Ok(collection)
}


// GET Collection 
fn doc_coll(coll_name:&str)->Collection{
    let collection = match mongo_connection(coll_name){
        Ok(coll) => coll,
        Err(_) => panic!("Error in collection")
    };
    collection
}



// Get All Data
pub fn all()->Result<Vec<Todo>,Error>{
    let collection = doc_coll(TODO);
    let cursor = collection.find(None, None).unwrap();


    cursor.map(|result|match result{
        
    Ok(doc) => match bson::from_bson(Document(doc)) {
        Ok(result_model) => Ok(result_model),
        Err(_) => panic!("document not found"),
    },
    Err(err) => Err(err),
        })
.collect::<Result<Vec<Todo>, Error>>()
}


// Insert One Record
pub fn insert(todo:Todo)->Result<ObjectId,Error>{

    let collection = doc_coll(TODO);
    let insertable = InsertableTodo::from_todo(todo.clone());
    match bson::to_bson(&insertable){
        Ok(model_bson)=>match model_bson {
            Document(model_doc)=>{
                match collection.insert_one(model_doc, None) {
                    Ok(res)=>match res.inserted_id {
                        res=> match bson::from_bson(res) {
                            Ok(res) => Ok(res),
                            Err(_) => panic!("Error"),
                        },
                        _ => panic!("Error")
                    },
                    Err(err) => Err(err)
                }
            }
            _ => panic!("Not inserted")
        },
        Err(_) => panic!("Not Found !!! or inserted")
    }
}

pub fn delete_collection(id: ObjectId)-> Result<DeleteResult, Error>{
    let collection = doc_coll(TODO);
    collection.delete_one(doc! {"_id": id}, None)
}

pub fn update_collection(id: ObjectId, todo: Todo) -> Result<Todo, Error> {
    let collection = doc_coll(TODO);
    let mut new_todo = todo.clone();
    new_todo.id = Some(id.clone());
    match bson::to_bson(&new_todo) {
        Ok(model_bson) => match model_bson {
            Document(model_doc) => {
                match collection.replace_one(doc! {"_id": id}, model_doc, None)
                {
                    Ok(_) => Ok(new_todo),
                    Err(err) => Err(err),
                }
            }
            _ => panic!("Error insert document"),
        },
        Err(_) => panic!("Error insert document"),
    }
}