//! A shader that uses dynamic data like the time since startup.
//! The time data is in the globals binding which is part of the `mesh_view_bindings` shader import.

mod func_gen;

use bevy::{
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin},
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
};
use func_gen::generate_tree;
use rand::SeedableRng;
const MAX_DEPTH: u32 = 20;

/// This example uses a shader source file from the assets subdirectory
pub const MESH2D_SHADER_HANDLE: Handle<Shader> = Handle::weak_from_u128(69420);

fn main() {
    App::new()
        .insert_resource(Seed(rand::random()))
        .add_plugins((
            DefaultPlugins,
            FpsOverlayPlugin {
                config: FpsOverlayConfig::default(),
            },
            MaterialPlugin::<CustomMaterial>::default(),
        ))
        .add_systems(Startup, setup)
        .run();
}

#[derive(Resource)]
struct Seed(pub u64);

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut shaders: ResMut<Assets<Shader>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    seed: ResMut<Seed>,
) {
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed.0);
    let cases: String = (0..100)
        .map(|i| {
            let r_tree = generate_tree(MAX_DEPTH, &mut rng);
            // info!("{:?}", r_tree);
            let g_tree = generate_tree(MAX_DEPTH, &mut rng);
            // info!("{:?}", g_tree);
            let b_tree = generate_tree(MAX_DEPTH, &mut rng);
            // info!("{:?}", b_tree);
            format!(
                "case {}u: {{return vec4f(({}), ({}), ({}), 1.0);}}\n",
                i, r_tree, g_tree, b_tree
            )
        })
        .collect();

    let shader_text = format!(
        "
            #import bevy_pbr::{{
                mesh_view_bindings::globals,
                forward_io::VertexOutput,
            }}
           @group(2) @binding(0) var<uniform> id: u32;

           @fragment
           fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {{
            switch id{{
                {cases}
                default: {{return vec4f(0.5, 0.0, 0.5, 1);}}
                }}
           }}
           ",
    );

    print!("{shader_text}");

    shaders.insert(
        &MESH2D_SHADER_HANDLE,
        Shader::from_wgsl(shader_text, file!()),
    );

    // cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(CustomMaterial { id: 1 })),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(CustomMaterial { id: 2 })),
        Transform::from_xyz(2.0, 0.5, 0.0),
    ));

    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct CustomMaterial {
    #[uniform(0)]
    id: u32,
}

impl Material for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        MESH2D_SHADER_HANDLE.into()
    }
}
