extern crate chrono;

pub mod common;
pub mod column;
pub mod cube;

#[cfg(test)]
mod tests {
	use common::*;
    use cube::*;

    #[test]
    fn it_works() {
    	// let t = DataType::INT;
    	// println!("{:?}",t);
     //    assert_eq!(t, DataType::INT);
    }

    #[test]
    fn add_col() {
        let mut cube = Cube::new(String::from("test_cube"),100);
        cube.new_col(
            String::from("_col_"),
             DataType::INT,
             ColType::DIMENSION,
             None
         );
        for i in  0..2000 {
            cube.add_row(String::from("_col_"), Record::INT(i))
        }

        println!("hello from add col");
        println!("{:?}",cube.get_col("_col_"));
        assert_eq!(2 + 2, 4);
    }
}
