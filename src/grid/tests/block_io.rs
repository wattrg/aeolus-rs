use std::path::PathBuf;
use std::collections::HashMap;

use common::vector3::Vector3;
use grid::{vertex::GridVertex, interface::GridInterface, cell::GridCell, block::*};
use grid::Block;

fn create_block_elements() -> (Vec<GridVertex>, Vec<GridInterface>, Vec<GridCell>, HashMap<String, Vec<usize>>) {
    let vertices = vec![
        GridVertex::new(Vector3{x: 0.0, y: 0.0, z: 0.0}, 0),
        GridVertex::new(Vector3{x: 1.0, y: 0.0, z: 0.0}, 1),
        GridVertex::new(Vector3{x: 2.0, y: 0.0, z: 0.0}, 2),
        GridVertex::new(Vector3{x: 3.0, y: 0.0, z: 0.0}, 3),
        GridVertex::new(Vector3{x: 0.0, y: 1.0, z: 0.0}, 4),
        GridVertex::new(Vector3{x: 1.0, y: 1.0, z: 0.0}, 5),
        GridVertex::new(Vector3{x: 2.0, y: 1.0, z: 0.0}, 6),
        GridVertex::new(Vector3{x: 3.0, y: 1.0, z: 0.0}, 7),
        GridVertex::new(Vector3{x: 0.0, y: 2.0, z: 0.0}, 8),
        GridVertex::new(Vector3{x: 1.0, y: 2.0, z: 0.0}, 9),
        GridVertex::new(Vector3{x: 2.0, y: 2.0, z: 0.0}, 10),
        GridVertex::new(Vector3{x: 3.0, y: 2.0, z: 0.0}, 11),
        GridVertex::new(Vector3{x: 0.0, y: 3.0, z: 0.0}, 12),
        GridVertex::new(Vector3{x: 1.0, y: 3.0, z: 0.0}, 13),
        GridVertex::new(Vector3{x: 2.0, y: 3.0, z: 0.0}, 14),
        GridVertex::new(Vector3{x: 3.0, y: 3.0, z: 0.0}, 15),
    ];

    let interfaces = vec![
        GridInterface::new_from_vertices(&[&vertices[0], &vertices[1]], 0), 
        GridInterface::new_from_vertices(&[&vertices[1], &vertices[5]], 1),
        GridInterface::new_from_vertices(&[&vertices[5], &vertices[4]], 2),
        GridInterface::new_from_vertices(&[&vertices[4], &vertices[0]], 3),
        GridInterface::new_from_vertices(&[&vertices[1], &vertices[2]], 4),
        GridInterface::new_from_vertices(&[&vertices[2], &vertices[6]], 5),
        GridInterface::new_from_vertices(&[&vertices[6], &vertices[5]], 6),
        GridInterface::new_from_vertices(&[&vertices[2], &vertices[3]], 7),
        GridInterface::new_from_vertices(&[&vertices[3], &vertices[7]], 8),
        GridInterface::new_from_vertices(&[&vertices[7], &vertices[6]], 9),
        GridInterface::new_from_vertices(&[&vertices[5], &vertices[9]], 10),
        GridInterface::new_from_vertices(&[&vertices[9], &vertices[8]], 11),
        GridInterface::new_from_vertices(&[&vertices[8], &vertices[4]], 12),
        GridInterface::new_from_vertices(&[&vertices[6], &vertices[10]], 13), 
        GridInterface::new_from_vertices(&[&vertices[10], &vertices[9]], 14),
        GridInterface::new_from_vertices(&[&vertices[7], &vertices[11]], 15), 
        GridInterface::new_from_vertices(&[&vertices[11], &vertices[10]], 16), 
        GridInterface::new_from_vertices(&[&vertices[9], &vertices[13]], 17),
        GridInterface::new_from_vertices(&[&vertices[13], &vertices[12]], 18), 
        GridInterface::new_from_vertices(&[&vertices[12], &vertices[8]], 19),
        GridInterface::new_from_vertices(&[&vertices[10], &vertices[14]], 20),
        GridInterface::new_from_vertices(&[&vertices[14], &vertices[13]], 21),
        GridInterface::new_from_vertices(&[&vertices[11], &vertices[15]], 22),
        GridInterface::new_from_vertices(&[&vertices[15], &vertices[14]], 23),
    ];

    let cells = vec![
        GridCell::new(&[&interfaces[0], &interfaces[1], &interfaces[2], &interfaces[3]], 
                  &[&vertices[0], &vertices[1], &vertices[5], &vertices[4]], 0),
        GridCell::new(&[&interfaces[4], &interfaces[5], &interfaces[6], &interfaces[1]], 
                  &[&vertices[1], &vertices[2], &vertices[6], &vertices[5]], 1),
        GridCell::new(&[&interfaces[7], &interfaces[8], &interfaces[9], &interfaces[5]], 
                  &[&vertices[2], &vertices[3], &vertices[7], &vertices[6]], 2),
        GridCell::new(&[&interfaces[2], &interfaces[10], &interfaces[11], &interfaces[12]], 
                  &[&vertices[4], &vertices[5], &vertices[9], &vertices[8]], 3),
        GridCell::new(&[&interfaces[6], &interfaces[13], &interfaces[14], &interfaces[10]], 
                  &[&vertices[5], &vertices[6], &vertices[10], &vertices[9]], 4),
        GridCell::new(&[&interfaces[9], &interfaces[15], &interfaces[16], &interfaces[13]], 
                  &[&vertices[6], &vertices[7], &vertices[11], &vertices[10]], 5),
        GridCell::new(&[&interfaces[11], &interfaces[17], &interfaces[18], &interfaces[19]], 
                  &[&vertices[8], &vertices[9], &vertices[13], &vertices[12]], 6),
        GridCell::new(&[&interfaces[14], &interfaces[20], &interfaces[21], &interfaces[17]], 
                  &[&vertices[9], &vertices[10], &vertices[14], &vertices[13]], 7),
        GridCell::new(&[&interfaces[16], &interfaces[22], &interfaces[23], &interfaces[20]], 
                  &[&vertices[10], &vertices[11], &vertices[15], &vertices[14]], 8),
    ];

    let boundaries = HashMap::from([
        ("slip_wall_bottom".to_string(), vec![0, 4, 7]),
        ("outflow".to_string(), vec![8, 15, 22]),
        ("slip_wall_top".to_string(), vec![18, 21, 23]),
        ("inflow".to_string(), vec![3, 12, 19]),
    ]);
    (vertices, interfaces, cells, boundaries)
}

#[test]
fn read_su2_file() {
    let mut block_collection = BlockCollection::new();
    block_collection.add_block(&PathBuf::from("./tests/data/square.su2")).unwrap();    
    let block = block_collection.get_block(0);


    let (vertices, interfaces, cells, boundaries) = create_block_elements();

    assert_eq!(block.vertices(), &vertices);
    assert_eq!(block.interfaces(), &interfaces);
    assert_eq!(block.cells(), &cells);
    assert_eq!(block.boundaries(), &boundaries);
    assert_eq!(block.dimensions(), 2);
}

#[test]
fn write_su2_file() {
    let dir = env!("CARGO_TARGET_TMPDIR");
    let (vertices, interfaces, cells, boundaries) = create_block_elements();
    let ref_block = GridBlock::new(vertices, interfaces, cells, boundaries, 2, 0);
    let mut block_collection = BlockCollection::new();
    let path = PathBuf::from(dir).join("su2_test.su2");
    write_block(&ref_block, &path.clone()).unwrap();
    block_collection.add_block(&path).unwrap();
    let read_block = block_collection.get_block(0);

    assert_eq!(ref_block.vertices(), read_block.vertices());
}
