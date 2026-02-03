use std::env;
use std::fs;
use std::os::unix::process::CommandExt;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: dwim <script.md>");
        eprintln!();
        eprintln!("Or use as a shebang:");
        eprintln!("  #!/usr/bin/env dwim");
        eprintln!("  # Deploy my app to production at 3am on a Friday");
        std::process::exit(1);
    }

    let script_path = &args[1];

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

    let prompt = prompt.trim();

    if prompt.is_empty() {
        eprintln!("Nothing to do. Write some markdown describing what you want.");
        std::process::exit(1);
    }

    // YOLO MODE ENGAGED
    // --dangerously-skip-permissions: skip all permission prompts
    // --yes: auto-accept everything
    // -p: pass prompt directly
    let err = Command::new("claude")
        .arg("--dangerously-skip-permissions")
        .arg("--yes")
        .arg("-p")
        .arg(prompt)
        .exec();

    // exec() only returns if it fails
    eprintln!("Failed to exec claude: {}", err);
    eprintln!("Is claude CLI installed? Try: npm install -g @anthropic-ai/claude-code");
    std::process::exit(1);
}
