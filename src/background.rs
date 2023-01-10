use bevy::{
    reflect::TypeUuid,
    render::render_resource::{
        AsBindGroup,
        ShaderRef,
    },
    sprite::{
        Material2d,
        Material2dPlugin,
        MaterialMesh2dBundle,
    },
};

use crate::prelude::*;

/// Plugin that will insert a background at Z = -10.0, use the custom 'Star
/// Nest' shader.
pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(Material2dPlugin::<BackgroundMaterial>::default())
            .add_startup_system(spawn_background);
    }
}

/// Spawn a simple stretched quad that will use background shader.
fn spawn_background(
    mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<BackgroundMaterial>>,
) {
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
        transform: Transform {
            translation: Vec3::ZERO,
            scale: Vec3::new(ARENA_WIDTH, ARENA_HEIGHT, 1f32),
            ..default()
        },
        material: materials.add(BackgroundMaterial {}),
        ..default()
    });
}

//----------------------------------------------------------------

#[derive(Debug, Clone, AsBindGroup, TypeUuid)]
#[uuid = "d1776d38-712a-11ec-90d6-0242ac120003"]
struct BackgroundMaterial {}

impl Material2d for BackgroundMaterial {
    fn vertex_shader() -> ShaderRef {
        "background.wgsl".into()
    }

    fn fragment_shader() -> ShaderRef {
        "background.wgsl".into()
    }

    // fn specialize( descriptor: &mut
    // bevy::render::render_resource::RenderPipelineDescriptor, layout:
    // &bevy::render::mesh::MeshVertexBufferLayout, key:
    // bevy::sprite::Material2dKey<Self>,) -> Result<(),
    // bevy::render::render_resource::SpecializedMeshPipelineError> { Ok(()) }
}
