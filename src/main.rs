use std::env;
use std::fs::{self, File};
use std::io::{self, IsTerminal, Read, Write};
use std::os::unix::process::CommandExt;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: dwim <script.md> [args...]");
        eprintln!();
        eprintln!("Or use as a shebang:");
        eprintln!("  #!/usr/bin/env dwim");
        eprintln!("  # Deploy my app to production at 3am on a Friday");
        std::process::exit(1);
    }

    let script_path = &args[1];
    let script_args = &args[2..];

    let content = match fs::read_to_string(script_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to read {}: {}", script_path, e);
            std::process::exit(1);
        }
    };

    // Strip shebang line if present
    let prompt = if content.starts_with("#!") {
        content.lines().skip(1).collect::<Vec<_>>().join("\n")
    } else {
        content
    };

    let prompt = prompt.trim_start().to_string();

    if prompt.is_empty() {
        eprintln!("Nothing to do. Write some markdown describing what you want.");
        std::process::exit(1);
    }

    // Build the full prompt with context
    let mut full_prompt = prompt;

    // Append arguments if any
    if !script_args.is_empty() {
        full_prompt.push_str("\n\n---\n\n## Arguments\n\n");
        for (i, arg) in script_args.iter().enumerate() {
            full_prompt.push_str(&format!("- `${}`: `{}`\n", i + 1, arg));
        }
    }

    // Handle stdin if not a terminal
    if !io::stdin().is_terminal() {
        let mut stdin_content = Vec::new();
        if let Ok(n) = io::stdin().read_to_end(&mut stdin_content) {
            if n > 0 {
                // Write to temp file
                let stdin_path = format!("/tmp/dwim-stdin-{}", std::process::id());
                match File::create(&stdin_path).and_then(|mut f| f.write_all(&stdin_content)) {
                    Ok(()) => {
                        full_prompt.push_str("\n\n---\n\n## Stdin\n\n");
                        full_prompt.push_str(&format!(
                            "Input was piped to this script. Contents are available at: `{}`\n",
                            stdin_path
                        ));
                    }
                    Err(e) => {
                        eprintln!("Warning: failed to write stdin to temp file: {}", e);
                    }
                }
            }
        }
    }

    // YOLO MODE ENGAGED
    // --dangerously-skip-permissions: skip all permission prompts
    // --yes: auto-accept everything
    // -p: pass prompt directly
    let err = Command::new("claude")
        .arg("--dangerously-skip-permissions")
        .arg("--yes")
        .arg("-p")
        .arg(&full_prompt)
        .exec();

    // exec() only returns if it fails
    eprintln!("Failed to exec claude: {}", err);
    eprintln!("Is claude CLI installed? Try: npm install -g @anthropic-ai/claude-code");
    std::process::exit(1);
}
