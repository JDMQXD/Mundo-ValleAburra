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

    // Funcion para agregar depredadores, limitado a UNO solo según requisitos
    pub fn agregar_depredadores(&mut self, cantidad: u32) {
        // Solo permitir UN depredador según los requisitos
        let cantidad_real = if cantidad > 0 { 1 } else { 0 };
        for i in 0..cantidad_real {
            self.depredadores.push(Depredador::new(i));
        }
    }

    // Funcion para agregar animales (presas), nacen como crías (edad 0) y crecen hasta ser adultas
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
                especie.edad_sacrificio(), // Las presas comienzan con edad de sacrificio para ser cazables
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
            // Si necesita cazar (reserva menor al óptimo)
            if dep.necesita_cazar() {
                // Buscar presas que pueden ser cazadas (han alcanzado edad de sacrificio)
                let mut presas_cazables: Vec<(usize, f32)> = self.presas
                    .iter()
                    .enumerate()
                    .filter(|(_, presa)| presa.puede_ser_cazado())
                    .map(|(idx, presa)| (idx, presa.peso_kg))
                    .collect();

                if !presas_cazables.is_empty() {
                    // Ordenar por peso (descendente) para obtener la más pesada
                    presas_cazables.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
                    
                    // Obtener todas las presas con el peso máximo (en caso de empate)
                    let peso_maximo = presas_cazables[0].1;
                    let presas_con_peso_maximo: Vec<usize> = presas_cazables
                        .iter()
                        .filter(|(_, peso)| (*peso - peso_maximo).abs() < 0.01) // Tolerancia para empates
                        .map(|(idx, _)| *idx)
                        .collect();

                    // En caso de empate, elegir una al azar
                    let pos_elegida = if presas_con_peso_maximo.len() == 1 {
                        presas_con_peso_maximo[0]
                    } else {
                        *presas_con_peso_maximo.choose(&mut rng).unwrap()
                    };

                    // Cazar la presa seleccionada
                    let presa = self.presas.remove(pos_elegida);
                    dep.cazar(&presa);
                    // reporta la presa que cazo, cuanto peso y que especie fue
                    reporte.push_str(&format!(
                        "Depredador #{} cazó presa ID {} ({:?}, {:.2} kg, {} días).\n",
                        dep.id, presa.id, presa.especie, presa.peso_kg, presa.edad_dias
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
                //Si el depredador esta vivo cada dia que pase va a consumir el nivel mínimo de la reserva
                if comio {
                    reporte.push_str(&format!(
                        "Depredador #{} consumió {:.1} kg. Reserva: {:.2} kg\n",
                        dep.id, dep.nivel_minimo_diario, dep.reserva_kg
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

        // Envejecer presas y verificar enfermedades
        let mut presas_enfermas = Vec::new();
        for (idx, presa) in self.presas.iter_mut().enumerate() {
            presa.envejecer_un_dia();
            // Verificar si se enferma (probabilidad diaria)
            if presa.verificar_enfermedad() {
                presas_enfermas.push(idx);
            }
        }

        // Remover presas que se enfermaron
        if !presas_enfermas.is_empty() {
            presas_enfermas.sort_unstable();
            presas_enfermas.reverse(); // Remover desde el final para mantener índices válidos
            for idx in presas_enfermas {
                let presa_enferma = self.presas.remove(idx);
                reporte.push_str(&format!(
                    "Presa ID {} ({:?}) murió por enfermedad.\n",
                    presa_enferma.id, presa_enferma.especie
                ));
            }
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
            let estado_alimentacion = if dep.reserva_kg >= dep.nivel_optimo_diario {
                "Óptimo"
            } else if dep.reserva_kg >= dep.nivel_minimo_diario {
                "Mínimo"
            } else {
                "Hambriento"
            };
            reporte.push_str(&format!(
                "- Depredador #{} | Edad: {} días | Reserva: {:.2} kg | Estado: {} | Días sin comer: {}\n",
                dep.id, dep.edad_dias, dep.reserva_kg, estado_alimentacion, dep.dias_sin_comer
            ));
        }

        reporte
    }
}
