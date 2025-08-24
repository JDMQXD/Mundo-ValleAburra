use macroquad::prelude::*;

#[derive(PartialEq)]
pub enum Pantalla {
    Menu,
    Opciones,
    Juego,
    Añadir
}

pub struct Interfaz {
    pub pantalla_actual: Pantalla,
}

impl Interfaz {
    pub fn new() -> Self {
        Self {
            pantalla_actual: Pantalla::Menu,
        }
    }


    pub fn dibujar(&mut self) {
        match self.pantalla_actual {
            Pantalla::Menu => {
                draw_text("Bienvenido a mrdar juego", 200.0, 100.0, 40.0, WHITE);

                if boton("Jugar", 200.0, 150.0, 200.0, 50.0) {
                    self.pantalla_actual = Pantalla::Juego;
                }

                if boton("Opciones", 200.0, 220.0, 200.0, 50.0) {
                    self.pantalla_actual = Pantalla::Opciones;
                }
                if boton("Salir", 200.0, 400.0, 200.0, 50.0) {
                    std::process::exit(0); 
                }
            }
            Pantalla::Opciones => {
                draw_text("Esta en desarrollo, perdoon", 200.0, 100.0, 40.0, WHITE);

                if boton("Volver al Menu", 200.0, 150.0, 200.0, 50.0) {
                    self.pantalla_actual = Pantalla::Menu;
                }
            }

            Pantalla::Añadir => {
            draw_text("Cuantos quieres", 200.0, 100.0, 40.0, WHITE);

            if boton("Añadir",200.0, 150.0, 200.0, 50.0) {
                self.pantalla_actual = Pantalla::Menu; 
            }
        }   

        Pantalla::Juego => {
            let ancho_pantalla = screen_width();
            let alto_pantalla = screen_height();

            let ancho_boton = 200.0;
            let alto_boton = 50.0;

           
            let titulo = "Mundo Valle de Aburra";
            let medida = measure_text(titulo, None, 40, 1.0);
            draw_text(
                titulo,
                (ancho_pantalla - medida.width) / 2.0,
                100.0,
                40.0,
                WHITE,
            );

            
            if boton("Salir al Menu", 20.0, 20.0, ancho_boton, alto_boton) {
                self.pantalla_actual = Pantalla::Menu;
            }

           
            let margen_inferior = 100.0;
            let espacio = 40.0;         

            
            let total_ancho = (ancho_boton * 2.0) + espacio;
            let x_inicio = (ancho_pantalla - total_ancho) / 2.0;
            let y_boton = alto_pantalla - margen_inferior;

           
            if boton("Añadir Presas", x_inicio, y_boton, ancho_boton, alto_boton) {
                self.pantalla_actual = Pantalla::Añadir;
            }

            
            if boton("Añadir Depredadores", x_inicio + ancho_boton + espacio, y_boton, 300.0, alto_boton) {
                self.pantalla_actual = Pantalla::Añadir;
            }
        }



        }
    }
}


fn boton(texto: &str, x: f32, y: f32, ancho: f32, alto: f32) -> bool {
    let mouse = mouse_position();
    let dentro = mouse.0 > x && mouse.0 < x + ancho && mouse.1 > y && mouse.1 < y + alto;

    let color = if dentro { GRAY } else { DARKGRAY };

    draw_rectangle(x, y, ancho, alto, color);
    draw_text(texto, x + 20.0, y + alto / 1.5, 30.0, WHITE);

    dentro && is_mouse_button_pressed(MouseButton::Left)
}
