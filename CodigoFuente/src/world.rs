use rand::seq::SliceRandom;
use rand::prelude::IteratorRandom;
use rand::Rng;
use crate::models::{Animal, ComportamientoAnimal, Depredador, Especie, Sexo};
use std::collections::HashMap;

// Etructura del mundo
pub struct Mundo {
    pub dia_actual: u32,
    pub depredadores: Vec<Depredador>,
    pub presas: Vec<Animal>,
    pub contador_animales: u32,
}

// Definimos el mundo que empiece totalmente vacio
impl Mundo {
    pub fn new() -> Self {
        Self {
            dia_actual: 0,
            depredadores: Vec::new(),
            presas: Vec::new(),
            contador_animales: 0,
        }
    }

    // Funcion para agregar depredadores, con ayuda del FOR se le asignan los ID para que no se repitan
    pub fn agregar_depredadores(&mut self, cantidad: u32) {
        for i in 0..cantidad {
            self.depredadores.push(Depredador::new(i));
        }
    }

    // Funcion para agregar animales (presas), con la misma forma de asignar ID como los depredadores
    pub fn agregar_presas(&mut self, especie: Especie, cantidad: u32) {
        let mut rng = rand::thread_rng();
        for _ in 0..cantidad {
            // Por cada presa nueva se suma 1 al contador
            self.contador_animales += 1;
            // Con ayuda del anterior rand se le asigna a la presa si va a ser macho o hembra
            let sexo = if rng.gen_bool(0.5) { Sexo::Macho } else { Sexo::Hembra };
            self.presas.push(Animal::new(
                self.contador_animales,
                especie.clone(),
                especie.edad_adulta(), // Las presas nacen ya adultos
                sexo,
            ));
        }
    }

    //Funcion que nos dara todos los detalles de los pasos de los dias
    pub fn paso_dia(&mut self) -> String {
        self.dia_actual += 1;
        let mut rng = rand::thread_rng();
        let mut reporte = format!("=== Día {} ===\n", self.dia_actual);

        // Envejecer depredadores
        for dep in &mut self.depredadores {
            dep.envejecer_un_dia();
        }

        // Mezclar orden de depredadores, esto se hizo para que cualquier depredador caze no siempre el depredador #0 es el que empiece
        let mut orden = (0..self.depredadores.len()).collect::<Vec<_>>();
        orden.shuffle(&mut rng);

        // Cada depredador intenta cazar UNA presa si lo necesita
        for &i in &orden {
            let dep = &mut self.depredadores[i];
            // Si la reserva es menor a 1 (osea que tiene hambre) empieza a cazar
            if dep.reserva_kg < 1.0 {
                // Elije la presa aleatoriamente.
                // Primero ve cuantas presas hay (o si no hay) de 0 al tamano de la lista de las presas
                // Segundo, aleatoriamente escoje una presa para cazar
                if let Some(pos) = (0..self.presas.len()).choose(&mut rng) {
                    // Cuando caze la presa removera la presa de la lista
                    let presa = self.presas.remove(pos);
                    dep.cazar(&presa);
                    // reporta la presa que cazo, cuanto peso y que especie fue la pobre
                    reporte.push_str(&format!(
                        "Depredador #{} cazó presa ID {} ({:?}, {:.2} kg).\n",
                        dep.id, presa.id, presa.especie, presa.peso_kg
                    ));
                }
            }
        }

        // Consumo diario y filtrado de depredadores vivos
        let mut vivos = Vec::new();
        for dep in &mut self.depredadores {
            let comio = dep.consumir_diario();
            // Si el depredador sigue vivo
            // Osea que cumple los requisitos de la funcion en models.rs
            if dep.esta_vivo() {
                //Si el depredador esta vivo cada dia que pase va a consumir 1 kilo de la reserva
                if comio {
                    reporte.push_str(&format!(
                        "Depredador #{} consumió 1 kg. Reserva: {:.2} kg\n",
                        dep.id, dep.reserva_kg
                    ));
                } else {
                    reporte.push_str(&format!(
                        "Depredador #{} no comió (lleva {} días sin comer). Reserva: {:.2} kg\n",
                        dep.id, dep.dias_sin_comer, dep.reserva_kg
                    ));
                }
                vivos.push(dep.clone());
            } else {
                // Si queda fuera de la lisra de esta vivo, es porque se murio
                reporte.push_str(&format!(
                    "Depredador #{} murió (Edad: {} días, Días sin comer: {}).\n",
                    dep.id, dep.edad_dias, dep.dias_sin_comer
                ));
            }
        }
        self.depredadores = vivos;

        // Envejecer presas
        for presa in &mut self.presas {
            presa.envejecer_un_dia();
        }

        // Filtrar presas vivas (muerte por vejez), si no lo estan los elimina de la lista
        self.presas.retain(|p| p.edad_dias < p.especie.edad_maxima());

        // Reproducción
        let mut nuevas_presas = Vec::new();
        for presa in &self.presas {
            //Si la presa es mayor y es hembra puede crear nuevos presas
            if presa.edad_dias >= presa.especie.edad_adulta() && presa.sexo == Sexo::Hembra {
                if rng.gen_bool(presa.especie.tasa_reproduccion_diaria() as f64) {
                    let cantidad = rng.gen_range(1..=presa.especie.max_crias_por_parto());
                    for _ in 0..cantidad {
                        self.contador_animales += 1;
                        let sexo = if rng.gen_bool(0.5) { Sexo::Macho } else { Sexo::Hembra };
                        nuevas_presas.push(Animal::new(
                            self.contador_animales,
                            presa.especie.clone(),
                            0,
                            sexo,
                        ));
                    }
                }
            }
        }
        if !nuevas_presas.is_empty() {
            reporte.push_str(&format!("Nacieron {} crías.\n", nuevas_presas.len()));
        }
        self.presas.extend(nuevas_presas);

        // Resumen de presas
        let mut mapa: HashMap<&Especie, usize> = HashMap::new();
        for presa in &self.presas {
            *mapa.entry(&presa.especie).or_insert(0) += 1;
        }

        reporte.push_str(&format!(
            "=== Resumen Día {} ===\nTotal presas: {}\n",
            self.dia_actual,
            self.presas.len()
        ));
        for (especie, count) in mapa {
            reporte.push_str(&format!("- {:?}: {}\n", especie, count));
        }

        // Resumen de depredadores
        reporte.push_str(&format!("Depredadores vivos: {}\n", self.depredadores.len()));
        for dep in &self.depredadores {
            reporte.push_str(&format!(
                "- Depredador #{} | Edad: {} días | Reserva: {:.2} kg | Días sin comer: {}\n",
                dep.id, dep.edad_dias, dep.reserva_kg, dep.dias_sin_comer
            ));
        }

        reporte
    }
}
