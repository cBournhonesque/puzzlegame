use crate::prelude::*;
use cfg_if::cfg_if;
use config_rs::{Config, ConfigError, Environment, File, FileFormat};
use std::env;
use bevy::utils::petgraph::dot::Config;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Default)]
#[allow(unused)]
pub enum Mode {
    Singleplayer,
    #[default]
    Multiplayer,
}

#[derive(Debug, Deserialize, Default)]
#[allow(unused)]
pub struct GeneralSettings {
    pub mode: Mode,
    pub log_filter: String,
    pub log_level: String,
    pub seed: u64,
    pub relative_speed: f32,
    pub server_render: bool,
    pub framerate: f64,
}

#[derive(Debug, Deserialize, Default)]
#[allow(unused)]
pub struct ChatSettings {
    pub max_rows: u8,
    pub max_width: f32,
    pub history_display_sec: f32,
    pub history_limit: usize,
}

#[derive(Debug, Deserialize, Default)]
#[allow(unused)]
pub struct AudioSettings {
    pub max_playing_sounds: f32,
    pub max_distance_sound: f32,
    pub loop_volume_min: f32,
    pub loop_volume_max: f32,
}

#[derive(Debug, Deserialize, Default)]
#[allow(unused)]
pub struct SettingsSettings {
    pub refresh_time_per_second: f64,
}

#[derive(Debug, Deserialize, Default)]
#[allow(unused)]
pub struct MovementSettings {
    pub timestep_ms: f32,
    // pub max_acceleration_per_second: f32,
    pub seconds_to_reach_max_speed_at_max_acceleration: f32,
    pub seconds_to_reach_min_speed_from_max_speed: f32,
    pub max_distance_for_acceleration: f32,
    pub min_distance_for_max_acceleration: f32,
}

#[derive(Debug, Deserialize, Default)]
#[allow(unused)]
pub struct SpatialSettings {
    pub recreate_after: usize,
}

#[derive(Debug, Deserialize, Default)]
#[allow(unused)]
pub struct ParticleSettings {
    pub base_speed_ratio: f32,
    pub speed: f32,
    pub lifetime_sec: f32,
}

#[derive(Debug, Deserialize, Default)]
#[allow(unused)]
pub struct GlowSettings {
    pub size: f32,
    pub base_speed_ratio: f32,
    pub hard_base_speed_ratio: f32,
    pub size_over_speed_ratio: f32,
}

#[derive(Debug, Deserialize, Default)]
#[allow(unused)]
pub struct SparkSettings {
    pub double_spark_max_distance: f32,
    pub max_distance: f32,
    pub time_between_particle_sec: f32,
    pub particle_lifetime_sec: f32,
    pub width: f32,
}

#[derive(Debug, Deserialize, Default)]
#[allow(unused)]
pub struct LightningSettings {
    pub min_distance: f32,
    pub max_distance: f32,
    pub time_between_particle_sec: f32,
    pub particle_lifetime_sec: f32,
    pub width: f32,
}

#[derive(Debug, Deserialize, Default)]
#[allow(unused)]
pub struct PlayerSettings {
    pub size: Size_,
    pub base_speed: f32,
    pub max_speed: f32,
    pub player_id: usize,
    pub start_size: f32,
    pub crown_size: f32,
    pub start_score: f32,
    pub segment_width: f32,
    pub spawn_free_radius: f32,
    pub head_z_value: f32,
    pub segment_z_value: f32,
    pub name_y_offset: f32,
    pub name_font_size: f32,
    pub death_animation_millis: u64,
    pub spark: SparkSettings,
    pub lightning: LightningSettings,
    pub particles: ParticleSettings,
    pub glow: GlowSettings,
}

#[derive(Debug, Deserialize, Default)]
#[allow(unused)]
pub struct FoodPlayerDeath {
    pub food_per_interval: u32,
    pub orthogonal_jitter: f32,
    pub food_per_size: f32,
}

// TODO: maybe implement a custom format for randomization? like size randomization? [a, b]
#[derive(Debug, Deserialize, Default)]
#[allow(unused)]
pub struct FoodSettings {
    pub speed_giver: f32,
    pub score_giver: f32,
    pub radius: f32,
    pub collision_radius: f32,
    pub size_giver: f32,
    pub spawn_timer_seconds: f32,
    pub z_value: f32,
    pub death_animation_millis: u64,
    pub death_animation_size: f32,
    pub player_death: FoodPlayerDeath,
}

