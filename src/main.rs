use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy::math::Vec3;
use rand::*;

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 8 })
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_system(spawn_things.system())
        .add_system(update_pos.system())
        .add_system(kill_particle.system())
        .run();
}

struct Particle;
struct Lifetime(i32);
struct Velocity(Vec3);
struct Acceleration(Vec3);
struct Alive(bool);

fn spawn_things(mut commands: Commands) {
    let shape = shapes::Circle {
        radius: 5.0,
        center: Default::default()
    };

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    let mut rng = thread_rng();
    for _i in 0..100 {
        commands.spawn_bundle(GeometryBuilder::build_as(
            &shape,
            ShapeColors::outlined(Color::BLACK, Color::BLACK),
            DrawMode::Outlined {
                fill_options: FillOptions::default(),
                outline_options: StrokeOptions::default().with_line_width(0.0),
            },
            Transform::from_xyz(0.0, 0.0, 0.0),

        )).insert(Particle)
            .insert(Acceleration(Vec3::new(0.0, 0.002, 0.0))).insert(Lifetime(255))
            .insert(Velocity(Vec3::new(rng.gen_range(-0.5..0.5), rng.gen_range(0.0..1.0), 0.0)))
            .insert(Alive(true));
    }
}

fn kill_particle(mut query: Query<(&mut Lifetime, &mut Alive, &mut ShapeColors)>) {
    for (mut lifetime, mut is_alive, mut colors) in query.iter_mut() {
        if !(is_alive.0) {
            continue;
        }

        lifetime.0 = lifetime.0 - 1;
        if lifetime.0 <= 0 {
            *colors = ShapeColors::outlined(Color::RED, Color::BLACK);
            is_alive.0 = false;
        }
    }
}

fn update_pos(mut query: Query<(&mut Transform, &mut Velocity, &Acceleration, &Alive), With<Particle>>) {

    for (mut pos, mut vel, accel, is_alive) in query.iter_mut() {
        // println!("Particle position: x - {}, y - {}", Transform.x, Transform.y);
        if (is_alive.0) {
            vel.0 = (vel.0) + (accel.0);
            pos.translation = pos.translation + (vel.0);
        }

    }
}
