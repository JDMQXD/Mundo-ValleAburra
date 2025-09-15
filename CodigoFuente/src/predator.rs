/* 
use crate::models::Animal;

#[derive(Debug)]
pub struct DepredadorAvanzado {
    pub id: u64,
    pub reserva_kg: f32,
    pub minimo_diario_kg: f32,
    pub optimo_diario_kg: f32,
    pub comidas_obtenidas_total: u64,
}

impl DepredadorAvanzado {
    pub fn new(id: u64, minimo_diario_kg: f32, optimo_diario_kg: f32) -> Self {
        Self {
            id,
            reserva_kg: 0.0,
            minimo_diario_kg,
            optimo_diario_kg,
            comidas_obtenidas_total: 0,
        }
    }

    pub fn cazar(&mut self, presa: Animal) {
        self.reserva_kg += presa.peso_kg;
        self.comidas_obtenidas_total += 1;
    }

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
