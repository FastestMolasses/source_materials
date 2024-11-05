use bevy::pbr::CascadeShadowConfigBuilder;
use bevy::prelude::*;
use std::f32::consts::PI;

use crate::{loading::WorldProps, world::GameState};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), create_scene)
            .add_systems(Update, animate_object.run_if(in_state(GameState::Playing)));
    }
}

fn create_scene(
    mut commands: Commands,
    world_props: Res<WorldProps>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Setup directional light similar to Source engine's default compile lighting
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::srgb(1.0, 0.95, 0.85), // Slightly warm light
            illuminance: 50000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_rotation(Quat::from_euler(
            EulerRot::XYZ,
            -45.0_f32.to_radians(),
            45.0_f32.to_radians(),
            0.0,
        )),
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 4.0,
            maximum_distance: 10.0,
            ..default()
        }
        .build(),
        ..default()
    });

    // Add an ambient light to simulate Source engine's ambient lighting
    commands.insert_resource(AmbientLight {
        color: Color::srgb(0.1, 0.1, 0.1),
        brightness: 0.3,
    });

    // Create ground plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default()),
        material: materials.add(StandardMaterial {
            base_color: Color::srgb(0.2, 0.2, 0.2),
            perceptual_roughness: 0.9,
            metallic: 0.0,
            reflectance: 0.2,
            ..default()
        }),
        transform: Transform::from_xyz(0.0, -0.5, 0.0),
        ..default()
    });

    // Spawn rotating crate
    commands.spawn((
        SceneBundle {
            scene: world_props.wood_crate.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        Rotating {
            initial_height: 0.5,
        },
    ));

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

#[derive(Component)]
struct Rotating {
    initial_height: f32,
}

fn animate_object(time: Res<Time>, mut query: Query<(&mut Transform, &Rotating)>) {
    for (mut transform, rotating) in &mut query {
        // Rotate around Y axis
        transform.rotate_y(time.delta_seconds() * PI / 2.0);

        // Add bouncing motion
        let bounce_height = 0.15; // Maximum bounce height
        let bounce_speed = 3.0; // Speed of the bounce

        // Calculate new Y position using a sine wave
        let y_offset = (time.elapsed_seconds() * bounce_speed).sin() * bounce_height;
        transform.translation.y = rotating.initial_height + y_offset;
    }
}