#[derive(Debug, Deserialize, Default)]
#[allow(unused)]
pub struct CameraSettings {
    pub height: f32,
    pub start_scale: f32,
    pub end_scale: f32,
    pub num_food_scale: f32,
}

#[derive(Debug, Deserialize, Default)]
#[allow(unused)]
pub struct ContactSettings {
    pub death_collision_radius: f32,
}

#[derive(Debug, Deserialize, Default)]
#[allow(unused)]
pub struct MapSettings {
    pub size: Size_,
    pub delta: f32,
    pub max_food_count: usize,
    pub spawn_buffer: f32,
    pub z_value: f32,
}

#[derive(Debug, Deserialize, Default)]
#[allow(unused)]
pub struct StatsSettings {
    pub num_scores_leaderboard: usize,
    pub top_scores_timestep_millis: f32,
}

#[derive(Debug, Deserialize, Default)]
#[allow(unused)]
pub struct NetworkSettings {
    pub tick_interval: u64,
    pub client_prediction: bool,
    pub show_confirmed: bool,
    pub server_reconciliation: bool,
    pub entity_interpolation: bool,
    pub interpolation_delay_ms: f64,
    pub interpolate_controlled: bool,
    pub server_add_core: bool,
}

#[derive(Debug, Deserialize, Default)]
#[allow(unused)]
pub struct AISettings {
    pub turn_reaction_ms: f32,
}

#[derive(Debug, Deserialize, Resource, Default)]
#[allow(unused)]
pub struct Settings {
    pub general: GeneralSettings,
    pub audio: AudioSettings,
    pub chat: ChatSettings,
    pub settings: SettingsSettings,
    pub movement: MovementSettings,
    pub spatial: SpatialSettings,
    pub player: PlayerSettings,
    pub food: FoodSettings,
    pub camera: CameraSettings,
    pub contact: ContactSettings,
    pub map: MapSettings,
    pub stats: StatsSettings,
    pub network: NetworkSettings,
    pub ai: AISettings,
    pub color: ColorSettings,
}

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        let settings = Settings::new().unwrap();

        cfg_if! {
            if #[cfg(not(target_arch = "wasm32"))] {
                let refresh = settings.settings.refresh_time_per_second;
                app.add_systems(
                    FixedUpdateSettings,
                    refresh_settings
                );
            } else {}
        }
        app.insert_resource(settings);
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn refresh_settings(mut settings: ResMut<Settings>) {
    *settings = Settings::new().unwrap();
}

impl Settings {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn new() -> Result<Self, ConfigError> {
        let run_mode = env::var("RUN_MODE").unwrap_or("dev".into());

        let config_prefix: String = "../shared/".into();
        let config_folder = format!("{}src/resources/config", config_prefix);

        let c = Config::builder()
            // Start off by merging in the "default" configuration file
            .add_source(File::with_name(&format!("{}/default", config_folder)))
            // Add in the current environment file
            // Default to 'development' env
            // Note that this file is _optional_
            .add_source(File::with_name(&format!("{}/{}", config_folder, run_mode)).required(false))
            // Add in a local configuration file
            // This file shouldn't be checked in to git
            .add_source(File::with_name(&format!("{}/local", config_folder)).required(false))
            // Add in settings from the environment (with a prefix of APP)
            // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
            .add_source(Environment::with_prefix("app"))
            .build()?;

        c.try_deserialize()
    }

    #[cfg(target_arch = "wasm32")]
    fn new() -> Result<Self, ConfigError> {
        let c = Config::builder()
            // Start off by merging in the "default" configuration file
            .add_source(File::from_str(include_str!("default.yaml"), FileFormat::Yaml))
            // Add in the current environment file
            // Default to 'development' env
            // Note that this file is _optional_
            .add_source(
                File::from_str(include_str!(concat!(env!("RUN_MODE"), ".yaml")), FileFormat::Yaml).required(false),
            )
            .build()?;

        c.try_deserialize()
    }
}
