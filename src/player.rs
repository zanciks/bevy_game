use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;
use crate::GameState;

#[derive(Component)]
pub struct Player;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_level)
            .add_systems(Startup, setup_player)
            .add_systems(Update, (player_movement, player_rotation, player_camera_rotation)
                .run_if(in_state(GameState::Playing))
            );
    }
}

fn setup_level(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    commands.spawn((
        RigidBody::Static,
        Collider::cuboid(8.0, 0.002, 8.0),
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane::from_size(8.0))),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..default()
        },
    ));
}

fn setup_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let capsule = shape::Capsule {
        radius: 0.5,
        depth: 2.0,
        ..default()
    };

    commands.spawn((
        RigidBody::Dynamic,
        Collider::capsule(capsule.depth, capsule.radius),
        PbrBundle {
            mesh: meshes.add(capsule.into()),
            material: materials.add(Color::RED.into()),
            transform: Transform::from_xyz(0.0, 4.0, 0.0),
            ..default()
        },
        Player
    ))
        .with_children(|parent| {
            parent.spawn(Camera3dBundle {
                ..Default::default()
            });
        });
}

fn player_movement( // translational movement only
    keys: Res<Input<KeyCode>>,
    mut players: Query<(&mut Transform, &Player)>,
    time: Res<Time>
) {
    for (mut transform, _player) in players.iter_mut() {
        let mut direction = Vec3::ZERO;
        direction.x = ((keys.pressed(KeyCode::D) as i32) - (keys.pressed(KeyCode::A) as i32)) as f32;
        direction.z = ((keys.pressed(KeyCode::S) as i32) - (keys.pressed(KeyCode::W) as i32)) as f32;

        direction = transform.rotation.mul_vec3(direction);
        transform.translation += direction * time.delta_seconds();
    }
}

fn player_rotation( // lateral movement, where both camera AND player must rotate
    keys: Res<Input<KeyCode>>,
    mut players: Query<(&mut Transform, &Player)>,
    time: Res<Time>
) {
    for (mut transform, _player) in players.iter_mut() {
        let rotation_speed = 0.75;
        let rotation = if keys.pressed(KeyCode::Left) {
            Quat::from_rotation_y(rotation_speed * time.delta_seconds())
        } else if keys.pressed(KeyCode::Right) {
            Quat::from_rotation_y(-rotation_speed * time.delta_seconds())
        } else {
            Quat::default()
        };

        transform.rotation *= rotation;
    }
}

fn player_camera_rotation( // non-lateral movement, where ONLY the camera should move
    keys: Res<Input<KeyCode>>,
    players: Query<&Children>,
    mut cameras: Query<(&Camera, &mut Transform)>,
    time: Res<Time>
) {
    for children in players.iter() {
        for child in children.iter() {
            if let Ok((_camera, mut transform)) = cameras.get_mut(*child) {
                let rotation_speed = 0.75;
                let rotation = if keys.pressed(KeyCode::Up) {
                    Quat::from_rotation_x(rotation_speed * time.delta_seconds())
                } else if keys.pressed(KeyCode::Down) {
                    Quat::from_rotation_x(-rotation_speed * time.delta_seconds())
                } else {
                    Quat::default()
                };

                transform.rotation *= rotation;

                let (axis, angle) = transform.rotation.to_axis_angle();
                let clamped_angle = angle.to_degrees().clamp(-45.0, 45.0).to_radians();
                transform.rotation = Quat::from_axis_angle(axis, clamped_angle);
            }
        }
    }
}