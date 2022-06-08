use std::env::current_dir;

fn main() -> wry::Result<()> {

    let cwd = current_dir().unwrap();


    use wry::{
      application::{
        event::{Event, StartCause, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
      },
      webview::WebViewBuilder,
    };
  
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
      .with_title("Hello World")
      .build(&event_loop)?;

    let _webview = WebViewBuilder::new(window)?
      .with_url(&format!("file://{}", cwd.join("index.html").to_str().unwrap()))?
      .with_devtools(true)
      .build()?;
  
    event_loop.run(move |event, _, control_flow| {
      *control_flow = ControlFlow::Wait;
  
      match event {
        Event::NewEvents(StartCause::Init) => println!("Wry has started!"),
        Event::WindowEvent {
          event: WindowEvent::CloseRequested,
          ..
        } => *control_flow = ControlFlow::Exit,
        _ => (),
      }
    });
  }