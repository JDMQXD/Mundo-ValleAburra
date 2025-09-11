mod models;
mod world;
mod predator;

use std::{thread, time::Duration};
use crate::world::Mundo;

fn main() {
    // Parámetros configurables
    let duracion_dia_segundos: u64 = 60; // <- aquí controlas "un día = X segundos". Cámbialo a 10, 1, etc.
    let mut mundo = Mundo::new();

    println!("Simulación depredador-presa (consola). Día dura {} segundos.", duracion_dia_segundos);
    println!("Presiona Ctrl+C para detener.");

    // Bucla principal: avanza días indefinidamente
    loop {
        let reporte = mundo.paso_dia();
        println!("{}", reporte);

        // Espera para simular el paso de un día en tiempo real
        thread::sleep(Duration::from_secs(duracion_dia_segundos));
    }
}

