//! A simple 3D scene with light shining over a cube sitting on a plane.

use bevy::{input::common_conditions::input_toggle_active, prelude::*};

use bevy_inspector_egui::quick::WorldInspectorPlugin;

use bevy_mod_picking::{DefaultPickingPlugins, PickableBundle, PickingCameraBundle};
use bevy_transform_gizmo::{GizmoPickSource, GizmoTransformable, TransformGizmoPlugin};

use smooth_bevy_cameras::{
    controllers::unreal::{UnrealCameraBundle, UnrealCameraController, UnrealCameraPlugin},
    LookTransformPlugin,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Better shadow quality:
        //.insert_resource(bevy::pbr::PointLightShadowMap { size: 4096 })
        //.insert_resource(bevy::pbr::DirectionalLightShadowMap { size: 4096 })
        .add_startup_system(setup)
        .add_plugin(LookTransformPlugin)
        .add_plugin(UnrealCameraPlugin::default())
        .add_plugins(DefaultPickingPlugins)
        .add_plugin(TransformGizmoPlugin::default())
        .add_system(toggle_camera_orbit)
        .add_plugin(
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Escape)),
        )
        .run();
}

#[derive(Debug, Reflect, Component)]
struct Pickable;

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(5.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
    // cube
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        Pickable,
        PickableBundle::default(),
        GizmoTransformable,
    ));
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    commands
        .spawn((
            Camera3dBundle {
                transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
                ..default()
            },
            PickingCameraBundle::default(),
            GizmoPickSource::default(),
        ))
        .insert(UnrealCameraBundle::new(
            UnrealCameraController::default(),
            Vec3::new(-2.0, 5.0, 5.0),
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::Y,
        ));
}

fn toggle_camera_orbit(
    mut query_cam: Query<&mut UnrealCameraController>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if !keyboard_input.just_released(KeyCode::Escape) {
        return;
    }

    let mut cam = query_cam.single_mut();
    cam.enabled = !cam.enabled;
}
