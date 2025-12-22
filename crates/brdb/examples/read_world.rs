use brdb::{BrFsReader, Brdb, IntoReader};
use std::path::PathBuf;

/// Reads a world and prints out some of its information
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // world file from argv
    let filename = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "world.brdb".to_string());
    let path = PathBuf::from(filename);
    if !path.exists() {
        eprintln!("File does not exist: {}", path.display());
        std::process::exit(1);
    }

    let db = Brdb::open(path)?.into_reader();

    let data = db.global_data()?;
    println!("Basic Brick assets: {:?}", data.basic_brick_asset_names);
    println!("Wire ports: {:?}", data.component_wire_port_names);
    println!("Component types: {:?}", data.component_type_names);
    println!("Component structs: {:?}", data.component_data_struct_names);
    println!("Component schemas: {}", db.components_schema()?);

    let mut grid_ids = vec![1];

    // Iterate all entity chunks to find dynamic brick grids...
    // This could totally be a helper function
    for index in db.entity_chunk_index()? {
        for e in db.entity_chunk(index)? {
            // Ensure the chunk is a dynamic brick grid
            if !e.is_brick_grid() {
                continue;
            }
            let Some(id) = e.id else {
                continue;
            };
            grid_ids.push(id);
        }
    }

    for gid in grid_ids {
        println!("Reading grid {gid}");
        let chunks = db.brick_chunk_index(gid)?;
        println!("Brick chunks: {chunks:?}");
        for chunk in chunks {
            let soa = db.brick_chunk_soa(gid, chunk.index)?;
            println!("Brick Soa {chunk}: {soa:?}");
            if chunk.num_components > 0 {
                let (_soa, components) = db.component_chunk_soa(gid, chunk.index)?;
                // println!("Components soa: {soa}");
                for c in components {
                    println!("Component: {c}");
                }
            }
            if chunk.num_wires > 0 {
                let soa = db.wire_chunk_soa(gid, chunk.index)?;
                println!("Wires soa: {soa}");
            }
        }
    }

    println!("Files: {}", db.get_fs()?.render());

    Ok(())
}
