use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;
use bevy_xpbd_3d::prelude::*;
use crate::GameState;
use crate::settings::Settings;

#[derive(Component)]
pub struct Player {
    speed: f32,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            speed: 2.0,
        }
    }
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_level)
            .add_systems(Startup, setup_player)
            .add_systems(Update, (player_movement, mouse_motion)
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

    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(0.0, 3.0, 0.0),
        ..default()
    });
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
        Player::default(),
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
    time: Res<Time<Physics>>
) {
    for (mut transform, player) in players.iter_mut() {
        let mut direction = Vec3::ZERO;
        direction.x = ((keys.pressed(KeyCode::D) as i32) - (keys.pressed(KeyCode::A) as i32)) as f32;
        direction.z = ((keys.pressed(KeyCode::S) as i32) - (keys.pressed(KeyCode::W) as i32)) as f32;
        direction.x *= player.speed;
        direction.z *= player.speed;

        direction = transform.rotation.mul_vec3(direction);
        transform.rotation.x = 0.0;
        transform.rotation.z = 0.0;

        transform.translation += direction * time.delta_seconds();
    }
}

fn mouse_motion(
    mut motion_evr: EventReader<MouseMotion>,
    mut players: Query<(&mut Transform, &Player, &Children)>,
    mut cameras: Query<(&Camera, &mut Transform), Without<Player>>,
    settings: Res<Settings>,
    time: Res<Time>,
) {
    for ev in motion_evr.read() {
        for (mut player_transform, _player, children) in players.iter_mut() {
            let rotation_speed = 0.03 * settings.mouse_sensitivity;
            let rotation_y = Quat::from_rotation_y(-rotation_speed * ev.delta.x * time.delta_seconds());

            player_transform.rotation *= rotation_y;

            for child in children.iter() {
                if let Ok((_camera, mut camera_transform)) = cameras.get_mut(*child) {
                    let rotation_x = Quat::from_rotation_x(-rotation_speed * ev.delta.y * time.delta_seconds());

                    let (axis, angle) = camera_transform.rotation.to_axis_angle();
                    let clamped_angle = angle.to_degrees().clamp(-60.0, 60.0).to_radians();
                    camera_transform.rotation = Quat::from_axis_angle(axis, clamped_angle);

                    camera_transform.rotation *= rotation_x;
                }
            }
        }
    }
}
