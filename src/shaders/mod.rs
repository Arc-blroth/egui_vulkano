pub mod vs {
    vulkano_shaders::shader! {
        ty: "vertex",
        path: "src/shaders/vert.vert"
    }
}

pub mod fs {
    vulkano_shaders::shader! {
        ty: "fragment",
        path: "src/shaders/frag.frag"
    }
}
