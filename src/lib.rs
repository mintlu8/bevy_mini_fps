#![doc = include_str!("../README.md")]

#[doc(hidden)]
pub use sysinfo;

/// N is only allowed to be a power of 2.
#[doc(hidden)]
pub struct RingBuffer<const N: usize> {
    buffer: [f32; N],
    ptr: usize,
    len: usize,
}

impl<const N: usize> RingBuffer<N> {
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push(&mut self, item: f32) {
        self.buffer[self.ptr] = item;
        self.ptr = (self.ptr + 1) & (N - 1);
        self.len = N.min(self.len + 1)
    }

    pub fn iter(&self) -> impl Iterator<Item = &f32> {
        self.buffer[0..self.len].iter()
    }
}
impl<const N: usize> Default for RingBuffer<N> {
    fn default() -> Self {
        Self { 
            buffer: [0.0; N], 
            ptr: 0, 
            len: 0
        }
    }
}

#[macro_export]
macro_rules! fps_plugin {
    () => {{
        
use ::bevy::prelude::*;
use ::core::fmt::Write;

fn __entity_count(world: &World) -> usize {
    world.entities().total_count()
}

#[allow(clippy::too_many_arguments)]
fn __diagnostic_system(
    entity_count: In<usize>,
    mut commands: Commands,
    mut sys_info: Local<$crate::sysinfo::System>,
    mut refresh_timer: Local<f32>,
    mut sys_timer: Local<f32>,
    mut time_buffer: Local<$crate::RingBuffer<128>>,
    cached: Local<::std::cell::OnceCell<[Entity; 5]>>,
    time: Res<Time>,
    mut text: Query<&mut Text>,
) {
    let [fps, max_ft, entities, cpu, ram] = *cached.get_or_init(|| {
        let mut result = [Entity::PLACEHOLDER; 5];
        const OUTER_MARGIN: f32 = 4.;
        const INNER_MARGIN: f32 = 2.;
        let font = TextFont {
            font_size: 16.0,
            ..default()
        };
        commands.spawn((
            Node {
                margin: UiRect { 
                    left: Val::Auto, 
                    right: Val::Px(OUTER_MARGIN), 
                    top: Val::Px(OUTER_MARGIN), 
                    bottom: Val::Auto 
                },
                ..Default::default()
            },
            BackgroundColor(Color::srgb(0.2, 0.2, 0.2))
        )).with_children(|c| {
            c.spawn(Node {
                flex_direction: FlexDirection::Column,
                margin: UiRect { 
                    left: Val::Px(INNER_MARGIN), 
                    right: Val::Px(INNER_MARGIN), 
                    top: Val::Px(INNER_MARGIN), 
                    bottom: Val::Px(INNER_MARGIN) 
                },
                align_items: AlignItems::FlexStart,
                ..Default::default()
            }).with_children(|c| {
                result = [
                    c.spawn((Node::default(), font.clone(), Text::new("FPS:"))).id(),
                    c.spawn((Node::default(), font.clone(), Text::new("Max Frametime:"))).id(),
                    c.spawn((Node::default(), font.clone(), Text::new("Entities:"))).id(),
                    c.spawn((Node::default(), font.clone(), Text::new("CPU Usage:"))).id(),
                    c.spawn((Node::default(), font.clone(), Text::new("Memory Usage:"))).id(),
                ];
            });
            c.spawn(Node {
                flex_direction: FlexDirection::Column,
                margin: UiRect { 
                    left: Val::Px(INNER_MARGIN), 
                    right: Val::Px(INNER_MARGIN), 
                    top: Val::Px(INNER_MARGIN), 
                    bottom: Val::Px(INNER_MARGIN) 
                },
                min_width: Val::Px(80.),
                align_items: AlignItems::FlexEnd,
                ..Default::default()
            }).with_children(|c| {
                result = [
                    c.spawn((Node::default(), font.clone(), Text::default())).id(),
                    c.spawn((Node::default(), font.clone(), Text::default())).id(),
                    c.spawn((Node::default(), font.clone(), Text::default())).id(),
                    c.spawn((Node::default(), font.clone(), Text::default())).id(),
                    c.spawn((Node::default(), font.clone(), Text::default())).id(),
                ];
            });
        });
        result
    });

    time_buffer.push(time.delta_secs());
    *sys_timer += time.delta_secs();
    if *sys_timer > 0.2 {
        *sys_timer -= 0.2;
        sys_info.refresh_memory();
        sys_info.refresh_cpu_usage();
    }
    
    *refresh_timer += time.delta_secs();
    if *refresh_timer > 0.05 {
        *refresh_timer -= 0.05;

        if let Ok(mut text) = text.get_mut(fps) {
            text.0.clear();
            let fps = time_buffer.len() as f32 / time_buffer.iter().sum::<f32>();
            let _ = write!(&mut text.0, "{:.0}", fps);
        };
        if let Ok(mut text) = text.get_mut(max_ft) {
            text.0.clear();
            let max_ft = time_buffer.iter().max_by(|a, b| 
                a.partial_cmp(b).unwrap_or(::core::cmp::Ordering::Equal)
            ).unwrap_or(&0.0);
            let _ = write!(&mut text.0, "{:.2}ms", max_ft * 1000.);
        };
        if let Ok(mut text) = text.get_mut(entities) {
            text.0.clear();
            let _ = write!(&mut text.0, "{}", *entity_count);
        };
        if let Ok(mut text) = text.get_mut(cpu) {
            let pct = sys_info.global_cpu_usage() as i32;
            text.0.clear();
            let _ = write!(&mut text.0, "{}%", pct);
        };
        if let Ok(mut text) = text.get_mut(ram) {
            let pct = (sys_info.used_memory() * 100).checked_div(sys_info.total_memory()).unwrap_or(0);
            text.0.clear();
            let _ = write!(&mut text.0, "{}%", pct);
        };
    }
}

fn __plugin(app: &mut App) {
    app.add_systems(First, __entity_count.pipe(__diagnostic_system));
}

__plugin
    }};
}
