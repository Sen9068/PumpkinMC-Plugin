use pumpkin_plugin_api::commands::CommandHandler;
use pumpkin_plugin_api::{Context, Plugin, PluginMetadata, permission};
use pumpkin_plugin_api::command::{Command, CommandSender, ConsumedArgs};
use pumpkin_plugin_api::permission::{Permission, PermissionDefault};
use pumpkin_plugin_api::Server;
use pumpkin_plugin_api::text::TextComponent;

use pumpkin_plugin_api::common::NamedColor;
use pumpkin_plugin_api::common::RgbColor;


use tracing::*;

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