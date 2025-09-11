use rand::Rng;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum Especie {
    Conejo,
    Cabra,
    Vaca,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Sexo {
    Macho,
    Hembra,
}

#[derive(Debug, Clone)]
pub struct Animal {
    pub id: u32,
    pub especie: Especie,
    pub edad_dias: u32,
    pub peso_kg: f32,
    pub sexo: Sexo,
}

#[derive(Debug)]
pub struct Depredador {
    pub id: u32,
    pub reserva_kg: f32, // comida almacenada
}

impl Depredador {
    pub fn new(id: u32) -> Self {
        Self { id, reserva_kg: 0.0 }
    }

    pub fn consumir_diario(&mut self) -> bool {
        if self.reserva_kg >= 1.0 {
            self.reserva_kg -= 1.0;
            true
        } else {
            false
        }
    }

    pub fn cazar(&mut self, presa: Animal) {
        self.reserva_kg += presa.peso_kg;
    }
}

pub trait ComportamientoAnimal {
    fn edad_adulta(&self) -> u32;
    fn edad_maxima(&self) -> u32;
    fn tasa_reproduccion_diaria(&self) -> f32;
    fn max_crias_por_parto(&self) -> u8;
    fn peso_inicial(&self) -> f32;
}

impl ComportamientoAnimal for Especie {
    fn edad_adulta(&self) -> u32 {
        match self {
            Especie::Conejo => 90,
            Especie::Cabra => 365,
            Especie::Vaca => 500,
        }
    }

    fn edad_maxima(&self) -> u32 {
        match self {
            Especie::Conejo => 2000,
            Especie::Cabra => 5000,
            Especie::Vaca => 6000,
        }
    }

    fn tasa_reproduccion_diaria(&self) -> f32 {
        match self {
            Especie::Conejo => 0.2,
            Especie::Cabra => 0.05,
            Especie::Vaca => 0.02,
        }
    }

    fn max_crias_por_parto(&self) -> u8 {
        match self {
            Especie::Conejo => 5,
            Especie::Cabra => 2,
            Especie::Vaca => 1,
        }
    }

    fn peso_inicial(&self) -> f32 {
        match self {
            Especie::Conejo => 2.0,
            Especie::Cabra => 40.0,
            Especie::Vaca => 200.0,
        }
    }
}
