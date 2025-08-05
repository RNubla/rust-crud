pub struct Crud<T> {
    pub data: Vec<T>,
}
pub trait HasId {
    fn id(&self) -> i64;
}

pub trait CrudTrait<T> {
    fn read_all_data(&self) -> &Vec<T>;
    fn create(&mut self, value: T);
    fn read_item(&self, id: i64) -> Option<&T>;
    // fn update(&mut self);
    fn delete(&mut self, id: i64);
}

impl<T: HasId> CrudTrait<T> for Crud<T> {
    fn read_all_data(&self) -> &Vec<T> {
        return &self.data;
    }
    fn read_item(&self, id: i64) -> Option<&T> {
        self.data.iter().find(|item| item.id() == id)
    }
    fn delete(&mut self, id: i64) {
        let exist = self.read_item(id).is_some();
        if exist {
            self.data.retain(|item| item.id() != id);
        } else {
            println!("Item with id {id} does not exist");
        }
    }
    fn create(&mut self, value: T) {
        self.data.push(value);
    }
}
