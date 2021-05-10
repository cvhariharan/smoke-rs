use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy::math::Vec3;
use rand::*;
use bevy::render::mesh::VertexAttributeValues;
use std::thread::sleep;
use std::{thread, time};

struct Particle;
struct Lifetime(i32);
struct Velocity(Vec3);
struct Acceleration(Vec3);
struct Alive(bool);
struct CreateTimer(Timer);

fn main() {
    App::build()
        .insert_resource(CreateTimer(Timer::from_seconds(0.001, true)))
        .insert_resource(Acceleration(Vec3::new(0.0, 0.002, 0.0)))
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(Msaa { samples: 8 })
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_system(spawn_things.system())
        .add_system(update_pos.system())
        .add_system(kill_particle.system())
        .add_system(apply_force.system())
        .run();
}

fn apply_force(acc: Res<Acceleration>, mut query: Query<&mut Acceleration>) {
    for mut accel in query.iter_mut() {
        accel.0 = accel.0 + acc.0;
    }
}

fn spawn_things(time: Res<Time>, mut timer: ResMut<CreateTimer>, asset_server: Res<AssetServer>, mut materials: ResMut<Assets<ColorMaterial>>, mut commands: Commands) {
    let shape = shapes::Circle {
        radius: 15.0,
        center: Default::default()
    };
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    let mut rng = thread_rng();
    let texture_handle = asset_server.load("texture.png");
    let tile_size = Vec2::splat(64.0);
    timer.0.tick(time.delta());
    if timer.0.just_finished() {
        commands.spawn_bundle(SpriteBundle {
            sprite: Sprite::new(tile_size),
            material: materials.add(texture_handle.into()),
            transform: Transform::from_xyz(rng.gen_range(-1.5..1.5), rng.gen_range(-1.5..1.5), 0.0),
            ..Default::default()
        }).insert(Particle)
            .insert(Acceleration(Vec3::new(0.0, 0.0, 0.0)))
            .insert(Velocity(Vec3::new(rng.gen_range(-1.5..1.5), rng.gen_range(0.0..1.0), 0.0)))
            .insert(Alive(true)).insert(Lifetime(255));
    }
}

fn kill_particle(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>, mut query: Query<(Entity, &mut Lifetime, &mut Alive, &Handle<ColorMaterial>)>) {
    for (entity, mut lifetime, mut is_alive, material_handle) in query.iter_mut() {
        lifetime.0 = lifetime.0 - 3;
        let m = materials.get_mut(material_handle);
        if let Some(material) = m {
            material.color = Color::rgba(material.color.r(), material.color.g(), material.color.b(), (lifetime.0 as f32)/255.0).into();
        }
        if lifetime.0 <= 0 {
            commands.entity(entity).despawn();
        }
    }
}

fn update_pos(mut query: Query<(&mut Transform, &mut Velocity, &Acceleration, &Alive), With<Particle>>) {
    for (mut pos, mut vel, accel, is_alive) in query.iter_mut() {
        if (is_alive.0) {
            vel.0 = (vel.0) + (accel.0);
            pos.translation = pos.translation + (vel.0);
        }
    }
}
