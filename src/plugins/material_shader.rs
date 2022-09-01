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

/// returns -1.0 to 1.0
fn rand_f32() -> f32 {
    rand::random::<f32>() * 2.0 - 1.0
}

fn rand_vec() -> Vec3 {
    Vec3::new(rand_f32(), rand_f32(), rand_f32())
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut glow_materials: ResMut<Assets<GlowyMaterial>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let spawn_radius: f32 = 10.0;
    let max_sphere_radius: f32 = 2.0;

    // spawn camera
    commands
        .spawn_bundle(Camera3dBundle {
            transform: Transform::from_xyz(spawn_radius, spawn_radius, spawn_radius)
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(OrbitCamera {
            distance: spawn_radius * 4.0,
            ..Default::default()
        });

    // spawn plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 1000.0 })),
        material: materials.add(Color::rgb(0.1, 0.1, 0.1).into()),
        transform: Transform::from_xyz(0.0, spawn_radius * -0.9, 0.0),
        ..Default::default()
    });

    // spawn spheres
    let env_texture = asset_server.load("textures/stone_alley_02_1k.hdr");
    let material = glow_materials.add(GlowyMaterial { env_texture });
    let mut locs: Vec<Vec3> = vec![];
    while locs.len() < 100 {
        let radius = rand_f32().abs() * max_sphere_radius;
        let loc = rand_vec() * spawn_radius;
        let rotation = Quat::from_rotation_arc(Vec3::ZERO, rand_vec());
        // check for collisions
        if locs
            .clone()
            .into_iter()
            .any(|l| loc.distance(l) < radius * 2.0)
        {
            continue;
        }
        locs.push(loc);

        commands
            .spawn()
            .insert_bundle(MaterialMeshBundle {
                mesh: meshes.add(Mesh::from(shape::UVSphere {
                    radius,
                    ..default()
                })),
                material: material.clone(),
                transform: Transform::from_translation(loc).with_rotation(rotation),
                ..default()
            })
            .add_children(|parent| {
                parent.spawn_bundle(PointLightBundle {
                    point_light: PointLight {
                        intensity: 10_000.0,
                        radius,
                        color: Color::rgb(0.5, 0.1, 0.0),
                        ..default()
                    },
                    ..default()
                });
            });
    }
}
