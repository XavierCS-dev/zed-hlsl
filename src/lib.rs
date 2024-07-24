struct HLSLExtension {}

impl zed_extension_api::Extension for HLSLExtension {
    fn new() -> Self {
        println!("test");
        Self {}
    }

    fn language_server_command(
        &mut self,
        _: &zed_extension_api::LanguageServerId,
        _: &zed_extension_api::Worktree,
    ) -> zed_extension_api::Result<zed_extension_api::Command> {
        Ok(zed_extension_api::Command {
            command: String::new(),
            args: Vec::new(),
            env: Vec::new(),
        })
    }
}

zed_extension_api::register_extension!(HLSLExtension);
//
