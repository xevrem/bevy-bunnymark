use bevy::{prelude::*, window::PresentMode, diagnostic::{FrameTimeDiagnosticsPlugin, Diagnostics}, core::FixedTimestep};
use rand::prelude::*;

#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "I am a window!".to_string(),
            width: 800.,
            height: 600.,
            present_mode: PresentMode::Fifo,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin)
        .add_startup_system(setup)
        .add_system(jiggle_position)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.0))
                .with_system(fps_track))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, window: Res<WindowDescriptor>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    let width = window.width;
    let height = window.height;
    for _i in 0..10000 {
        let x: f32 = width * random::<f32>() - width / 2.0;
        let y: f32 = height * random::<f32>() - height / 2.0;
        let z: f32 = 5.0 * random::<f32>() - height / 2.5;
        commands
            .spawn_bundle(SpriteBundle {
                texture: asset_server.load("bunny.png"),
                transform: Transform::from_translation(Vec3::new(x, y, z)),
                ..default()
            })
            .insert(Position { x, y });
    }
}

fn jiggle_position(mut query: Query<(&mut Position, &mut Transform)>) {
    for (mut position, mut transform) in query.iter_mut() {
        position.x += random::<f32>() * 5.0 - 2.5;
        position.y += random::<f32>() * 5.0 - 2.5;
        transform.translation = Vec3::new(position.x, position.y, 0.0);
    }
}

fn fps_track(diag: Res<Diagnostics>) {
    if let Some(frame_time)= diag.get(FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(avg_time) = frame_time.average(){
            info!("roughly {} fps", avg_time );
        }
    }
}
