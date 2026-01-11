use zed_extension_api as zed;

struct MojoExtension {
    cached_binary_path: Option<String>,
}

impl MojoExtension {
    fn language_server_binary(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {

        let get_args = |path: &str| -> Vec<String> {
            if path.ends_with("mojo") {
                vec!["lsp".to_string()]
            } else {
                vec![]
            }
        };

        // Check cache first.
        if let Some(path) = &self.cached_binary_path {
            let args = get_args(path);
            
            if path.starts_with('/') {
                if std::fs::metadata(path).is_ok() {
                    return Ok(zed::Command {
                        command: path.clone(),
                        args,
                        env: Default::default(),
                    });
                }
            } else {
                // For relative paths (e.g. .venv/...), we must execute via shell to ensure
                // they resolve relative to the Project Root (CWD), not the extension directory.
                let script = format!("exec {} {}", path, args.join(" "));
                return Ok(zed::Command {
                    command: "/bin/sh".to_string(),
                    args: vec!["-c".to_string(), script],
                    env: Default::default(),
                });
            }
            self.cached_binary_path = None;
        }

        // 1. Check VIRTUAL_ENV (activated)
        if let Some(venv_path) = worktree.shell_env().iter().find(|(key, _)| key == "VIRTUAL_ENV").map(|(_, value)| value.to_string()) {
            let check_paths = vec![
                format!("{}/bin/mojo-lsp-server", venv_path),
                format!("{}/bin/mojo-lsp", venv_path),
                format!("{}/bin/mojo", venv_path),
            ];

            for path in check_paths {
                if std::fs::metadata(&path).is_ok() {
                    self.cached_binary_path = Some(path.clone());
                    return Ok(zed::Command {
                        command: path.clone(),
                        args: get_args(&path),
                        env: Default::default(),
                    });
                }
            }
        }

        // 2. Check for local project venvs (.venv/pyvenv.cfg presence)
        // We look for config files because checking binary existence is expensive/restricted.
        let local_venv_check_paths = vec![
            (".venv/pyvenv.cfg", ".venv/bin/mojo-lsp-server"),
            (".venv/pyvenv.cfg", ".venv/bin/mojo-lsp"),
            (".venv/pyvenv.cfg", ".venv/bin/mojo"),
            ("venv/pyvenv.cfg", "venv/bin/mojo-lsp-server"),
            ("venv/pyvenv.cfg", "venv/bin/mojo-lsp"),
            ("venv/pyvenv.cfg", "venv/bin/mojo"),
        ];
        
        for (config_path, binary_path) in local_venv_check_paths {
            if worktree.read_text_file(config_path).is_ok() {
                 self.cached_binary_path = Some(binary_path.to_string());
                 
                 let args = get_args(binary_path);
                 let script = format!("exec {} {}", binary_path, args.join(" "));
                 
                 return Ok(zed::Command {
                    command: "/bin/sh".to_string(),
                    args: vec!["-c".to_string(), script],
                    env: Default::default(),
                 });
            }
        }

        // 3. Check PATH
        let path_candidates = vec!["mojo-lsp-server", "mojo-lsp", "mojo-language-server", "mojo"];
        for binary_name in path_candidates {
            if let Some(path) = worktree.which(binary_name) {
                self.cached_binary_path = Some(path.clone());
                return Ok(zed::Command {
                    command: path.clone(),
                    args: get_args(&path),
                    env: Default::default(),
                });
            }
        }

        // 4. Check standard Modular installation (fallback)
        if let Some(home) = std::env::var("HOME").ok() {
            let modular_paths = vec![
                format!("{}/.modular/pkg/packages.modular.com_mojo/bin/mojo-lsp-server", home),
                format!("{}/.modular/pkg/packages.modular.com_mojo/bin/mojo-lsp", home),
                 format!("{}/.modular/bin/mojo-lsp-server", home),
                format!("{}/.modular/bin/mojo-lsp", home),
            ];
            for path in modular_paths {
                 if std::fs::metadata(&path).is_ok() {
                    self.cached_binary_path = Some(path.clone());
                     return Ok(zed::Command {
                        command: path.clone(),
                        args: get_args(&path),
                        env: Default::default(),
                    });
                }
            }
        }

        // If we still haven't found it, dump the environment to help the user debug.
        let shell_env_path = worktree.shell_env().iter().find(|(key, _)| key == "PATH").map(|(_, v)| v.to_string()).unwrap_or("Unset".to_string());
        let path_list = shell_env_path.split(':').collect::<Vec<_>>().join("\n- ");
        
        let venv_env = worktree.shell_env().iter().find(|(key, _)| key == "VIRTUAL_ENV").map(|(_, v)| v.to_string()).unwrap_or("Unset".to_string());
        let cwd = std::env::current_dir().map(|p| p.to_string_lossy().to_string()).unwrap_or("Unknown".to_string());
        
        Err(format!(
            "Mojo Language Server not found.\n\n\
            Checked:\n\
            - VIRTUAL_ENV: {}\n\
            - Project .venv/venv roots\n\
            - PATH and Standard Modular locations\n\n\
            Debug Info:\n\
            - CWD Check: {}\n\
            - VENV Check: {}\n\
            - PATH Check:\n- {}", 
            venv_env, cwd, venv_env, path_list
        ).into())
    }
}

impl zed::Extension for MojoExtension {
    fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        self.language_server_binary(language_server_id, worktree)
    }
}

zed::register_extension!(MojoExtension);
