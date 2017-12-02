extern crate chrono;

pub mod common;
pub mod column;
pub mod cube;

use olap::cube::*;
use olap::common::Record;

#[derive(Debug)]
pub struct Query<'a>{
	pub cube: &'a Cube,
	select: Vec<String>
}

impl<'a> Query<'a>{
	pub fn select(&mut self, mut sel: Vec<String>) -> &'a Query {
        self.select.append(&mut sel);
        self
    }

    pub fn run(&self) -> Vec<Vec<&Record>> {
    	let mut res = Vec::new();
    	
        if self.cube.columns.values().len() >0  {
            let (_, c) = self.cube.columns.iter().next().unwrap();
            if c.store.len() ==0 {
                return res;
            }            
        }


    	for i in 0..self.cube.row_count {
    		if self.select.len() == 0 {
    			let mut row: Vec<&Record> = Vec::new();
    			for v in self.cube.columns.values() {
    				row.push(&v.store[i])
    			}
    			res.push(row);
    		}
    	}
    	
    	res
    }
}

pub fn olap(cube: &Cube) -> Query {
	Query {
		cube: cube,
		select: Vec::new()
	}
}