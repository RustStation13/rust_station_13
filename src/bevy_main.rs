//! Beginning of the bevy initialization

use bevy::{app::{App, AppExit, PluginGroup}, window::{Window, WindowPlugin}, DefaultPlugins};

struct ConfiguredDefaultPlugins;

impl PluginGroup for ConfiguredDefaultPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        let backend = {
            
            #[cfg(any(target_os = "windows", target_os = "linux"))]
            {
                Some(bevy::render::settings::Backends::VULKAN)
            }

            #[cfg(target_os = "macos")]
            {
                Some(bevy::render::settings::Backends::METAL)
            }

            // Optional: fallback if none match (useful if you're targeting more OSes)
            #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
            {
                None
            }
        };

        DefaultPlugins
            .build()
            .set(bevy::render::RenderPlugin {
                render_creation: bevy::render::settings::RenderCreation::Automatic(bevy::render::settings::WgpuSettings {
                    backends: backend,
                    ..Default::default()
                }),
                synchronous_pipeline_compilation: false,
            })
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Rust Station 13".to_owned(),
                    ..Default::default()
                }),
                ..Default::default()
            })
    }
}

pub fn start_client() -> AppExit {
    App::new()
        .add_plugins(ConfiguredDefaultPlugins)
        .run()
}