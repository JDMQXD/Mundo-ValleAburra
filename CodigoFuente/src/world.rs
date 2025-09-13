use rand::seq::SliceRandom;
use rand::prelude::IteratorRandom;
use rand::Rng;
use crate::models::{Animal, ComportamientoAnimal, Depredador, Especie, Sexo};

pub struct Mundo {
    pub dia_actual: u32,
    pub depredadores: Vec<Depredador>,
    pub presas: Vec<Animal>,
    pub contador_animales: u32,
}

impl Mundo {
    pub fn new() -> Self {
        Self {
            dia_actual: 0,
            depredadores: Vec::new(),
            presas: Vec::new(),
            contador_animales: 0,
        }
    }

    pub fn agregar_depredadores(&mut self, cantidad: u32) {
        for i in 0..cantidad {
            self.depredadores.push(Depredador::new(i));
        }
    }

    pub fn agregar_presas(&mut self, especie: Especie, cantidad: u32) {
        let mut rng = rand::thread_rng();
        for _ in 0..cantidad {
            self.contador_animales += 1;
            self.presas.push(Animal {
                id: self.contador_animales,
                especie: especie.clone(),
                edad_dias: especie.edad_adulta(), // nacen adultos
                peso_kg: especie.peso_inicial(),
                sexo: if rng.r#gen::<bool>() { Sexo::Macho } else { Sexo::Hembra },
            });
        }
    }

    pub fn paso_dia(&mut self) -> String {
        self.dia_actual += 1;
        let mut rng = rand::thread_rng();
        let mut reporte = format!("=== Día {} ===\n", self.dia_actual);

        // Mezclar orden de depredadores
        let mut orden = (0..self.depredadores.len()).collect::<Vec<_>>();
        orden.shuffle(&mut rng);

        // Cada depredador intenta cazar UNA presa si lo necesita
        for &i in &orden {
            let dep = &mut self.depredadores[i];
            if dep.reserva_kg < 1.0 {
                if let Some(pos) = (0..self.presas.len()).choose(&mut rng) {
                    let presa = self.presas.remove(pos);
                    dep.cazar(presa.clone());
                    reporte.push_str(&format!(
                        "Día {}: Depredador #{} cazó ID {} ({:?}, {:.2} kg). Reserva: {:.2} kg\n",
                        self.dia_actual, dep.id, presa.id, presa.especie, presa.peso_kg, dep.reserva_kg
                    ));
                }
            }
        }

        // Consumo diario
        for dep in &mut self.depredadores {
            if dep.consumir_diario() {
                reporte.push_str(&format!(
                    "Día {}: Depredador #{} consumió 1 kg. Reserva: {:.2} kg\n",
                    self.dia_actual, dep.id, dep.reserva_kg
                ));
            } else {
                reporte.push_str(&format!(
                    "Día {}: Depredador #{} no tenía suficiente reserva y está en riesgo.\n",
                    self.dia_actual, dep.id
                ));
            }
        }

        // Reproducción
        let mut nuevas_presas = Vec::new();
        for presa in &self.presas {
            if presa.edad_dias >= presa.especie.edad_adulta() {
                if rng.r#gen::<f32>() < presa.especie.tasa_reproduccion_diaria() {
                    let cantidad = rng.gen_range(1..=presa.especie.max_crias_por_parto());
                    for _ in 0..cantidad {
                        self.contador_animales += 1;
                        nuevas_presas.push(Animal {
                            id: self.contador_animales,
                            especie: presa.especie.clone(),
                            edad_dias: 0,
                            peso_kg: presa.especie.peso_inicial(),
                            sexo: if rng.r#gen::<bool>() { Sexo::Macho } else { Sexo::Hembra },
                        });
                    }
                }
            }
        }
        if !nuevas_presas.is_empty() {
            reporte.push_str(&format!(
                "Día {}: Nacieron {} crías.\n",
                self.dia_actual,
                nuevas_presas.len()
            ));
        }
        self.presas.extend(nuevas_presas);

        reporte
    }
}
