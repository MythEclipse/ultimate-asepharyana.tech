use bevy::prelude::*;
use bevy::window::WindowResolution;
use rand::Rng;

use bevy::core_pipeline::bloom::BloomSettings;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::asset::AssetMetaCheck;
use bevy::color::palettes::css::*;

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
enum CinematicState {
    #[default]
    Intro,
    Simulation,
}

const BACKGROUND_STAR_COUNT: usize = 1500;

#[derive(Debug, Clone, Copy)]
enum PlanetType {
    Mercury,
    Venus,
    Earth,
    Mars,
    Jupiter,
    Saturn,
    Uranus,
    Neptune,
}

impl PlanetType {
    fn texture_path(&self) -> &'static str {
        match self {
            PlanetType::Mercury => "mercury.jpg",
            PlanetType::Venus => "venus.jpg",
            PlanetType::Earth => "earth.jpg",
            PlanetType::Mars => "mars.jpg",
            PlanetType::Jupiter => "jupiter.jpg",
            PlanetType::Saturn => "saturn.jpg",
            PlanetType::Uranus => "uranus.jpg",
            PlanetType::Neptune => "neptune.jpg",
        }
    }

    fn orbit_radius(&self) -> f32 {
        match self {
            PlanetType::Mercury => 80.0,
            PlanetType::Venus => 140.0,
            PlanetType::Earth => 200.0,
            PlanetType::Mars => 280.0,
            PlanetType::Jupiter => 450.0,
            PlanetType::Saturn => 650.0,
            PlanetType::Uranus => 850.0,
            PlanetType::Neptune => 1050.0,
        }
    }

    fn size(&self) -> f32 {
        match self {
            PlanetType::Mercury => 3.0,
            PlanetType::Venus => 7.0,
            PlanetType::Earth => 7.5,
            PlanetType::Mars => 4.0,
            PlanetType::Jupiter => 25.0,
            PlanetType::Saturn => 22.0,
            PlanetType::Uranus => 12.0,
            PlanetType::Neptune => 11.0,
        }
    }

    fn orbit_speed(&self) -> f32 {
        // Keplers 3rd Law approx
        20.0 / self.orbit_radius().sqrt()
    }
}

const PLANETS: [PlanetType; 8] = [
    PlanetType::Mercury,
    PlanetType::Venus,
    PlanetType::Earth,
    PlanetType::Mars,
    PlanetType::Jupiter,
    PlanetType::Saturn,
    PlanetType::Uranus,
    PlanetType::Neptune,
];

#[derive(Component)]
struct Sun;

#[derive(Component)]
struct Planet {
    orbit_radius: f32,
    orbit_speed: f32,
    angle: f32,
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
        }).set(AssetPlugin {
            file_path: "assets".to_string(),
            meta_check: AssetMetaCheck::Never,
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
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 3D Camera with Bloom and HDR

    // 3D Camera with Bloom and HDR
    commands.spawn((
        Camera3dBundle {
            projection: Projection::Perspective(PerspectiveProjection {
                far: 20000.0,
                ..default()
            }),
            camera: Camera {
                hdr: true,
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface,
            transform: Transform::from_xyz(0.0, 800.0, 1500.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        BloomSettings::default(),
        CinematicCamera,
    ));

    // The Sun: Realistic Texture + High Emissive + PointLight
    let sun_texture = asset_server.load("sun.jpg");
    
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Sphere::new(45.0)),
            material: materials.add(StandardMaterial {
                base_color: ORANGE.into(),
                base_color_texture: Some(sun_texture),
                emissive: LinearRgba::new(1.0, 0.4, 0.1, 1.0) * 80.0,
                ..default()
            }),
            ..default()
        },
        Sun,
    )).with_children(|parent| {
        parent.spawn(PointLightBundle {
            point_light: PointLight {
                color: Color::WHITE,
                intensity: 2_000_000_000.0,
                range: 20000.0,
                shadows_enabled: true,
                ..default()
            },
            ..default()
        });
    });

    // Realistic Planets
    for planet_type in PLANETS {
        let orbit_radius = planet_type.orbit_radius();
        let orbit_speed = planet_type.orbit_speed();
        let size = planet_type.size();
        let angle = rand::random::<f32>() * std::f32::consts::TAU;

        let path = planet_type.texture_path();
        let planet_texture = asset_server.load(path);
        
        let fallback_color = match planet_type {
            PlanetType::Mercury => SILVER,
            PlanetType::Venus => YELLOW,
            PlanetType::Earth => BLUE,
            PlanetType::Mars => RED,
            PlanetType::Jupiter => ORANGE_RED,
            PlanetType::Saturn => GOLD,
            PlanetType::Uranus => LIGHT_BLUE,
            PlanetType::Neptune => DARK_BLUE,
        };

        let planet_entity = commands.spawn((
            PbrBundle {
                mesh: meshes.add(Sphere::new(1.0)),
                material: materials.add(StandardMaterial {
                    base_color: fallback_color.into(),
                    base_color_texture: Some(planet_texture),
                    metallic: 0.1,
                    perceptual_roughness: 0.8,
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
            },
        )).id();
        
        // Special Case: Saturn's Rings
        if matches!(planet_type, PlanetType::Saturn) {
            let ring_texture = asset_server.load("saturn_ring.jpg");
            
            commands.entity(planet_entity).with_children(|parent| {
                parent.spawn(PbrBundle {
                    mesh: meshes.add(Torus::new(0.2, 2.2)),
                    material: materials.add(StandardMaterial {
                        base_color_texture: Some(ring_texture),
                        alpha_mode: AlphaMode::Blend,
                        ..default()
                    }),
                    transform: Transform::from_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
                    ..default()
                });
            });
        }
    }

    // Starry Background
    let star_mesh = meshes.add(Sphere::new(0.3));
    let star_material = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        emissive: LinearRgba::WHITE * 5.0,
        ..default()
    });

    let mut rng = rand::thread_rng();
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
        brightness: 0.15,
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

