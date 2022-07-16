
use bevy::{prelude::{*, shape::{Quad, UVSphere}}};
use bevy_inspector_egui::*;
use bevy_light_cycles::*;
use splines::*;

fn main() {
    App::new()
    .insert_resource(WindowDescriptor {
        title: "Day Night Cycle Example".to_string(),
        ..default()
    })
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(LightCyclesPlugin)
        .add_startup_system(setup)
        .run()
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assets: Res<AssetServer>,
) {
    let dirt = materials.add(StandardMaterial {
        base_color_texture: Some(assets.load("dirt_albedo.png")),
        normal_map_texture: Some(assets.load("dirt_normal.png")),
        occlusion_texture: Some(assets.load("dirt_orm.png")),
        metallic: 0.1,
        perceptual_roughness: 0.9,
        ..default()
    });
    
    let rock = materials.add(StandardMaterial {
        base_color_texture: Some(assets.load("rock_albedo.png")),
        normal_map_texture: Some(assets.load("rock_normal.png")),
        occlusion_texture: Some(assets.load("rock_orm.png")),
        metallic: 0.2,
        perceptual_roughness: 0.6,
        ..default()
    });

    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(Quad::new(Vec2::new(10.0, 10.0)))),
        material: dirt.clone(),
        transform: Transform::from_xyz(5.0, 5.0, 0.0),
        ..default()
    });

    for x in 0..5 {
        for y in 0..5 {
            commands.spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(UVSphere{
                    radius: 0.5,
                    sectors: 9,
                    stacks: 6,
                })),
                material: rock.clone(),
                transform: Transform::from_xyz((x * 2 + 1) as f32, (y * 2 + 1) as f32, 0.25),
                ..default()
            });
        }
    }

    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(0.0, 0.0, 4.0).looking_at(Vec3::new(3.0, 3.0, 0.0), Vec3::Z),
        ..PerspectiveCameraBundle::new_3d()
    });

    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::rgb(1.0, 1.0, 1.0),
            illuminance: 10000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(10.0, 5.0, 1000.0).looking_at(Vec3::new(5.0, 10.0, 0.0), Vec3::Z),
        ..default()
    }).insert(IlluminanceCycle {
        spline: Spline::from_vec(vec![
            Key::new(0.0, 1000.0, splines::Interpolation::Linear),
            Key::new(5.0, 50000.0, splines::Interpolation::Linear),
            Key::new(10.0, 1000.0, splines::Interpolation::Linear)
        ]),
        timer: Timer::from_seconds(10.0, true)
    });
}