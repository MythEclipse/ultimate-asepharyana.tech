use bevy::prelude::*;
use bevy::window::WindowResolution;

use bevy::asset::AssetMetaCheck;
use bevy::color::palettes::css::*;
use rand::RngExt;

// CinematicState removed to allow infinite simulation without state transition.

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
struct CinematicTimer {
    elapsed: f32,
    duration: f32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Phantom Solar Protocol".into(),
                resolution: WindowResolution::new(1920, 1080),
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
        }).set(bevy::render::RenderPlugin {
            render_creation: bevy::render::settings::RenderCreation::Automatic(bevy::render::settings::WgpuSettings {
                backends: Some(bevy::render::settings::Backends::all()),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(CinematicTimer { elapsed: 0.0, duration: 10.0 })
        .add_systems(Startup, (setup, signal_readiness))
        .add_systems(Update, (
            update_cinematic_timer,
            orbital_mechanics,
            cinematic_camera_movement,
        ))
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
    commands.spawn((
        Camera::default(),
        Camera3d::default(),
        bevy::render::camera::CameraRenderGraph::new(bevy::core_pipeline::core_3d::graph::Core3d),
        Transform::from_xyz(0.0, 800.0, 1500.0).looking_at(Vec3::ZERO, Dir3::Y),
        Projection::Perspective(PerspectiveProjection {
            far: 20000.0,
            ..default()
        }),
        bevy::core_pipeline::tonemapping::Tonemapping::TonyMcMapface,
        bevy::render::view::Hdr, // Stands as a component in 0.18
        bevy::post_process::bloom::Bloom::default(), // Component in 0.18
        CinematicCamera,
    ));

    // The Sun: Realistic Texture + High Emissive + Unlit
    let sun_texture = asset_server.load("sun.jpg");

    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(45.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::from(ORANGE),
            base_color_texture: Some(sun_texture),
            emissive: LinearRgba::new(1.0, 0.4, 0.1, 1.0) * 12.0, // Boosted emissive for Bloom
            unlit: true,
            ..default()
        })),
        Sun,
    ));

    // Global Light (Sun representation)
    commands.spawn((
        DirectionalLight {
            color: Color::WHITE,
            illuminance: 15000.0,
            shadows_enabled: true, // Re-enabled for WebGPU
            ..default()
        },
        Transform::from_xyz(0.0, 100.0, 100.0).looking_at(Vec3::ZERO, Dir3::Y),
    ));

    // Realistic Planets
    for planet_type in PLANETS {
        let orbit_radius = planet_type.orbit_radius();
        let orbit_speed = planet_type.orbit_speed();
        let size = planet_type.size();
        let angle = rand::rng().random_range(0.0..std::f32::consts::TAU);

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

        let mut transform = Transform::from_xyz(
            orbit_radius * angle.cos(),
            0.0,
            orbit_radius * angle.sin(),
        ).with_scale(Vec3::splat(size));

        // Add axial tilt if Saturn
        if matches!(planet_type, PlanetType::Saturn) {
            transform = transform.with_rotation(Quat::from_rotation_z(26.7f32.to_radians()));
        }

        let planet_entity = commands.spawn((
            Mesh3d(meshes.add(Sphere::new(1.0))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::from(fallback_color),
                base_color_texture: Some(planet_texture),
                metallic: 0.1,
                perceptual_roughness: 0.8,
                ..default()
            })),
            transform,
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
                parent.spawn((
                    Mesh3d(meshes.add(Annulus::new(1.2, 2.8))), // Realistic thin ring disc
                    MeshMaterial3d(materials.add(StandardMaterial {
                        base_color_texture: Some(ring_texture),
                        alpha_mode: AlphaMode::Blend,
                        cull_mode: None, // Visible from both sides
                        unlit: false,
                        ..default()
                    })),
                    Transform::from_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
                ));
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

    let mut rng = rand::rng();
    for _ in 0..BACKGROUND_STAR_COUNT {
        let dist = rng.random_range(2000.0..4000.0);
        let theta = rng.random_range(0.0..std::f32::consts::TAU);
        let phi = rng.random_range(0.0..std::f32::consts::PI);

        let x = dist * phi.sin() * theta.cos();
        let y = dist * phi.sin() * theta.sin();
        let z = dist * phi.cos();

        commands.spawn((
            Mesh3d(star_mesh.clone()),
            MeshMaterial3d(star_material.clone()),
            Transform::from_xyz(x, y, z),
            BackgroundStar,
        ));
    }

    commands.spawn(AmbientLight {
        color: Color::WHITE,
        brightness: 80.0,
        ..default()
    });
}

// cleanup_intro removed. Simulation continues indefinitely.

fn orbital_mechanics(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Planet)>,
) {
    let dt = time.delta_secs();
    for (mut transform, mut planet) in &mut query {
        planet.angle += planet.orbit_speed * dt;
        transform.translation.x = planet.orbit_radius * planet.angle.cos();
        transform.translation.z = planet.orbit_radius * planet.angle.sin();

        // Tilt the orbit slightly for aesthetics
        transform.translation.y = (planet.orbit_radius * 0.1) * (planet.angle * 0.5).cos();
    }
}

fn update_cinematic_timer(
    time: Res<Time>,
    mut timer: ResMut<CinematicTimer>,
) {
    timer.elapsed += time.delta().as_secs_f32();
}

fn cinematic_camera_movement(
    mut query: Query<&mut Transform, With<CinematicCamera>>,
    timer: Res<CinematicTimer>,
) {
    let mut transform = query.single_mut().unwrap();

    let t = timer.elapsed;
    let d = timer.duration;
    
    // Initial fly-by transition (progress 0.0 to 1.0 during duration)
    let progress = (t / d).min(1.0);
    
    // Smooth transition from intro fly-by to infinite drift
    // Radius starts at 1200, moves to 700 during intro, then slow oscillation
    let base_radius = 1200.0 - (progress * 500.0);
    let drift_radius = if t > d { (t - d).sin() * 50.0 } else { 0.0 };
    let radius = base_radius + drift_radius;

    // Rotation continues forever
    let rot_speed = 0.15;
    let rot_angle = t * rot_speed;

    // Height targets 0.0 from 400.0, then slow vertical drift
    let base_height = 400.0 * (1.0 - progress);
    let drift_height = if t > d { ((t - d) * 0.5).cos() * 100.0 } else { 0.0 };
    let height = base_height + drift_height;

    transform.translation.x = radius * rot_angle.cos();
    transform.translation.z = radius * rot_angle.sin();
    transform.translation.y = height;

    // Look at sun with a slight dynamic offset to keep it "cinematic"
    let look_offset = if t > d {
        Vec3::new((t * 0.3).sin() * 20.0, (t * 0.2).cos() * 10.0, 0.0)
    } else {
        Vec3::ZERO
    };
    
    transform.look_at(Vec3::ZERO + look_offset, Dir3::Y);
}


