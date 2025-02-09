use std::env;
use std::process::{Command, exit};
use std::io::{self, Write};
use std::sync::OnceLock;

static PROJECT_NAME: OnceLock<String> = OnceLock::new();

struct Installer;

impl Installer {
    fn new() -> Self {
        info("Welcome to the 🌙 MoonShine Installer 🌙");
        Self
    }

    fn laravel(self) -> Self {
        install_laravel();
        self
    }

    fn moonshine(self) -> Self {
        install_moonshine();
        self
    }

    fn localization(self) -> Self {
        install_localization();
        self
    }

    fn addons(self) -> Self {
        install_addons();
        self
    }

    fn run(self) {
        info("✅ Installation completed! Run 'php artisan serve' to start your project.");
        exit(0);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args[1] == "new" {
        if args.len() < 2 {
            error("❌ Error: Missing project name. Usage: moonshine new <project-name>");
            exit(1);
        }

        set_project_name(args[2].clone())
    } else {
        set_project_name(ask("Enter your Laravel project name: "))
    }

    Installer::new()
        .laravel()
        .moonshine()
        .localization()
        .addons()
        .run();
}

fn info(msg: &str) {
    hr();
    println!("{}", msg);
    hr()
}

fn error(msg: &str) {
    hr();
    eprintln!("{}", msg);
    hr()
}

fn hr() {
    println!("{}", "-".repeat(50));
}

fn ask(question: &str) -> String {
    print!("{}", question);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn confirm(question: &str) -> bool {
    let question = format!("{} (yes/no)", question);
    let answer = ask(&question);
    answer.to_lowercase().trim() == "" || answer.to_lowercase() == "yes" || answer.to_lowercase() == "y"
}

fn set_project_name(name: String) {
    PROJECT_NAME.set(name).expect("Failed to set project name");
}

fn get_project_name() -> &'static str {
    PROJECT_NAME.get().expect("Project name not set")
}

fn composer(package_name: &str) {
    let project_name = get_project_name();
    let status = Command::new("composer")
        .args(["require", package_name])
        .current_dir(project_name)
        .status()
        .expect(&format!("Failed to install {}", package_name));

    if status.success() {
        info(&format!("✅ Installed: {}", package_name));
    } else {
        error(&format!("❌ Failed: {}", package_name));
    }
}

fn artisan(name: &str, command_name: &str, options: &[&str]) {
    let mut args = vec!["artisan", command_name];
    args.extend_from_slice(options);

    let status = Command::new("php")
        .args(&args)
        .current_dir(get_project_name())
        .status()
        .expect(&format!("Failed to run {}", command_name));

    if status.success() {
        info(&format!("✅ {} installed successfully!", name));
    } else {
        error(&format!("❌ {} installation failed!", name));
    }
}

fn install_laravel() {
    let project_name = get_project_name();
    info("🚀 Installing Laravel...");
    let status = Command::new("composer")
        .args(["create-project", "laravel/laravel", project_name])
        .status()
        .expect("Failed to install Laravel");
    if status.success() {
        info("✅ Laravel installed successfully!");
    } else {
        error("❌ Laravel installation failed!");
    }
}

fn install_moonshine() {
    info("🌙 Installing MoonShine...");
    composer("moonshine/moonshine");
    artisan("MoonShine","moonshine:install", &[]);
}

fn install_localization() {
    if confirm("📦 Do you want to install russian language?") {
        composer("moonshine/ru");
        artisan("Russian localization","vendor:publish", &["--provider=\"MoonShine\\Ru\\Providers\\RuServiceProvider\""]);
    }
}

fn install_addons() {
    if confirm("📦 Do you want to install import/export package?") {
        composer("moonshine/import-export");
    }

    if confirm("📦 Do you want to install permissions package?") {
        composer("moonshine/permissions");
        artisan("Migrate", "migrate", &[])
    }
}
