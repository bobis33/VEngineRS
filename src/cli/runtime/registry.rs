use super::cli_runtime_app::RuntimeCli;
pub fn setup_runtime_cli() -> RuntimeCli {
    let mut cli = RuntimeCli::new();

    cli.register("help", |_args| {
        println!("Available commands:");
        for c in ["help", "echo", "quit"] {
            println!(" - {}", c);
        }
        Ok(())
    });

    cli.register("echo", |_args| {
        println!("{}", _args.join(" "));
        Ok(())
    });

    cli.register("quit", |_| {
        println!("Use Escape to quit the application.");
        Ok(())
    });

    cli
}