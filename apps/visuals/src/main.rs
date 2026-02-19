use bevy::prelude::*;
use bevy::window::WindowResolution;
use rand::Rng;

const PARTICLE_COUNT: usize = 2000;
const PARTICLE_SPEED: f32 = 100.0;
const MOUSE_REPEL_RADIUS: f32 = 200.0;
const RETURN_SPEED: f32 = 2.0;

#[derive(Component)]
struct Particle {
    original_pos: Vec3,
    velocity: Vec3,
}

#[derive(Resource, Default)]
struct MousePosition(Option<Vec2>);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Asepharyana Visuals".into(),
                resolution: WindowResolution::new(1920.0, 1080.0),
                canvas: Some("#bevy".into()),
                fit_canvas_to_parent: true,
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::NONE)) // Transparent background
        .init_resource::<MousePosition>()
        .add_systems(Startup, setup)
        .add_systems(Update, (update_mouse, move_particles))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    let mut rng = rand::thread_rng();
    let bounds = Vec2::new(1920.0, 1080.0);

    for _ in 0..PARTICLE_COUNT {
        let x = rng.gen_range(-bounds.x / 2.0..bounds.x / 2.0);
        let y = rng.gen_range(-bounds.y / 2.0..bounds.y / 2.0);
        let z = rng.gen_range(-10.0..10.0);
        let pos = Vec3::new(x, y, z);

        let color = if rng.gen_bool(0.7) {
            Color::linear_rgba(0.5, 0.0, 1.0, 0.8) // Purple
        } else {
            Color::linear_rgba(0.0, 1.0, 1.0, 0.8) // Cyan
        };

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::new(2.0, 2.0)),
                    ..default()
                },
                transform: Transform::from_translation(pos),
                ..default()
            },
            Particle {
                original_pos: pos,
                velocity: Vec3::ZERO,
            },
        ));
    }
}

fn update_mouse(
    mut mouse_pos: ResMut<MousePosition>,
    window_query: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
) {
    let window = window_query.single();
    let (camera, camera_transform) = camera_query.single();

    if let Some(cursor_pos) = window.cursor_position() {
        if let Some(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) {
            mouse_pos.0 = Some(world_pos);
        }
    } else {
        mouse_pos.0 = None;
    }
}

fn move_particles(
    time: Res<Time>,
    mouse_pos: Res<MousePosition>,
    mut query: Query<(&mut Transform, &mut Particle)>,
) {
    let dt = time.delta_seconds();

    for (mut transform, mut particle) in &mut query {
        let mut force = Vec3::ZERO;

        // Repel from mouse
        if let Some(mouse) = mouse_pos.0 {
            let mouse_vec = Vec3::new(mouse.x, mouse.y, 0.0);
            let diff = transform.translation - mouse_vec;
            let dist = diff.length();

            if dist < MOUSE_REPEL_RADIUS && dist > 0.001 {
                let repel_strength = (1.0 - dist / MOUSE_REPEL_RADIUS).powi(2);
                force += diff.normalize() * repel_strength * PARTICLE_SPEED * 10.0;
            }
        }

        // Return to original position (drift)
        let home_diff = particle.original_pos - transform.translation;
        force += home_diff * RETURN_SPEED;

        // Apply force to velocity with damping
        particle.velocity += force * dt;
        particle.velocity *= 0.95; // Damping

        transform.translation += particle.velocity * dt;
    }
}
