use crate::date::date;

#[derive(Clone, Default)]
pub struct Person {
    bday: date::Date,
    name: String,
    height: f32,
}

impl Person {
    pub fn new(name: &String, bday: &date::Date, height: f32) -> Result<Person, &'static str> {
        if height < 0.0 {
            return Err("Height can't have a negative value!");
        }

        Ok(Person {
            bday: bday.to_owned(),
            name: name.to_string(),
            height,
        })
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_birthday(&self) -> &date::Date {
        &self.bday
    }

    pub fn get_height(&self) -> String {
        self.height.to_string()
    }

    pub fn repr(&self) -> String {
        "Name: ".to_string()
            + &self.name
            + &" Birthday: ".to_string()
            + &self.bday.repr()
            + &" Height: ".to_string()
            + &self.height.to_string()
    }
}