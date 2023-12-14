use bevy::{prelude::*, render::{render_resource::{Texture, SamplerDescriptor, AddressMode}, color, texture::ImageSampler}};
use bevy_infinite_grid::{
    GridShadowCamera, InfiniteGridBundle, InfiniteGridPlugin, InfiniteGridSettings,
};
use bevy_flycam::prelude::*;
use bevy::core_pipeline::clear_color::ClearColorConfig;

const DEMO_COORDINATES:Vec3 = Vec3::new(0.0, 0.0, 0.0);
const CAMERA_SPAWN:Vec3 = Vec3::new(0.0, 4.37, 14.77);


/*
    These data structures define general relationships between assets and types as they relate to the dodgeball gamemode.
    They're all pretty bespoke, but this is just a for fun project so slap any definitions we need in here
*/

#[derive(Default)]
struct DebugAssets {
    debug_grid_texture_c: Handle<Image>,
    debug_grid_texture_n: Handle<Image>
}

#[derive(Resource, Default)]
struct DodgeballAssets {
    debug_assets: DebugAssets
}



/* -------------------------------------------------------------------------- */
fn fixup_images(
    mut ev_asset: EventReader<AssetEvent<Image>>,
    mut assets: ResMut<Assets<Image>>,
    game_assets: Res<DodgeballAssets>
) {
    for ev in ev_asset.iter() {
        match ev {
            AssetEvent::Created { handle } => {
                // a texture was just loaded or changed!

                // WARNING: this mutable access will cause another
                // AssetEvent (Modified) to be emitted!
                let texture = assets.get_mut(handle).unwrap();
                // ^ unwrap is OK, because we know it is loaded now
                
                let desc = SamplerDescriptor {
                    address_mode_u: AddressMode::Repeat,
                    address_mode_v: AddressMode::Repeat,
                    address_mode_w: AddressMode::Repeat,
                    ..Default::default()
                };

                if *handle == game_assets.debug_assets.debug_grid_texture_c ||  *handle == game_assets.debug_assets.debug_grid_texture_n {
                    // it is our special map image!
                    println!("SWAG");
                    texture.sampler_descriptor = ImageSampler::Descriptor(desc);
                } 
            }
            AssetEvent::Modified { handle } => {
                // an image was modified
            }
            AssetEvent::Removed { handle } => {
                // an image was unloaded
            }
        }
    }
}

fn setup_demo_area(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut game_assets: ResMut<DodgeballAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut standard_materials: ResMut<Assets<StandardMaterial>>,
) {
    let color_texture_handle = asset_server.load("../../data/prototype/grid_C.png");
    let normal_texture_handle = asset_server.load("../../data/prototype/grid_N.png");
    
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_translation(Vec3::X * 15. + Vec3::Y * 20.)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    let prototype_grid_mat = standard_materials.add(StandardMaterial { 
        base_color_texture: Some(color_texture_handle.clone()),
        normal_map_texture: Some(normal_texture_handle.clone()),
        alpha_mode: AlphaMode::Blend,
        ..default()
    });

    // Create some stupid boxes in code to serve as like a wall for a rocket to spawn and hit as an initial demo
    //Spawn the wall
    // commands.spawn(PbrBundle {
    //     material: prototype_grid_mat.clone(),
    //     mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
    //     transform: Transform::from_translation(DEMO_COORDINATES + Vec3::new(-7.5, 2.5, 0.0))
    //         .with_scale(Vec3::new(0.3, 5.0, 8.0)),
    //     ..default()
    // });

    //Spawn the floor
    commands.spawn(PbrBundle {
        material: prototype_grid_mat.clone(),
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        transform: Transform::from_translation(DEMO_COORDINATES + Vec3::new(0.0, 0.0, 0.0))
            .with_scale(Vec3::new(1.0, 0.3, 1.0)),
        ..default()
    });

    game_assets.debug_assets.debug_grid_texture_c = color_texture_handle;
    game_assets.debug_assets.debug_grid_texture_n = normal_texture_handle;

}

fn setup_debug_flycam(
    mut commands: Commands
) {
    let camera_transform = Transform::from_translation(CAMERA_SPAWN).looking_at(DEMO_COORDINATES, Vec3::Y);
    // Transform::looking_at(self, DEMO_COORDINATES, up).with_translation(CAMERA_SPAWN);
    commands.spawn((
        Camera3dBundle {
            camera_3d: Camera3d { 
                clear_color: ClearColorConfig::Custom(Color::rgb(0.1, 0.1, 0.1)), ..Default::default() },
            transform: camera_transform,
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
            fadeout_distance: 500.,
            dot_fadeout_strength: 0.15,
            shadow_color: None,
            x_axis_color: Color::LIME_GREEN,
            z_axis_color: Color::LIME_GREEN,
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
        .init_resource::<DodgeballAssets>()
        .add_systems(Update, fixup_images)
        .add_systems(Startup, setup_debug_flycam)
        .add_systems(Startup, setup_debug_grid)
        .add_systems(Startup, setup_demo_area);
        //     .add_systems(Update, (hello_world, greet_people));
    }
}
