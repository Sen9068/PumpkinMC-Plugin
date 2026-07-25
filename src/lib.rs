use pumpkin_plugin_api::commands::CommandHandler;
use pumpkin_plugin_api::player::Player;
use pumpkin_plugin_api::{Context, Plugin, PluginMetadata, permission};
use pumpkin_plugin_api::command::{Command, CommandSender, ConsumedArgs};
use pumpkin_plugin_api::permission::{Permission, PermissionDefault};
use pumpkin_plugin_api::Server;
use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::events::{EventData, EventHandler, EventPriority, PlayerJoinEvent, PlayerLeaveEvent};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;


use pumpkin_plugin_api::common::NamedColor;
use pumpkin_plugin_api::common::RgbColor;


use serde::ser;
use tracing::*;

#[derive(serde::Deserialize, serde::Serialize)]
struct PluginConfig {
    join_message: String,
}

impl Default for PluginConfig {
    fn default() -> Self {
        Self {
            join_message: "{player} joined the server".to_string(),
        }
    }
}


struct OnPlayerJoin {
    join_message: String,
}

impl EventHandler<PlayerJoinEvent> for OnPlayerJoin {
    fn handle(&self, _server: Server, mut event: EventData<PlayerJoinEvent>) -> EventData<PlayerJoinEvent> {
        info!("Testoksjdksndksanjdkjd");
        let name = event.player.get_name();
        let text = self.join_message.replace("{player}", &name);
        let message: TextComponent = TextComponent::text(&format!("{} joined the server", name));
        message.color_rgb(RgbColor { r: 0x00, g: 0x99, b: 0xFF });
        event.join_message = TextComponent::text(&text);
        event
    }
}


struct OnPlayerLeave;

impl EventHandler<PlayerLeaveEvent> for OnPlayerLeave {
    fn handle(&self, _server: Server, mut event: EventData<PlayerLeaveEvent>) -> EventData<PlayerLeaveEvent> {
        let name = event.player.get_name();
        let message: TextComponent = TextComponent::text(&format!("{} left the server", name));
        message.color_rgb(RgbColor { r:0x00, g: 0x99, b: 0xFF });
        event.leave_message = message;
        event
    }
}

struct TestCommandHandler;

impl CommandHandler for TestCommandHandler {
    fn handle(
        &self,
        sender: CommandSender,
        _server: Server,
        _args: ConsumedArgs,
    ) -> Result<i32, pumpkin_plugin_api::command::CommandError> {
        let msg = TextComponent::text("Napisal si /test");
        msg.color_rgb(RgbColor { r: 0x00, g: 0x99, b: 0xFF });
        sender.send_message(msg);
        Ok(0)
    }
}

struct IdkCommandHandler;

impl CommandHandler for IdkCommandHandler {
    fn handle(
        &self,
        sender: CommandSender,
        _server: Server
        , _args: ConsumedArgs
    ) -> Result<i32, pumpkin_plugin_api::command::CommandError> {
        let msg: TextComponent = TextComponent::text("plain");
        msg.color_rgb(RgbColor { r: 0x00, g: 0x99, b: 0xFF });
        sender.send_message(msg);
        Ok(0)
    }
}


pub fn init_test_command() -> Command {
    let testnames = ["test".to_string(), "testcommand".to_string()];
    let testdescription = "My first cmd";

    Command::new(&testnames, testdescription). execute(TestCommandHandler)

}

pub fn init_idk_command() -> Command {

    let names = ["idk".to_string()];
    let description = "My first cmd";

    Command::new(&names, description).execute(IdkCommandHandler)
}

struct HelloPlugin;
impl Plugin for HelloPlugin {
    fn new() -> Self {
        HelloPlugin
    }

    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            name: "command".into(),
            version: env!("CARGO_PKG_VERSION").into(),
            authors: vec!["Bjorn".into()],
            description: "A simple example plugin".into(),
            dependencies: vec![],
            permissions: vec![],
        }
    }

    fn on_load(&mut self, context: Context) -> pumpkin_plugin_api::Result<()> {
        info!("Hello from the example plugin!");

    
    fn load_config(context: &Context) -> PluginConfig {

        let data_folder = context.get_data_folder();
        info!("Data folder path {}", data_folder);

        let config_path = Path::new(&data_folder).join("config.yml");

        if let Ok(contents) = fs::read_to_string(&config_path) {
            serde_yaml::from_str(&contents).unwrap_or_default()

        } else {
            let default_config = PluginConfig::default();
            let _ = fs::create_dir_all(&data_folder);
            if let Ok(yaml) = serde_yaml::to_string(&default_config) {
                let _ = fs::write(&config_path, yaml);

            if let Err(e) = fs::create_dir_all(&data_folder) {
                error!("Failed to create data folder '{}': '{}'", data_folder, e);
            }

            
        }
        default_config
        }
    }

        let config = load_config(&context);

        context.register_event_handler::<PlayerJoinEvent, _>(
            OnPlayerJoin { join_message: config.join_message },
            EventPriority::Normal,
            true,
        )?;

        context.register_event_handler::<PlayerLeaveEvent, OnPlayerLeave>(
            OnPlayerLeave,
            EventPriority::Normal,
            true,

        )?;
        

        context.register_permission(&Permission {
            node: "command:test".to_string(),
            description: "Important test perms".to_string(),
            default: PermissionDefault::Allow,
            children: Vec::new(),
        })?;


        context.register_permission(&Permission {
            node: "command:idk".to_string(),
            description: "idk command".to_string(),
            default: PermissionDefault::Allow,
            children: Vec::new(),
        })?;

        context.register_command(init_test_command(), "command:test");
        context.register_command(init_idk_command(), "command:idk");

        Ok(())
    }

    fn on_unload(&mut self, _context: Context) -> pumpkin_plugin_api::Result<()> {
        info!("Example plugin unloaded. Goodbye!");
        Ok(())
    }
}

pumpkin_plugin_api::register_plugin!(HelloPlugin);