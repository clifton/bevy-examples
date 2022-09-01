use bevy::asset::AssetServerSettings;
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};

use crate::cameras::orbit::{OrbitCamera, OrbitCameraPlugin};

pub struct MaterialShaderPlugin;

impl Plugin for MaterialShaderPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AssetServerSettings {
            watch_for_changes: true,
            ..default()
        })
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugin(MaterialPlugin::<GlowyMaterial>::default())
        .add_plugin(OrbitCameraPlugin)
        .add_startup_system(setup);
    }
}

#[derive(AsBindGroup, Debug, Clone, TypeUuid)]
#[uuid = "69b8e112-1f3d-45d3-abb2-cee24eb3990d"]
struct GlowyMaterial {
    #[texture(0)]
    #[sampler(1)]
    env_texture: Handle<Image>,
}

impl Material for GlowyMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/glowy.wgsl".into()
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut glow_materials: ResMut<Assets<GlowyMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let env_texture = asset_server.load("textures/stone_alley_02_1k.hdr");
    let material = glow_materials.add(GlowyMaterial { env_texture });

    commands.spawn().insert_bundle(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::UVSphere {
            radius: 1.0,
            ..default()
        })),
        material,
        ..default()
    });

    commands
        .spawn_bundle(Camera3dBundle {
            transform: Transform::from_xyz(5.0, 2.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(OrbitCamera::default());

    // commands.spawn_bundle(PbrBundle {
    //     mesh: meshes.add(Mesh::from(shape::Plane { size: 100.0 })),
    //     material: materials.add(Color::rgb(0.1, 0.1, 0.1).into()),
    //     ..default()
    // });
}
