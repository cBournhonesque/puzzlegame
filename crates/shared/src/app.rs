//! This file contains the main plugin for the game. It is responsible for loading all other plugins and systems.
use bevy::prelude::*;
use bevy::window::WindowResolution;

pub struct MainPlugin{
    // is the main plugin running on the client or the server
    is_server: bool,
    // do we want to enable rendering
    render: bool,
}

impl MainPlugin {
    fn new(app: &App, is_server: bool, render: bool) -> MainPlugin {
        Self {
            is_server,
            render,
        }
    }
}

pub fn build_app(is_server: bool) -> App {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    let mut app = App::new();

    app.add_plugins(WindowPlugin {
        primary_window: Some(Window {
            // mode: WindowMode::Fullscreen,
            resolution: WindowResolution::new(800., 800.),
            // fill the entire browser window
            // fit_canvas_to_parent: true,
            canvas: Some("#bevy".to_string()),
            ..default()
        }),
        ..default()
    });

    app
}



/// Just [MinimalPlugins]
pub fn minimal_app() -> App {
    let mut app = App::new();
    app.add_plugin(MinimalPlugins);
    app
}

/// minimal app with the simulation plugin
pub fn simulation_app() -> App {
    let mut app = App::new();
    app.add_plugin(SimulationPlugin);
    app
}

/// Minimal app with the simulation plugin and interaction plugin
pub fn interaction_app() -> App {
    let mut app = App::new();
    app.add_plugin(ClientPlugin);
    app
}