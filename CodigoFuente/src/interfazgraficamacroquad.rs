use macroquad::prelude::*;
use crate::world::Mundo;
use crate::models::Especie;

enum Estado {
    Formulario {
        paso: usize,
        input: String,
        depredadores: u32,
        conejos: u32,
        cabras: u32,
        vacas: u32,
    },
    Simulacion {
        mundo: Mundo,
        reporte: Vec<String>,
        timer: f32,
    },
}

pub async fn run_app() {
    let mut estado = Estado::Formulario {
        paso: 0,
        input: String::new(),
        depredadores: 0,
        conejos: 0,
        cabras: 0,
        vacas: 0,
    };

    loop {
        clear_background(LIGHTGRAY);

        match &mut estado {
            Estado::Formulario {
                paso,
                input,
                depredadores,
                conejos,
                cabras,
                vacas,
            } => {
                let preguntas = [
                    "¿Cuántos depredadores quieres?",
                    "¿Cuántos conejos iniciales?",
                    "¿Cuántas cabras iniciales?",
                    "¿Cuántas vacas iniciales?",
                ];

                draw_text(&preguntas[*paso], 20.0, 50.0, 30.0, BLACK);
                draw_text(&format!("> {}", input), 20.0, 100.0, 30.0, DARKBLUE);

                if let Some(ch) = get_char_pressed() {
                    match ch {
                        '\r' | '\n' => {
                            let valor: u32 = input.trim().parse().unwrap_or(0);
                            match *paso {
                                0 => *depredadores = valor,
                                1 => *conejos = valor,
                                2 => *cabras = valor,
                                3 => {
                                    *vacas = valor;

                                    let mut mundo = Mundo::new();
                                    mundo.agregar_depredadores(*depredadores);
                                    mundo.agregar_presas(Especie::Conejo, *conejos);
                                    mundo.agregar_presas(Especie::Cabra, *cabras);
                                    mundo.agregar_presas(Especie::Vaca, *vacas);

                                    estado = Estado::Simulacion {
                                        mundo,
                                        reporte: Vec::new(),
                                        timer: 0.0,
                                    };
                                    continue;
                                }
                                _ => {}
                            }
                            *paso += 1;
                            input.clear();
                        }
                        '\u{8}' => {
                            input.pop();
                        }
                        _ if ch.is_ascii_digit() => {
                            input.push(ch);
                        }
                        _ => {}
                    }
                }
            }

            Estado::Simulacion { mundo, reporte, timer } => {
                *timer += get_frame_time();

                if *timer >= 5.0 {
                    *timer = 0.0;
                    let rep = mundo.paso_dia();
                    reporte.clear(); // solo guardamos lo más reciente
                    reporte.push(rep);
                }

                // === Cajita Día ===
                draw_box_with_text(20.0, 20.0, 300.0, 60.0,
                    &format!("Día actual: {}", mundo.dia_actual));

                // === Cajita Población ===
                draw_box_with_text(20.0, 100.0, 300.0, 100.0,
                    &format!("Depredadores: {}\nPresas: {}", 
                             mundo.depredadores.len(), 
                             mundo.presas.len()));

                // === Cajita por especies ===
                let conejos = mundo.presas.iter().filter(|a| a.especie == Especie::Conejo).count();
                let cabras = mundo.presas.iter().filter(|a| a.especie == Especie::Cabra).count();
                let vacas = mundo.presas.iter().filter(|a| a.especie == Especie::Vaca).count();

                draw_box_with_text(20.0, 220.0, 300.0, 120.0,
                    &format!("Conejos: {}\nCabras: {}\nVacas: {}", conejos, cabras, vacas));

                // === Cajita último reporte ===
                let mut texto_reporte = String::new();
                if let Some(ultimo) = reporte.last() {
                    texto_reporte = ultimo.clone();
                }
                draw_box_with_text(350.0, 20.0, 800.0, 420.0, &texto_reporte);
            }
        }

        next_frame().await;
    }
}

/// Función auxiliar: dibuja una caja con borde y texto
fn draw_box_with_text(x: f32, y: f32, w: f32, h: f32, text: &str) {
    draw_rectangle(x, y, w, h, WHITE);
    draw_rectangle_lines(x, y, w, h, 2.0, BLACK);

    let mut y_text = y + 25.0;
    for line in text.lines() {
        draw_text(line, x + 10.0, y_text, 20.0, BLACK);
        y_text += 25.0;
    }
}
