mod models;
mod world;
mod interfazgraficamacroquad;

#[macroquad::main("Simulación Ecosistema")]
async fn main() {
    interfazgraficamacroquad::run_app().await;
}
