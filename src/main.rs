use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    pbr::CascadeShadowConfigBuilder,
    prelude::*,
    reflect::*,
    render::{
        render_resource::{AsBindGroup, ShaderRef},
        view::RenderLayers,
    },
    ui::UiPlugin,
};

use bevy_rapier3d::{prelude::*, rapier::prelude};
use gestalt::{
    character::CharacterPlugin,
    interactions::{Interactable, InteractionKind, InteractionsPlugin},
    ui::MyUiPlugin,
};
use std::{f32::consts::PI, thread::spawn};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(MaterialPlugin::<CustomMaterial>::default())
        .add_plugin(MyUiPlugin)
        .add_plugin(CharacterPlugin)
        .add_plugin(InteractionsPlugin)
        // .add_plugin(InteractionPlugin)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // .add_plugin(RapierDebugRenderPlugin {
        //     enabled: true,
        //     always_on_top: true,
        //     style: DebugRenderStyle::default(),
        //     mode: DebugRenderMode::default(),
        // })
        // .add_plugin(FpsCameraPlugin)
        .add_startup_system(setup_physics)
        .run();
}

fn setup_physics(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut cmaterials: ResMut<Assets<CustomMaterial>>,
) {
    /* Create the ground. */
    // commands
    //     .spawn(Collider::cuboid(100.0, -0.1, 100.0))
    //     .insert(TransformBundle::from(Transform::from_xyz(0.0, -2.0, 0.0)));

    // commands
    //     .spawn(Collider::cuboid(100.0, 0.01, 100.0))
    //     .insert(MaterialMeshBundle {
    //         mesh: meshes.add(shape::Plane::from_size(100.0).into()),
    //         material: cmaterials.add(CustomMaterial {}),
    //         transform: Transform::from_xyz(0.0, -2.0, 0.0),
    //         ..default()
    //     });

    commands
        .spawn(Collider::cuboid(100.0, 0.01, 100.0))
        .insert(Friction {
            coefficient: 1.0,
            combine_rule: CoefficientCombineRule::Max,
        })
        .insert(MaterialMeshBundle {
            mesh: meshes.add(shape::Plane::from_size(100.0).into()),
            material: materials.add(Color::GRAY.into()),
            transform: Transform::from_xyz(0.0, -2.0, 0.0),
            ..default()
        });

    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::cuboid(0.2, 0.2, 0.2))
        .insert(Interactable::new(InteractionKind::PickUp))
        .insert(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube::new(0.4))),
            material: materials.add(Color::RED.into()),
            transform: Transform::from_xyz(-2.0, 2.0, -2.0),
            ..default()
        });

    // commands.spawn(DirectionalLightBundle {
    //     directional_light: DirectionalLight {
    //         illuminance: 2000.0,
    //         shadows_enabled: true,
    //         ..default()
    //     },
    //     transform: Transform {
    //         translation: Vec3::new(0.0, 2.0, 0.0),
    //         rotation: Quat::from_rotation_x(-PI / 4.),
    //         ..default()
    //     },
    //     cascade_shadow_config: CascadeShadowConfigBuilder { ..default() }.into(),
    //     ..default()
    // });
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 4.0, 4.0),
        ..default()
    });

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(1.4, 8.0, 1.4))),
            material: materials.add(Color::BLUE.into()),
            transform: Transform::from_xyz(2.0, -0.5, 2.0),
            ..default()
        })
        .insert(Collider::cuboid(0.7, 4.0, 0.7));
}

impl Material for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/custom.wgsl".into()
    }
}

#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
pub struct CustomMaterial {}
