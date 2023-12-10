use bevy::prelude::*;
use bevy_infinite_grid::{
    GridShadowCamera, InfiniteGridBundle, InfiniteGridPlugin, InfiniteGridSettings,
};
use bevy_flycam::prelude::*;

fn setup_debug_flycam(
    mut commands: Commands
) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 4.37, 14.77),
            ..default()
        },
        FlyCam,
        GridShadowCamera,
    ));
}

fn setup_debug_grid(
    mut commands: Commands
) {
    commands.spawn(InfiniteGridBundle {
        settings: InfiniteGridSettings {
            // shadow_color: None,
            ..default()
        },
        ..default()
    });

    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_translation(Vec3::X * 15. + Vec3::Y * 20.)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // let mat = standard_materials.add(StandardMaterial::default());

    // // cube
    // commands.spawn(PbrBundle {
    //     material: mat.clone(),
    //     mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
    //     transform: Transform {
    //         translation: Vec3::new(3., 4., 0.),
    //         rotation: Quat::from_rotation_arc(Vec3::Y, Vec3::ONE.normalize()),
    //         scale: Vec3::splat(1.5),
    //     },
    //     ..default()
    // });

    // commands.spawn(PbrBundle {
    //     material: mat.clone(),
    //     mesh: meshes.add(Mesh::from(shape::Cube { size: 2.0 })),
    //     transform: Transform::from_xyz(0.0, 2.0, 0.0),
    //     ..default()
    // });
}


pub struct DodgeballPlugin;

impl Plugin for DodgeballPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InfiniteGridPlugin);
        app.add_plugins(NoCameraPlayerPlugin)
        .insert_resource(MovementSettings {
            ..Default::default()
        })
        // Unreal movement layout
        .insert_resource(KeyBindings {
            move_ascend: KeyCode::E,
            move_descend: KeyCode::Q,
            ..Default::default()
        })
        .add_systems(Startup, setup_debug_flycam)
        .add_systems(Startup, setup_debug_grid);
        //     .add_systems(Update, (hello_world, greet_people));
    }
}
