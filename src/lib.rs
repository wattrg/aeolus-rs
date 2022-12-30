pub mod util;
pub mod numerical_methods;

#[allow(non_snake_case)]
pub mod grid;

pub mod gas;

/// Short hand for returning a result with some generic `Ok` type
/// and a dynamic `Err` type
pub type DynamicResult<T> = Result<T, Box<dyn std::error::Error>>;
