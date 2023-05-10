use crate::{earth::Earth, get_flights_from_js};
use bevy::prelude::*;

/* Plugin setup */

pub struct PlanesPlugin;

impl Plugin for PlanesPlugin {
    fn build(&self, _app: &mut App) {
        _app.add_event::<NewPlaneEvent>()
            .add_system(spawn_plane)
            .add_system(update_planes);
        //.add_system(despawn_landed);
    }
}

/* Plane components */

/// It's a plane
#[derive(Component)]
pub struct Plane {
    uuid: String,
    landed: bool,
}

pub struct NewPlaneEvent {
    pub uuid: String,
    pub latitude: f32,
    pub longitude: f32,
}

fn update_planes(
    mut wr_newplane: EventWriter<NewPlaneEvent>,
    mut planes_query: Query<(&mut Plane, &mut Transform), With<Plane>>,
) {
    let mut flight = get_flights_from_js();

    let mut i = 0;
    while i < 50 {
        if flight.is_none() {
            return;
        }

        let flight_info = flight.unwrap();

        /* Matches an existing plane */
        let mut found: bool = false;

        for (mut plane, mut curr_transform) in &mut planes_query {
            let transform =
                get_transform_from_x_y(flight_info.longitude, flight_info.latitude, 1.0);

            if plane.uuid == flight_info.uuid {
                curr_transform.translation = transform.translation;
                curr_transform.rotation = transform.rotation;
                curr_transform.scale = transform.scale;

                plane.landed = flight_info.landed;

                /* Skip creating a new one */
                found = true;
            }
        }

        if found == false {
            let new_uuid = flight_info.uuid.clone();
            wr_newplane.send(new_plane_event(
                new_uuid,
                flight_info.longitude,
                flight_info.latitude,
            ));
        }

        i = i + 1;
        flight = get_flights_from_js();
    }
}

fn new_plane_event(uuid: String, x: f32, y: f32) -> NewPlaneEvent {
    NewPlaneEvent {
        uuid: uuid,
        longitude: x,
        latitude: y,
    }
}

fn spawn_plane(
    mut ev_newplane: EventReader<NewPlaneEvent>,
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    earth_query: Query<Entity, With<Earth>>,
) {
    for ev in ev_newplane.iter() {
        let (x, y) = (ev.longitude, ev.latitude);
        let transform = get_transform_from_x_y(x, y, 1.0);

        for earth in earth_query.iter() {
            let new_plane = commands
                .spawn((
                    Plane {
                        uuid: ev.uuid.clone(),
                        landed: false,
                    },
                    PbrBundle {
                        transform: transform,
                        material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
                        mesh: meshes.add(shape::Plane::from_size(0.025).into()),
                        ..default()
                    },
                ))
                .id();
            commands.entity(earth).push_children(&[new_plane]);
        }
    }
}

fn convert_2d_to_3d(x: f32, y: f32, r: f32) -> (f32, f32, f32) {
    let theta = x * 2.0 * std::f32::consts::PI;
    let phi = y * std::f32::consts::PI;
    let x_prime = r * phi.sin() * theta.cos();
    let y_prime = r * phi.sin() * theta.sin();
    let z_prime = r * phi.cos();
    (x_prime, y_prime, z_prime)
}

fn get_transform_from_x_y(x: f32, y: f32, r: f32) -> Transform {
    let (x, y, r) = convert_2d_to_3d(x, y, r);

    let coords_vector = Vec3::new(x, y, r);

    // Calculate the normal vector of the plane (towards the sphere's center)
    let plane_normal = coords_vector.normalize();

    // Calculate the rotation angle to align the plane with the sphere's surface
    let rotation_angle = plane_normal.y.acos();

    // Calculate the rotation axis (perpendicular to the plane)
    let rotation_axis = Vec3::new(plane_normal.z, 0.0, -plane_normal.x).normalize();

    // Create a quaternion rotation
    let rotation = Quat::from_axis_angle(rotation_axis, rotation_angle);

    // Create a translation matrix to position the plane
    let translation_matrix = Mat4::from_translation(coords_vector);

    // Create a rotation matrix from the quaternion
    let rotation_matrix = Mat4::from_rotation_translation(rotation, Vec3::ZERO);

    // Combine the rotation and translation matrices
    let transformation_matrix = translation_matrix * rotation_matrix;

    Transform::from_matrix(transformation_matrix)
}
