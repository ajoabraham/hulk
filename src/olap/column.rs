
use olap::common::*;

#[derive(Debug)]
pub struct Column{
	pub data_type: DataType,
	pub name: String,
	pub column_type: ColType,
    pub store: Vec<Record>,
    pub agg: Option<AggMode>
}