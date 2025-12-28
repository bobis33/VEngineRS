use winit::event::WindowEvent;
use winit::event_loop::{ControlFlow, EventLoop};

mod system_info;
mod window;
mod cli;

use system_info::SystemInfo;
use window::winit_app;

use cli::startup::cli_startup_app::{Cli, Command};
use cli::runtime::registry::setup_runtime_cli;

fn main() {
    let startup_cli = Cli::parse_cli();

    let runtime_cli = std::sync::Arc::new(setup_runtime_cli());
    let runtime_cli_thread = runtime_cli.clone();

    std::thread::spawn(move || {
        use std::io::{self, BufRead};
        for line in io::stdin().lock().lines() {
            let line = line.unwrap();
            if let Err(e) = runtime_cli_thread.execute(&line) {
                println!("Error: {}", e);
            }
        }
    });

    match startup_cli.command.unwrap_or(Command::Run { width: 1920, height: 1080 }) {
        Command::Info => {
            SystemInfo::new().print();
        }

        Command::Run { width, height } => {
            SystemInfo::new().print();
            entry(EventLoop::new().unwrap(), width, height);
        }
    }
}


fn entry(event_loop: EventLoop<()>, width: u32, height: u32) {
    let app = winit_app::WinitAppBuilder::with_init(
        move |elwt| {
            winit_app::make_window(elwt, |attrs| {
                attrs.with_title("VEngine RS").with_inner_size(
                    winit::dpi::PhysicalSize { width, height },
                )
            })
        },
        |_elwt, _window| (),
    )
        .with_event_handler(|window, _surface, window_id, event, elwt| {
            elwt.set_control_flow(ControlFlow::Wait);

            match event {
                WindowEvent::CloseRequested if window.id() == window_id => {
                    elwt.exit();
                }

                WindowEvent::KeyboardInput { event, .. } => {
                    if let winit::event::KeyEvent {
                        state: winit::event::ElementState::Pressed,
                        logical_key: winit::keyboard::Key::Named(winit::keyboard::NamedKey::Escape),
                        ..
                    } = event
                    {
                        elwt.exit();
                    }
                }

                _ => {}
            }
        });

    winit_app::run_app(event_loop, app);
}
