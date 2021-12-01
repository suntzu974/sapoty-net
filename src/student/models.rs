use serde::{Deserialize, Serialize};

#[derive(Debug,Serialize, Deserialize)]
pub struct Student {
    pub ident:String,
    pub firstname: String,
    pub lastname:String,
    pub courses: Vec<Course>,
}
#[derive(Debug,Serialize, Deserialize)]
pub struct Course {
    pub classroom:String,
    pub course:String,
}

impl Default for Student {
    fn default() -> Student {
         Student {
            ident: String::new(),
            firstname: String::new(),
            lastname: String::new(),
            courses:Vec::new(),
        }
    }
}
