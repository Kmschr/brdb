use brdb::{Brdb, IntoReader};
use std::path::PathBuf;

/// Reads a world and prints out all entities with their structs and class names
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

    // Get global data to access entity type names and class names
    let data = db.global_data()?;
    println!("=== Entity Type Names ===");
    for ((i, name), class) in data
        .entity_type_names
        .iter()
        .enumerate()
        .zip(data.entity_data_class_names.iter())
    {
        println!("{i}: {name} - {class}");
    }
    println!();

    // Read all entity chunks
    println!("=== Entity Chunks ===");
    let entity_chunk_indices = db.entity_chunk_index()?;
    println!("Found {} entity chunks", entity_chunk_indices.len());
    println!();

    let mut total_entities = 0;
    for chunk_index in entity_chunk_indices {
        println!("--- Chunk {} ---", chunk_index);
        let entities = db.entity_chunk(chunk_index)?;

        for (i, entity) in entities.iter().enumerate() {
            total_entities += 1;
            println!("  Entity {i}:");
            println!("    Asset: {}", entity.asset);
            println!("    ID: {:?}", entity.id);
            println!("    Owner Index: {:?}", entity.owner_index);
            println!("    Location: {:?}", entity.location);
            println!("    Rotation: {:?}", entity.rotation);
            println!("    Velocity: {:?}", entity.velocity);
            println!("    Angular Velocity: {:?}", entity.angular_velocity);
            println!("    Frozen: {:?}", entity.frozen);
            println!("    Sleeping: {:?}", entity.sleeping);
            println!();
        }
    }

    println!("=== Summary ===");
    println!("Total entities: {total_entities}");

    // Also print the entity chunk SoA data
    println!();
    println!("=== Entity Chunk SoA Data ===");
    for chunk_index in db.entity_chunk_index()? {
        println!("--- Chunk {} (SoA) ---", chunk_index);
        let (soa, entity_data) = db.entity_chunk_soa(chunk_index)?;

        println!("  Type Counters: {:?}", soa.type_counters);
        println!("  Number of entities with data: {}", entity_data.len());

        for (i, data) in entity_data.iter().enumerate() {
            if let Some(struct_data) = data {
                println!("  Entity {i} struct: {struct_data}");
            } else {
                println!("  Entity {i}: None");
            }
        }
        println!();
    }

    Ok(())
}
