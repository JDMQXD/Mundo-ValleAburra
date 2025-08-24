use macroquad::prelude::*;
use interfaz::Interfaz;

mod interfaz;

fn window_conf() -> Conf {
    Conf {
        window_title: "MyGame".to_string(),
        fullscreen: true,         
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut app = Interfaz::new();

    loop {
        clear_background(BLACK);

        app.dibujar();

        next_frame().await;
    }
}
