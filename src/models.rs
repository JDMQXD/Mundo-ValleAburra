use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Especie {
    Conejo,
    Cabra,
    Vaca,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EtapaDeVida {
    Cria,
    Adulto,
}

#[derive(Debug, Clone)]
pub struct Animal {
    pub id: u64,
    pub especie: Especie,
    pub edad_dias: u32,
    pub etapa: EtapaDeVida,
    pub peso_kg: f32,
    pub sexo_macho: bool,
}

pub trait ComportamientoAnimal {
    fn espacio_requerido(&self) -> u32;
    fn comida_requerida_por_dia(&self) -> u32;
    fn edad_adulta(&self) -> u32; // en días
    fn edad_maxima(&self) -> u32; // en días
    fn edad_sacrificio(&self) -> u32; // edad mínima para ser cazada
    fn tasa_reproduccion_diaria(&self) -> f32; // probabilidad por día
    fn max_crias_por_parto(&self) -> u8;
    fn calcular_peso(&self, edad_dias: u32) -> f32;
}

impl ComportamientoAnimal for Especie {
    fn espacio_requerido(&self) -> u32 {
        match self {
            Especie::Conejo => 1,
            Especie::Cabra => 5,
            Especie::Vaca => 20,
        }
    }

    fn comida_requerida_por_dia(&self) -> u32 {
        match self {
            Especie::Conejo => 1,
            Especie::Cabra => 2,
            Especie::Vaca => 10,
        }
    }

    fn edad_adulta(&self) -> u32 {
        match self {
            Especie::Conejo => 180,    // 6 meses
            Especie::Cabra => 365,     // 1 año
            Especie::Vaca => 365 * 2,  // 2 años
        }
    }

    fn edad_maxima(&self) -> u32 {
        match self {
            Especie::Conejo => 365 * 8,
            Especie::Cabra => 365 * 15,
            Especie::Vaca => 365 * 25,
        }
    }

    fn edad_sacrificio(&self) -> u32 {
        // edad mínima para que depredador considere cazarla
        // la dejamos en la edad adulta por simplicidad
        self.edad_adulta()
    }

    fn tasa_reproduccion_diaria(&self) -> f32 {
        match self {
            Especie::Conejo => 0.12, // alta tasa
            Especie::Cabra => 0.02,
            Especie::Vaca => 0.005,
        }
    }

    fn max_crias_por_parto(&self) -> u8 {
        match self {
            Especie::Conejo => 6,
            Especie::Cabra => 2,
            Especie::Vaca => 1,
        }
    }

    fn calcular_peso(&self, edad_dias: u32) -> f32 {
        // curva de Gompertz aproximada (parámetros por especie)
        let (a, k, ti) = match self {
            Especie::Conejo => (2.5f32, 0.03f32, 90.0f32),
            Especie::Cabra  => (50.0f32, 0.01f32, 180.0f32),
            Especie::Vaca   => (600.0f32, 0.005f32, 365.0f32),
        };
        let t = edad_dias as f32;
        let ex_in = -k * (t - ti);
        let ex_out = -f32::exp(ex_in);
        let peso = a * f32::exp(ex_out);
        peso
    }
}

/// Crea una población inicial de `cantidad` animales de `especie`.
pub fn crear_poblacion_inicial(cantidad: u32, especie: Especie, start_id: &mut u64) -> Vec<Animal> {
    let mut pobl = Vec::with_capacity(cantidad as usize);
    let mut rng = rand::thread_rng();
    for _ in 0..cantidad {
        let sexo_macho = rng.gen_bool(0.5);
        let edad_dias = rng.gen_range(0..(especie.edad_adulta() + 30)); // edades variadas alrededor de adulto
        let peso = especie.calcular_peso(edad_dias);
        pobl.push(Animal {
            id: *start_id,
            especie,
            edad_dias,
            etapa: if edad_dias >= especie.edad_adulta() { EtapaDeVida::Adulto } else { EtapaDeVida::Cria },
            peso_kg: peso,
            sexo_macho,
        });
        *start_id += 1;
    }
    pobl
}
