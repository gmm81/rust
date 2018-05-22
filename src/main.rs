pub mod domain;

use domain::profile::*;

fn main() {
    let test = Profile {
        id: 1,
        first_name: "George".to_string(),
        last_name: "Maistrenko".to_string(),
        email: "george.maistrenko@mobintegro.com".to_string(),
        age: 37,
        sex: true,
    };
    print!("Profile full name: {}", test.get_full_name())
}