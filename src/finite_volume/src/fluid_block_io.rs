use std::{path::Path, collections::HashMap};

use crate::{fluid_block::FluidBlock, flow::FlowStates};
use common::{DynamicResult, vector3::Vector3, number::Real};
use grid::{cell::CellShape, interface::InterfaceShape, Vertex, Id, Interface, Cell, Block, block::{GridFileType, write_block}};

/// Light weight copy of vertex geometric data
pub struct VertexIO {
    pos: Vector3,
    id: usize,
}
impl Id for VertexIO {
    fn id(&self) -> usize {
        self.id
    }
}

impl Vertex for VertexIO {
    fn pos(&self) -> &Vector3 {
        &self.pos
    }
}

impl VertexIO {
    pub fn new(x: Real, y: Real, z: Real, id: usize) -> VertexIO {
        VertexIO{pos: Vector3{x, y, z}, id}
    }
}

/// Light weight copy of interface geometric data
#[derive(Clone)]
pub struct InterfaceIO {
    id: usize,
    shape: InterfaceShape,
    vertex_ids: Vec<usize>,
}

impl Id for InterfaceIO {
    fn id(&self) -> usize {
        self.id
    }
}

impl Interface for InterfaceIO {
    fn shape(&self) -> &InterfaceShape {
        &self.shape
    }

    fn vertex_ids(&self) -> &Vec<usize> {
        &self.vertex_ids
    }
}

/// Light weight copy of cell geometric data
pub struct CellIO {
    id: usize,
    vertex_ids: Vec<usize>,
    interface_ids: Vec<usize>,
    shape: CellShape, 
}

impl Id for CellIO {
    fn id(&self) -> usize {
        self.id
    }
}

impl Cell for CellIO {
    fn shape(&self) -> &CellShape {
        &self.shape
    }

    fn vertex_ids(&self) -> &Vec<usize> {
        &self.vertex_ids
    }

    fn interface_ids(&self) -> Vec<usize> {
        self.interface_ids.clone()
    }
}


/// Provides functionality to read/write fluid blocks.
/// Each [`FluidBlockIO`] object has a reference to 
/// a single [`FluidBlock`]. It doesn't assume that the device
/// the data is on can write to the file system (i.e. it could
/// be on the GPU). Thus it copies the data.
/// This is wastefull when the device can write to the file system. 
/// But this is an optimisation for the future.
pub struct FluidBlockIO<'a> {
    fluid_block: &'a FluidBlock,
    flow_states: FlowStates,
    vertices: Vec<VertexIO>,
    interfaces: Vec<InterfaceIO>,
    cells: Vec<CellIO>,
    dimensions: u8,
    id: usize,
}

impl<'a> FluidBlockIO<'a> {
    pub fn new(fluid_block: &'a FluidBlock) -> FluidBlockIO<'a> {
        let vertices = Vec::with_capacity(fluid_block.vertices().len());
        let interfaces = Vec::with_capacity(fluid_block.interfaces().len());
        let cells = Vec::with_capacity(fluid_block.cells().len());
        let flow_states = FlowStates::with_capacity(fluid_block.cells().len());
        let dimensions = fluid_block.dimensions();
        let id = fluid_block.id();
        let mut fluid_block_io = FluidBlockIO{
            fluid_block: &fluid_block, flow_states, vertices, interfaces, cells, dimensions, id
        };
        fluid_block_io.copy_interfaces();
        fluid_block_io.copy_cells();
        fluid_block_io
    }

    pub fn write_fluid_block(&mut self, path: &Path) -> DynamicResult<()> {
        self.copy_flow_state();
        self.copy_vertex_positions();
        self.write_to_file(path)?;
        Ok(())
    }

    pub fn id(&self) -> usize {
        self.id
    }

    fn copy_flow_state(&mut self) {
        self.flow_states = self.fluid_block.cells().flow_states().clone();
    }

    fn copy_vertex_positions(&mut self) {
        self.vertices.clear();
        let vertices = self.fluid_block.vertices();
        for i_vtx in 0 .. vertices.len() {
            self.vertices.push(VertexIO{
                pos: Vector3{
                    x: vertices.x[i_vtx],
                    y: vertices.y[i_vtx],
                    z: vertices.z[i_vtx]
                },
                id: i_vtx
            }) 
        }
    }

    fn copy_interfaces(&mut self) {
        self.interfaces.clear();
        let interfaces = self.fluid_block.interfaces();
        for i_face in 0 .. interfaces.len() {
            self.interfaces.push(InterfaceIO{
                id: i_face,
                shape: interfaces.shape()[i_face],
                vertex_ids: interfaces.vertices()[i_face].to_vec(),
            });       
        }
    }

    fn copy_cells(&mut self) {
        self.cells.clear();
        let cells = self.fluid_block.cells();
        for i_cell in 0 .. cells.len() {
            self.cells.push(CellIO{
                id: i_cell,
                vertex_ids: cells.vertices()[i_cell].to_vec(),
                interface_ids: cells.interfaces()[i_cell].to_vec(),
                shape: cells.shape()[i_cell],
            });
        }
    }

    fn write_to_file(&self, path: &Path) -> DynamicResult<()> {
        let mut file_path = path.to_path_buf();
        let ext = GridFileType::Native.extension();
        file_path.set_file_name(format!("blk{:0>4}.{}", self.id, ext));
        write_block(self, &file_path)?; 
        Ok(())
    }
}

impl<'a> Block<VertexIO, InterfaceIO, CellIO> for FluidBlockIO<'a> {
    fn vertices(&self) -> &Vec<VertexIO> {
        &self.vertices
    }

    fn interfaces(&self) -> &Vec<InterfaceIO> {
        &self.interfaces
    }

    fn cells(&self) -> &Vec<CellIO> {
        &self.cells
    }

    fn boundaries(&self) -> &HashMap<String, Vec<usize>> {
        todo!()
    }

    fn dimensions(&self) -> u8 {
        self.dimensions
    }

    fn id(&self) -> usize {
        self.id
    }
}

pub fn read_fluid_block(path: &Path) -> DynamicResult<()> {
    todo!()
}
