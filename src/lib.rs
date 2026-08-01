use std::sync::Arc;
use std::path::Path;

use pumpkin::plugin::Context;
use pumpkin_api_macros::{plugin_impl, plugin_method};
use tracing::*;

use serde::{Deserialize, Serialize};

// NOTE: verify these import paths against docs.pumpkinmc.org — the native
// crate may expose these under different module paths than pumpkin_plugin_api.
use pumpkin::plugin::api::text::TextComponent;
use pumpkin::plugin::api::common::RgbColor;
use pumpkin::plugin::api::events::{EventData, EventHandler, EventPriority, PlayerJoinEvent, PlayerLeaveEvent};
use pumpkin::plugin::api::command::{Command, CommandHandler, CommandSender, ConsumedArgs};
use pumpkin::plugin::api::permission::{Permission, PermissionDefault};
use pumpkin::Server;

#[derive(Serialize, Deserialize)]
struct PluginConfig {
    join_message: String,
    leave_message: String,
}

impl Default for PluginConfig {
    fn default() -> Self {
        Self {
            join_message: "{player} joined the server".to_string(),
            leave_message: "{player} left the server".to_string(),
        }
    }
}

fn load_config(context: &Context) -> PluginConfig {
    let data_folder = context.get_data_folder();
    info!("Data folder path {}", data_folder);

    if let Err(e) = std::fs::create_dir_all(&data_folder) {
        error!("Failed to create data folder '{}': '{}'", data_folder, e);
    }

    let config_path = Path::new(&data_folder).join("config.yml");

    if let Ok(contents) = std::fs::read_to_string(&config_path) {
        serde_yaml::from_str(&contents).unwrap_or_default()
    } else {
        let default_config = PluginConfig::default();
        if let Ok(yaml) = serde_yaml::to_string(&default_config) {
            let _ = std::fs::write(&config_path, yaml);
        }
        default_config
    }
}

struct OnPlayerJoin {
    join_message: String,
}

impl EventHandler<PlayerJoinEvent> for OnPlayerJoin {
    fn handle(&self, _server: Server, mut event: EventData<PlayerJoinEvent>) -> EventData<PlayerJoinEvent> {
        let name = event.player.get_name();
        let text = self.join_message.replace("{player}", &name);

        let mut message = TextComponent::text(&text);
        message.color_rgb(RgbColor { r: 0x00, g: 0x99, b: 0xFF });

        event.join_message = message;
        event
    }
}

struct OnPlayerLeave {
    leave_message: String,
}

impl EventHandler<PlayerLeaveEvent> for OnPlayerLeave {
    fn handle(&self, _server: Server, mut event: EventData<PlayerLeaveEvent>) -> EventData<PlayerLeaveEvent> {
        let name = event.player.get_name();
        let text = self.leave_message.replace("{player}", &name);

        let mut message = TextComponent::text(&text);
        message.color_rgb(RgbColor { r: 0x00, g: 0x99, b: 0xFF });

        event.leave_message = message;
        event
    }
}

struct TestCommandHandler;

impl CommandHandler for TestCommandHandler {
    fn handle(&self, sender: CommandSender, _server: Server, _args: ConsumedArgs) -> Result<i32, pumpkin::plugin::api::command::CommandError> {
        let mut msg = TextComponent::text("Napisal si /test");
        msg.color_rgb(RgbColor { r: 0x00, g: 0x99, b: 0xFF });
        sender.send_message(msg);
        Ok(0)
    }
}

pub fn init_test_command() -> Command {
    let names = ["test".to_string(), "testcommand".to_string()];
    Command::new(&names, "My first cmd").execute(TestCommandHandler)
}

#[plugin_impl]
pub struct HelloPlugin;

impl HelloPlugin {
    pub fn new() -> Self {
        HelloPlugin
    }
}

impl Default for HelloPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl HelloPlugin {
    #[plugin_method]
    async fn on_load(&mut self, context: Arc<Context>) -> Result<(), String> {
        info!("Hello from the example plugin!");

        let config = load_config(&context);

        context
            .register_event_handler::<PlayerJoinEvent, _>(
                OnPlayerJoin { join_message: config.join_message },
                EventPriority::Normal,
                true,
            )
            .map_err(|e| e.to_string())?;

        context
            .register_event_handler::<PlayerLeaveEvent, _>(
                OnPlayerLeave { leave_message: config.leave_message },
                EventPriority::Normal,
                true,
            )
            .map_err(|e| e.to_string())?;

        context
            .register_permission(&Permission {
                node: "command:test".to_string(),
                description: "Important test perms".to_string(),
                default: PermissionDefault::Allow,
                children: Vec::new(),
            })
            .map_err(|e| e.to_string())?;

        context.register_command(init_test_command(), "command:test");

        Ok(())
    }

    #[plugin_method]
    async fn on_unload(&mut self, _context: Arc<Context>) -> Result<(), String> {
        info!("Example plugin unloaded. Goodbye!");
        Ok(())
    }
}