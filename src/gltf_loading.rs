//  WIP GLTF IMPLEMENTATION
//  TO BE MERGED w/ model.rs && resource.rs WHEN FINISHED

use gltf::Document; 

pub fn load_gltf_file() -> anyhow::Result<Document> {
    let (gltf, buffers, _) = gltf::import("res/Box.gltf")?;
    for mesh in gltf.meshes() {
        println!("Mesh #{}", mesh.index());
        for primitive in mesh.primitives() {
            println!("- Primitive #{}", primitive.index());
            let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
            if let Some(iter) = reader.read_positions() {
                for vertex_position in iter {
                    println!("Vertex positions: {:?}", vertex_position);
                }
            }
        }
    }
    Ok(gltf)
}