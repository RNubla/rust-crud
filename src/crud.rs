pub struct Crud<T> {
    pub data: Vec<T>,
}
pub trait CrudTrait<T> {
    fn read_all_data(&self) -> &Vec<T>;
    fn create(&mut self, value: T);
    // fn read(&mut self, id: i64 );
    // fn update(&mut self);
    // fn delete(&mut self);
}

impl<T> CrudTrait<T> for Crud<T> {
    fn read_all_data(&self) -> &Vec<T> {
        return &self.data;
    }
    fn create(&mut self, value: T) {
        self.data.push(value);
    }
}
