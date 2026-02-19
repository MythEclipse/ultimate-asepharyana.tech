use bevy::prelude::*;
use bevy::window::WindowResolution;
use rand::Rng;

use bevy::core_pipeline::bloom::BloomSettings;
use bevy::core_pipeline::tonemapping::Tonemapping;

const PLANET_COUNT: usize = 8;
const BACKGROUND_STAR_COUNT: usize = 1500;

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
enum CinematicState {
    #[default]
    Intro,
    Simulation,
}

#[derive(Component)]
struct Sun;

#[derive(Component)]
struct Planet {
    orbit_radius: f32,
    orbit_speed: f32,
    angle: f32,
    _size: f32,
}

#[derive(Component)]
struct BackgroundStar;

#[derive(Component)]
struct CinematicCamera;

#[derive(Resource)]
struct CinematicTimer(Timer);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Phantom Solar Protocol".into(),
                resolution: WindowResolution::new(1920.0, 1080.0),
                canvas: Some("#bevy".into()),
                fit_canvas_to_parent: true,
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(CinematicTimer(Timer::from_seconds(10.0, TimerMode::Once)))
        .init_state::<CinematicState>()
        .add_systems(Startup, (setup, signal_readiness))
        .add_systems(Update, (
            handle_cinematic_transition,
            orbital_mechanics,
            cinematic_camera_movement.run_if(in_state(CinematicState::Intro)),
        ))
        .add_systems(OnEnter(CinematicState::Simulation), cleanup_intro)
        .run();
}

fn signal_readiness() {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            if let Some(parent) = window.parent().ok().flatten() {
                let _ = parent.post_message(&wasm_bindgen::JsValue::from_str("PROTOCOL_READY"), "*");
            }
        }
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 3D Camera with Bloom and HDR
    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface,
            transform: Transform::from_xyz(0.0, 500.0, 1200.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        BloomSettings::default(),
        CinematicCamera,
    ));

    let mut rng = rand::thread_rng();
    
    // The Sun: Central Emissive Star
    let sun_mesh = meshes.add(Sphere::new(40.0));
    commands.spawn((
        PbrBundle {
            mesh: sun_mesh,
            material: materials.add(StandardMaterial {
                base_color: Color::linear_rgba(1.0, 0.9, 0.5, 1.0),
                emissive: LinearRgba::new(1.0, 0.6, 0.2, 1.0) * 15.0,
                ..default()
            }),
            ..default()
        },
        Sun,
    ));

    // Planets
    let planet_mesh = meshes.add(Sphere::new(1.0));
    for i in 1..=PLANET_COUNT {
        let orbit_radius = 100.0 + (i as f32 * 80.0) + rng.gen_range(-20.0..20.0);
        let orbit_speed = rng.gen_range(0.2..0.8) / (orbit_radius / 100.0);
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
        let size = rng.gen_range(5.0..15.0);

        let color = match i % 3 {
            0 => Color::linear_rgba(0.2, 0.4, 1.0, 1.0), // Blueish
            1 => Color::linear_rgba(1.0, 0.3, 0.2, 1.0), // Reddish
            _ => Color::linear_rgba(0.3, 1.0, 0.5, 1.0), // Greenish
        };

        commands.spawn((
            PbrBundle {
                mesh: planet_mesh.clone(),
                material: materials.add(StandardMaterial {
                    base_color: color,
                    emissive: LinearRgba::from(color) * 2.0,
                    metallic: 0.8,
                    perceptual_roughness: 0.2,
                    ..default()
                }),
                transform: Transform::from_xyz(
                    orbit_radius * angle.cos(),
                    0.0,
                    orbit_radius * angle.sin(),
                ).with_scale(Vec3::splat(size)),
                ..default()
            },
            Planet {
                orbit_radius,
                orbit_speed,
                angle,
                _size: size,
            },
        ));
    }

    // Starry Background
    let star_mesh = meshes.add(Sphere::new(0.3));
    let star_material = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        emissive: LinearRgba::WHITE * 5.0,
        ..default()
    });

    for _ in 0..BACKGROUND_STAR_COUNT {
        let dist = rng.gen_range(2000.0..4000.0);
        let theta = rng.gen_range(0.0..std::f32::consts::TAU);
        let phi = rng.gen_range(0.0..std::f32::consts::PI);
        
        let x = dist * phi.sin() * theta.cos();
        let y = dist * phi.sin() * theta.sin();
        let z = dist * phi.cos();

        commands.spawn((
            PbrBundle {
                mesh: star_mesh.clone(),
                material: star_material.clone(),
                transform: Transform::from_xyz(x, y, z),
                ..default()
            },
            BackgroundStar,
        ));
    }

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.05,
    });
}

fn cleanup_intro(
    mut commands: Commands,
    sun_query: Query<Entity, With<Sun>>,
    planet_query: Query<Entity, With<Planet>>,
) {
    for entity in &sun_query {
        commands.entity(entity).despawn_recursive();
    }
    for entity in &planet_query {
        commands.entity(entity).despawn_recursive();
    }
}

fn orbital_mechanics(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Planet)>,
) {
    let dt = time.delta_seconds();
    for (mut transform, mut planet) in &mut query {
        planet.angle += planet.orbit_speed * dt;
        transform.translation.x = planet.orbit_radius * planet.angle.cos();
        transform.translation.z = planet.orbit_radius * planet.angle.sin();
        
        // Tilt the orbit slightly for aesthetics
        transform.translation.y = (planet.orbit_radius * 0.1) * (planet.angle * 0.5).cos();
    }
}

fn handle_cinematic_transition(
    time: Res<Time>,
    mut timer: ResMut<CinematicTimer>,
    state: Res<State<CinematicState>>,
    mut next_state: ResMut<NextState<CinematicState>>,
) {
    if *state.get() == CinematicState::Intro {
        if timer.0.tick(time.delta()).just_finished() {
            next_state.set(CinematicState::Simulation);
        }
    }
}

fn cinematic_camera_movement(
    _time: Res<Time>,
    mut query: Query<&mut Transform, With<CinematicCamera>>,
    timer: Res<CinematicTimer>,
) {
    let mut transform = query.single_mut();
    let progress = timer.0.fraction();
    
    // Orbital fly-by path
    let radius = 1200.0 - (progress * 600.0); // Close in from 1200 to 600
    let rot_angle = progress * std::f32::consts::PI * 0.5; // Rotate 90 degrees
    let height = 400.0 * (1.0 - progress); // Drop from 400 to 0 (looking flat at the star)

    transform.translation.x = radius * rot_angle.cos();
    transform.translation.z = radius * rot_angle.sin();
    transform.translation.y = height;
    
    transform.look_at(Vec3::ZERO, Vec3::Y);
}
