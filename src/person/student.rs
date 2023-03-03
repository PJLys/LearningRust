use crate::date::date::Date;
use super::Person;
#[derive(Clone, Default)]
pub struct Student {
    id : Option<i32>,
    p : Person,
    uni: String,
    major : String
}

impl Student {
    pub fn new(
        name: &String, bday: &Date, height: f32,
        uni : &String,
        major : &String) -> Result<Student, &'static str> {
            
        let p = Person::new(name,bday,height)?;

        
        Ok(Student {
            id : None,
            p,
            uni: uni.to_string(),
            major: major.to_string()
        })
    }

    pub fn set_id(&mut self, id: i32) {
        self.id = Some(id);
    }

    pub fn get_id(&self) -> Option<i32> {
        self.id
    }
}

