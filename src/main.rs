mod models;
mod world;

use std::{thread, time::Duration};
use std::io;
use crate::models::Especie;
use crate::world::Mundo;

fn main() {
    let mut mundo = Mundo::new();
    let mut input = String::new();

    println!("¿Cuántos depredadores quieres?");
    io::stdin().read_line(&mut input).unwrap();
    let depredadores: u32 = input.trim().parse().unwrap_or(5);
    mundo.agregar_depredadores(depredadores);
    input.clear();

    println!("¿Cuántos conejos iniciales?");
    io::stdin().read_line(&mut input).unwrap();
    let conejos: u32 = input.trim().parse().unwrap_or(20);
    mundo.agregar_presas(Especie::Conejo, conejos);
    input.clear();

    println!("¿Cuántas cabras iniciales?");
    io::stdin().read_line(&mut input).unwrap();
    let cabras: u32 = input.trim().parse().unwrap_or(5);
    mundo.agregar_presas(Especie::Cabra, cabras);
    input.clear();

    println!("¿Cuántas vacas iniciales?");
    io::stdin().read_line(&mut input).unwrap();
    let vacas: u32 = input.trim().parse().unwrap_or(3);
    mundo.agregar_presas(Especie::Vaca, vacas);
    input.clear();

    println!("Simulación depredador-presa iniciada.");

    for _ in 0..30 {
        let reporte = mundo.paso_dia();
        println!("{}", reporte);
        thread::sleep(Duration::from_secs(5)); // cada "día" dura 5 segundos
    }
}
