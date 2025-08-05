use crate::crud::HasId;

#[derive(Debug)]
pub struct Person {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub age: i64,
}

pub trait PersonTrait {
    fn say_hello(&self);
}
// impl Person {
//     pub fn say_hello(&self) {
//         println!("Hello There")
//     }
//     pub fn smile(&mut self) {
//         self.is_smiling = true
//     }
// }
impl HasId for Person {
    fn id(&self) -> i64 {
        self.id
    }
}
impl PersonTrait for Person {
    fn say_hello(&self) {
        println!("Hello There")
    }
}
