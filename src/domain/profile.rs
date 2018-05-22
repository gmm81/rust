pub struct Profile {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub age: i8,
    pub sex: bool,
}

impl Profile {
    pub fn get_full_name(self) -> String {
        self.last_name + " " + &self.first_name
    }
}