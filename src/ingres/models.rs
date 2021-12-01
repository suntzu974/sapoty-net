use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Tappent {
    pub i1depot:i32,
    pub i1natmvt:i32,
    pub i4numbl:i32,
    pub i1depote:i32,
    pub i2derlig:i32,
    pub i1nbmaj:i32,
    pub i1nbedit:i32,
    pub vcobs:String,
    pub vcusrbl:String,
    pub chsolde:bool,
    pub didatcre:String,
    pub vcusrcre:String,
    pub didatann:String,
    pub chheurann:String,
    pub vcusrann:String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tapplig {
    pub i1depot:i32,
    pub i1natmvt:i32,
    pub i4numbl:i32,
    pub i2numlig:i32,
    pub chcodi:String,
    pub f8qte:f64,
    pub chunite:String,
    pub f8qteustk:f64,
    pub chunitstk:String,
    pub f8qteustkdd:f64,
    pub chsolde:String,
    pub didatcre:String,
    pub chheurcre:String,
    pub vcusrcre:String,
    pub didatmaj:String,
    pub vcusrmaj:String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseApp {
    #[serde(rename="tappent")]
    pub header: Tappent
    #[serde(rename="tapplig")]
    pub body: Vec<Tapplig>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tnumbl {
    pub i1ent:i32,
    pub i1depot:i32,
    pub i1natmvt:i32,
    pub i4numbl:i32,
}