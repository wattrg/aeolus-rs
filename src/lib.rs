pub mod util;
pub mod numerical_methods;
pub mod grid;
pub mod gas;
pub mod config;
pub mod solvers;

/// Short hand for returning a result with some generic `Ok` type
/// and a dynamic `Err` type
pub type DynamicResult<T> = Result<T, Box<dyn std::error::Error>>;

