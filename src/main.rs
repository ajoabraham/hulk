extern crate rand;

pub mod olap;

use olap::olap as olap;
use olap::cube::Cube;
use olap::common::{ DataType, ColType, Record, AggMode};

use rand::Rng;



fn main() {
	let mut rng = rand::thread_rng();

	let mut c = Cube::new(String::from("hello"), 10);
	c.new_col(String::from("col1"), DataType::INT, ColType::MEASURE, Some(AggMode::SUM));
	c.new_col(String::from("col2"), DataType::TEXT, ColType::DIMENSION, None);

	for i in 0..50 {
		let b: i32;
		if rng.gen() { // random bool
		    b = rng.gen::<i32>();
		}else{
			b = i;
		}
		c.columns.get_mut("col1").unwrap().store.push(Record::INT(b));

		let s: String;
		if i % 2 == 0 {
			s = String::from(format!("test{}",5))
		}else {
		    s = String::from(format!("test{}",10))
		}
		c.columns.get_mut("col2").unwrap().store.push(Record::TEXT(s));
	}

	let q = olap(&c);
	println!(" {:?} ", c);
	println!(" {:?} ", q.run());
    println!("Hello, world!");
}
