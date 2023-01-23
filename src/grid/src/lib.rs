use std::collections::HashMap;

use cell::CellShape;
use common::vector3::Vector3;
use interface::InterfaceShape;

/// Handles geometric vertices
pub mod vertex;

/// Handles geometric interfaces
pub mod interface;

/// Handles geometric cells
pub mod cell;

/// Hanles unstructured grids
pub mod block;

mod su2;

mod geom_calc;

pub trait Cell {
    fn shape(&self) -> &CellShape;
    fn vertex_ids(&self) -> &Vec<usize>;
    fn id(&self) -> usize;
}

pub trait Interface {
    fn shape(&self) -> &InterfaceShape;
    fn vertex_ids(&self) -> &Vec<usize>;
    fn id(&self) -> usize;
}

pub trait Vertex {
    fn pos(&self) -> &Vector3;
    fn id(&self) -> usize;
}

/// Interface for interrorgating a block. This helps with abstracting
/// the writing of different types of blocks to file.
pub trait Block<V, I, C>
    where V: Vertex, I: Interface, C: Cell 
{
    fn vertices(&self) -> &Vec<V>;
    fn interfaces(&self) -> &Vec<I>;
    fn cells(&self) -> &Vec<C>;
    fn boundaries(&self) -> &HashMap<String, Vec<usize>>;
    fn dimensions(&self) -> u8;
    fn id(&self) -> usize;
}
