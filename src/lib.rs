pub type Day = i8;

pub mod load;
pub use load::get_data;

pub mod solutions;
pub use solutions::solve;
