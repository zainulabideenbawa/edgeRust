use bson::DateTime;

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct Rooms{
    #[serde(rename = "_id")] 
    pub id:Option<ObjectId>,
    pub roomId:Option<String>,
    pub ip:Option<String>,
    pub port:Option<String>,
    pub vituralPort:Option<String>,
    pub status:Option<bool>
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct InsertableRoom{
    pub roomId:Option<String>,
    pub ip:Option<String>,
    pub port:Option<String>,
    pub vituralPort:Option<String>,
    pub status:Option<bool>
}

impl InsertableRoom{
    pub fn from_rooms(room:Rooms)->InsertableRoom{
        InsertableRooms{
            roomId:room.roomId,
            ip:room.ip,
            port:room.port,
            vituralPort:room.vituralPort,
            status:room.status
        }
    }
}


#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct Logs{
    #[serde(rename = "_id")] 
    pub id:Option<ObjectId>,
    pub roomId:Option<String>,
    pub userName:Option<String>,
    pub applinceName:Option<String>,
    pub status:Option<bool>,
    pub dateTime:Option<DateTime>
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct InsertableLog{
    pub roomId:Option<String>,
    pub userName:Option<String>,
    pub applinceName:Option<String>,
    pub status:Option<bool>,
    pub dateTime:Option<DateTime>
}

impl InsertableLog{
    pub fn from_log(log:Logs)->InsertableLog{
        InsertableRooms{
            roomId:log.roomId,
            userName:log.userName,
            applinceName:log.applinceName,
            status:log.status,
            dateTime:log.dateTime
        }
    }
}



#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct Logs{
    #[serde(rename = "_id")] 
    pub id:Option<ObjectId>,
    pub roomId:Option<String>,
    pub userName:Option<String>,
    pub applinceName:Option<String>,
    pub status:Option<bool>,
    pub dateTime:Option<DateTime>
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct InsertableLog{
    pub roomId:Option<String>,
    pub userName:Option<String>,
    pub applinceName:Option<String>,
    pub status:Option<bool>,
    pub dateTime:Option<DateTime>
}

impl InsertableLog{
    pub fn from_log(log:Logs)->InsertableLog{
        InsertableRooms{
            roomId:log.roomId,
            userName:log.userName,
            applinceName:log.applinceName,
            status:log.status,
            dateTime:log.dateTime
        }
    }
}
