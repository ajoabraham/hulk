
use olap::common::*;
use olap::column::*;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Cube{
    pub name: String,
    pub row_count: usize,
    pub columns: HashMap<String,Column>
}

// type Value = Option;
impl Cube{
    pub fn new(name: String, row_count: usize) -> Cube{
        Cube { name: name, columns: HashMap::new(), row_count: row_count }
    }

    pub fn new_col(&mut self,  
            name: String, 
            data_type: DataType, 
            column_type: ColType, 
            agg: Option<AggMode>){

        self.columns.insert(
            name.clone(),
            Column {
                name: name,
                data_type: data_type,
                column_type: column_type,
                agg: agg,
                store: Vec::with_capacity(self.row_count)
            }
        );

    }

    pub fn get_col<'a>(&'a self, name: &'a str) -> Option<&'a Column> {
        self.columns.get(name)
    }

    pub fn add_row(&mut self, name: String, row: Record){
        match self.columns.get_mut(&name) {
            Some(column) => column.store.push(row),
            None => panic!("Column {} does not exists", name)
        }
    }

}