use ambient_api::{
    components::core::{
        app::main_scene,
        camera::{aspect_ratio_from_window, fog, fovy},
        physics::linear_velocity,
        transform::{lookat_target, rotation, translation},
    },
    concepts::{make_perspective_infinite_reverse_camera, make_transformable},
    messages::Frame,
    prelude::*,
};
use components::{last_jump_time, player_vehicle, vehicle, vehicle_hud};

mod common;

const CAMERA_OFFSET: Vec3 = vec3(0.5, 1.8, 0.6);
const RENDER_DEBUG: bool = false;

#[main]
pub fn main() {
    let camera_id = Entity::new()
        .with_merge(make_perspective_infinite_reverse_camera())
        .with(aspect_ratio_from_window(), EntityId::resources())
        .with_default(main_scene())
        .with_default(fog())
        .with(translation(), vec3(5., 5., 2.))
        .with(lookat_target(), vec3(0., 0., 1.))
        .spawn();

    spawn_query(vehicle()).bind(move |vehicles| {
        for (id, _) in vehicles {
            let hud_id = Entity::new()
                .with_merge(make_transformable())
                .with_default(local_to_world())
                .with_default(local_to_parent())
                .with_default(mesh_to_local())
                .with_default(mesh_to_world())
                .with_default(main_scene())
                .with(text(), "0".to_string())
                .with(color(), vec4(1., 1., 1., 1.))
                .with(translation(), vec3(0.35, 0., 0.3))
                .with(
                    rotation(),
                    Quat::from_rotation_z(25.0f32.to_radians())
                        * Quat::from_rotation_x(-65.0f32.to_radians()),
                )
                .with(scale(), Vec3::ONE * 0.005)
                .with(font_size(), 48.0)
                .with(parent(), id)
                .spawn();

            entity::add_component(id, vehicle_hud(), hud_id);
            entity::add_component_if_required(id, children(), vec![]);
            entity::mutate_component(id, children(), |children| {
                children.push(hud_id);
            });
        }
    });

    despawn_query(vehicle_hud())
        .requires(vehicle())
        .bind(move |vehicles| {
            for (vehicle_id, hud_id) in vehicles {
                entity::despawn(hud_id);
                entity::mutate_component(vehicle_id, children(), |children| {
                    children.retain(|&c| c != hud_id);
                });
            }
        });

    // HACK: despawn all wheels on spawn
    spawn_query(name()).bind(|entities| {
        for (id, name) in entities {
            if name.starts_with("wheel") {
                entity::despawn(id);
            }
        }
    });

    query((
        vehicle_hud(),
        rotation(),
        linear_velocity(),
        last_jump_time(),
    ))
    .each_frame(|huds| {
        for (_, (hud_id, rot, lv, ljt)) in huds {
            entity::set_component(
                hud_id,
                text(),
                format!(
                    "{:.1}\n{:.1}s",
                    speed_kph(lv, rot),
                    common::JUMP_TIMEOUT - (time() - ljt).min(common::JUMP_TIMEOUT),
                ),
            );
        }
    });

    Frame::subscribe(move |_| {
        let player_id = player::get_local();
        let Some(vehicle_id) = entity::get_component(player_id, player_vehicle()) else { return; };
        let Some(vehicle_position) = entity::get_component(vehicle_id, translation()) else { return; };
        let Some(vehicle_rotation) = entity::get_component(vehicle_id, rotation()) else { return; };
        let Some(vehicle_linear_velocity) = entity::get_component(vehicle_id, linear_velocity()) else { return; };

        let camera_position = vehicle_position + vehicle_rotation * CAMERA_OFFSET;
        entity::set_component(camera_id, translation(), camera_position);
        entity::set_component(
            camera_id,
            lookat_target(),
            camera_position + vehicle_rotation * -Vec3::Y,
        );
        let kph = speed_kph(vehicle_linear_velocity, vehicle_rotation);
        entity::set_component(camera_id, fovy(), 0.9 + (kph.abs() / 300.0).clamp(0.0, 1.0));

        let input = input::get();
        let direction = {
            let mut direction = Vec2::ZERO;
            if input.keys.contains(&KeyCode::W) {
                direction.y += 1.;
            }
            if input.keys.contains(&KeyCode::S) {
                direction.y -= 1.;
            }
            if input.keys.contains(&KeyCode::A) {
                direction.x -= 1.;
            }
            if input.keys.contains(&KeyCode::D) {
                direction.x += 1.;
            }
            direction
        };
        messages::Input {
            direction,
            jump: input.keys.contains(&KeyCode::Space),
        }
        .send_server_unreliable();
    });

    if RENDER_DEBUG {
        DebugUI.el().spawn_interactive();
        DebugLines.el().spawn_interactive();
    }
}

fn speed_kph(linear_velocity: Vec3, rotation: Quat) -> f32 {
    linear_velocity.dot(rotation * -Vec3::Y) * 3.6
}

#[element_component]
fn DebugUI(hooks: &mut Hooks) -> Element {
    let messages = hooks.use_query(components::debug_messages());

    FlowColumn::el(messages.into_iter().map(|(id, msgs)| {
        FlowColumn::el([
            Text::el(format!("{}", id,)).section_style(),
            FlowColumn::el(
                msgs.into_iter()
                    .map(|s| Text::el(s).with(color(), vec4(1., 1., 1., 1.))),
            ),
        ])
    }))
    .with_padding_even(10.)
    .with_background(vec4(1., 1., 1., 0.02))
}

#[element_component]
fn DebugLines(hooks: &mut Hooks) -> Element {
    let lines = hooks.use_query(components::debug_lines());

    Group::el(lines.into_iter().flat_map(|(_, lines)| {
        lines
            .chunks(2)
            .map(|line| {
                let [start, end]: [Vec3; 2] = line.try_into().unwrap();

                Element::new()
                    .init_default(rect())
                    .with_default(main_scene())
                    .with(line_from(), start)
                    .with(line_to(), end)
                    .with(line_width(), 0.05)
                    .with(color(), vec4(1., 1., 1., 1.))
            })
            .collect::<Vec<_>>()
    }))
}
