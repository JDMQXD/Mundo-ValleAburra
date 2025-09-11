use crate::models::{Animal, Especie};
use rand::Rng;
use crate::models::ComportamientoAnimal;


#[derive(Debug)]
pub struct Depredador {
    pub reserva_kg: f32,
    pub minimo_diario_kg: f32,
    pub optimo_diario_kg: f32,
    pub comidas_obtenidas_total: u64,
}

impl Depredador {
    pub fn new(minimo_diario_kg: f32, optimo_diario_kg: f32) -> Self {
        Self {
            reserva_kg: 0.0,
            minimo_diario_kg,
            optimo_diario_kg,
            comidas_obtenidas_total: 0,
        }
    }

    /// Busca entre `presas` (vec mutable) la presa elegible más pesada (edad >= edad_sacrificio).
    /// Si encuentra una, la elimina del vec y suma su peso a la reserva; devuelve Some(animal).
    pub fn cazar_mas_pesada(&mut self, presas: &mut Vec<Animal>) -> Option<Animal> {
        // identificar índices de presas elegibles
        let mut candidate_idx: Option<usize> = None;
        let mut max_peso = -1.0f32;
        for (i, a) in presas.iter().enumerate() {
            let edad_sac = a.especie.edad_sacrificio();
            if a.edad_dias >= edad_sac {
                if (a.peso_kg - max_peso).abs() < f32::EPSILON {
                    // empate: seleccionamos aleatoriamente entre mantener o cambiar
                    let mut rng = rand::thread_rng();
                    if rng.gen_bool(0.5) {
                        candidate_idx = Some(i);
                    }
                } else if a.peso_kg > max_peso {
                    max_peso = a.peso_kg;
                    candidate_idx = Some(i);
                }
            }
        }

        if let Some(idx) = candidate_idx {
            let presa = presas.swap_remove(idx); // la eliminamos
            self.reserva_kg += presa.peso_kg;
            self.comidas_obtenidas_total += 1;
            Some(presa)
        } else {
            None
        }
    }

    /// Consume de la reserva (para "dormir" sin enfermar). Retorna si alcanzó el mínimo.
    pub fn consumir_minimo(&mut self) -> bool {
        if self.reserva_kg >= self.minimo_diario_kg {
            self.reserva_kg -= self.minimo_diario_kg;
            true
        } else {
            // no hay suficiente reserva -> falla en alimentarse
            self.reserva_kg = 0.0;
            false
        }
    }

    /// Si sobra, intenta consumir hasta nivel óptimo (no obligatorio)
    pub fn intentar_optimizar_estado(&mut self) {
        if self.reserva_kg >= self.optimo_diario_kg {
            self.reserva_kg -= self.optimo_diario_kg;
        }
    }
}
