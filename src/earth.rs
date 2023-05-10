use std::f32::consts::TAU;

use bevy::{prelude::*, render::render_resource::TextureFormat};
use bevy_mod_paramap::*;

/* Using textures from bevy_mod_paramap */
const NORMAL_MAP: &str = "earth\\normal_map.jpg";
const HEIGHT_MAP: &str = "earth\\elevation_surface.jpg";
const ROUGH_MAP: &str = "earth\\metallic_roughness.png";
const ALBEDO_MAP: &str = "earth\\base_color.jpg";
const EMI_MAP: &str = "earth\\emissive.jpg";

pub struct EarthPlugin;

#[derive(Component)]
pub struct Earth;

#[derive(Component, PartialEq)]
struct Spin(f32);

impl Plugin for EarthPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ParallaxMaterialPlugin)
            .insert_resource(AmbientLight {
                color: Color::BLACK,
                brightness: 0.01,
            })
            .insert_resource(ClearColor(Color::BLACK))
            .insert_resource(Normal(None))
            .add_system(update_normal)
            .add_startup_system(spawn_earth)
            .add_system(spin);
    }
}

fn spawn_earth(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ParallaxMaterial>>,
    mut normal: ResMut<Normal>,
    assets: Res<AssetServer>,
) {
    let mut sphere: Mesh = shape::UVSphere::default().into();
    let normal_handle = assets.load(NORMAL_MAP);
    normal.0 = Some(normal_handle.clone());
    sphere.generate_tangents().unwrap();

    commands.spawn((
        Name::new("Earth"),
        Earth,
        Spin(0.1),
        MaterialMeshBundle {
            transform: Transform::from_rotation(Quat::from_euler(
                EulerRot::XYZ,
                -TAU / 4.0,
                0.0,
                TAU / 2.0,
            )),
            mesh: meshes.add(sphere),
            // https://github.com/nicopap/bevy_mod_paramap/blob/main/examples/earth3d.rs
            material: materials.add(ParallaxMaterial {
                // reduce roughness set in the "earth/metallic_roughness.png" file
                perceptual_roughness: 0.75,
                // The base color. See README for source.
                base_color_texture: Some(assets.load(ALBEDO_MAP)),
                // Since emissive_texture value is multiplied by emissive, we use emissive
                // to reduce the intensity of the emissive_texture, so that the lights only
                // show up in earth's penumbra.
                emissive: Color::rgb_u8(30, 30, 30),
                // the nighttime visuals. See README for source.
                emissive_texture: Some(assets.load(EMI_MAP)),
                // The normal map generated from "earth/elevation_surface.png" using GIMP's
                // Filters -> Generic -> Normal Map filter.
                normal_map_texture: normal_handle,
                // See README for source.
                height_map: assets.load(HEIGHT_MAP),
                // Set the water to have a low roughness, while surface has high roughness.
                metallic_roughness_texture: Some(assets.load(ROUGH_MAP)),
                // How "deep" to displace stuff
                height_depth: 0.01,
                // Use the quality algo, for show.
                algorithm: ParallaxAlgo::ReliefMapping,
                // This is an unreasonably high value, but since we expect to inspect up close
                // the surface of the texture, we need to set the max_height_layers pretty high.
                max_height_layers: 128.0,
                flip_normal_map_y: false,
                ..default()
            }),
            ..default()
        },
    ));

    commands.spawn((PointLightBundle {
        point_light: PointLight {
            intensity: 500.0,
            ..default()
        },
        transform: Transform::from_xyz(2.0, 0.5, 2.0),
        ..default()
    },));

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(2.5, 1.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        PanOrbitCamera::default(),
    ));
}

fn spin(time: Res<Time>, mut query: Query<(&mut Transform, &Spin)>) {
    for (mut transform, spin) in query.iter_mut() {
        transform.rotate_y(spin.0 * time.delta_seconds());
    }
}

#[derive(Component)]
struct PanOrbitCamera {
    /// The "focus point" to orbit around. It is automatically updated when panning the camera
    pub focus: Vec3,
    pub radius: f32,
    pub upside_down: bool,
}

impl Default for PanOrbitCamera {
    fn default() -> Self {
        PanOrbitCamera {
            focus: Vec3::ZERO,
            radius: 5.0,
            upside_down: false,
        }
    }
}

/// Store handle of the earth normal to later modify its format
/// in [`update_normal`].
#[derive(Resource)]
struct Normal(Option<Handle<Image>>);

/// Work around the fact that the default bevy image loader sets the
/// normal's format to something incompatible with normal shaders.
/// The format must be one of the `TextureFormat` ending in `*Unorm`.
///
/// In this function, we wait until the image is loaded, immediately
/// change its format and never run the core logic afterward.
///
/// Without proper format, it looks like the light source moves as the
/// earth move, and there is major glitchy artifacts on the poles.
fn update_normal(
    mut already_ran: Local<bool>,
    mut images: ResMut<Assets<Image>>,
    normal: Res<Normal>,
) {
    if *already_ran {
        return;
    }
    if let Some(normal) = normal.0.as_ref() {
        if let Some(mut image) = images.get_mut(normal) {
            image.texture_descriptor.format = TextureFormat::Rgba8Unorm;
            *already_ran = true;
        }
    }
}
