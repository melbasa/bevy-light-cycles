use std::time::Duration;

use bevy::prelude::*;
use splines::Spline;

#[derive(Component, Clone)]
pub struct IlluminanceCycle {
    pub spline: Spline<f32, f32>,
    pub timer: Timer
}

pub struct LightCyclesPlugin;

impl Plugin for LightCyclesPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(LightCyclesPlugin::do_cycling);
    }
}

impl LightCyclesPlugin {
    fn do_cycling(
        time: Res<Time>,
        mut lights: Query<(&mut DirectionalLight, &mut IlluminanceCycle)>
    ) {
        lights.iter_mut().for_each(|(mut direction, mut cycle)| {
            let elapsed = cycle.timer.tick(time.delta()).elapsed_secs();
            info!("Cycling: {}", elapsed);
            if let Some(strength) = cycle.spline.sample(elapsed) {
                direction.illuminance = strength;
            }
        });
    }
}