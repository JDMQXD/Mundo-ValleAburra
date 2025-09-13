use crate::models::{Animal, ComportamientoAnimal};
use rand::Rng;

#[derive(Debug)]
pub struct Depredador {
    pub id: u64,
    pub reserva_kg: f32,
    pub minimo_diario_kg: f32,
    pub optimo_diario_kg: f32,
    pub comidas_obtenidas_total: u64,
}

impl Depredador {
    pub fn new(id: u64, minimo_diario_kg: f32, optimo_diario_kg: f32) -> Self {
        Self {
            id,
            reserva_kg: 0.0,
            minimo_diario_kg,
            optimo_diario_kg,
            comidas_obtenidas_total: 0,
        }
    }

    /// Buscar presa más pesada (edad >= sacrificio)
    pub fn cazar_mas_pesada(&mut self, presas: &mut Vec<Animal>) -> Option<Animal> {
        let mut candidate_idx: Option<usize> = None;
        let mut max_peso = -1.0f32;
        for (i, a) in presas.iter().enumerate() {
            let edad_sac = a.especie.edad_sacrificio();
            if a.edad_dias >= edad_sac {
                if (a.peso_kg - max_peso).abs() < f32::EPSILON {
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
            let presa = presas.swap_remove(idx);
            self.reserva_kg += presa.peso_kg;
            self.comidas_obtenidas_total += 1;
            Some(presa)
        } else {
            None
        }
    }

    /// Consumo basal al empezar el día
    pub fn gasto_diario(&mut self) -> bool {
        if self.reserva_kg >= self.minimo_diario_kg {
            self.reserva_kg -= self.minimo_diario_kg;
            true
        } else {
            self.reserva_kg = 0.0;
            false
        }
    }
}


