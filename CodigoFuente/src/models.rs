use rand::Rng;

/// Aqui definimos las especies de presas que tendremos
#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum Especie {
    Conejo,
    Cabra,
    Vaca,
}
// Si son machos o hembras
#[derive(Debug, Clone, PartialEq)]
pub enum Sexo {
    Macho,
    Hembra,
}

// Crecimiento gomperzt
pub type GrowthFunction = Box<dyn Fn(u32) -> f32 + Send + Sync>;

// funcion que usamos con gomperzt para ver el peso segun la edad del animlal
pub fn create_growth_function(species: Especie) -> GrowthFunction {
    let (a, b, k) = match species {
        // Peso, velocidad de creciemiento, curva de tiempo
        Especie::Conejo => (5.0, 2.5, 0.05),
        Especie::Cabra => (75.0, 2.8, 0.01),
        Especie::Vaca => (700.0, 3.0, 0.008),
    };

    Box::new(move |age_days| {
        let t = age_days as f32;
        a * (-b * (-k * t).exp()).exp()
    })
}

/// Animal (presa)
pub struct Animal {
    pub id: u32,
    pub especie: Especie,
    pub edad_dias: u32,
    pub peso_kg: f32,
    pub sexo: Sexo,
    pub growth_fn: GrowthFunction,
}

impl Animal {
    pub fn new(id: u32, especie: Especie, edad_inicial: u32, sexo: Sexo) -> Self {
        let growth_fn = create_growth_function(especie.clone());
        let peso_kg = (growth_fn)(edad_inicial);
        Self {
            id,
            especie,
            edad_dias: edad_inicial,
            peso_kg,
            sexo,
            growth_fn,
        }
    }

    // Funcion que permite que cada dia que pase se sume uno
    // y el peso se basa de la edad con la grompertz
    pub fn envejecer_un_dia(&mut self) {
        self.edad_dias += 1;
        self.peso_kg = (self.growth_fn)(self.edad_dias);
    }
}

// Lo clonamos manualmente para evitar problemas con el Box<dyn Fn(u32) porque no daba
// problemas proque no nos dejaba clonarlo automaticamente
impl Clone for Animal {
    fn clone(&self) -> Self {
        let growth_fn = create_growth_function(self.especie.clone());
        Self {
            id: self.id,
            especie: self.especie.clone(),
            edad_dias: self.edad_dias,
            peso_kg: self.peso_kg,
            sexo: self.sexo.clone(),
            growth_fn,
        }
    }
}

/// Depredador 
#[derive(Debug, Clone)]
pub struct Depredador {
    pub id: u32,
    pub edad_dias: u32,
    pub edad_maxima: u32,
    pub reserva_kg: f32,
    pub dias_sin_comer: u32,
}

impl Depredador {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            edad_dias: 0,
            edad_maxima: 4000,
            reserva_kg: 0.0,
            dias_sin_comer: 0,
        }
    }

    // funcion que permite sumar un dia a los animales
    pub fn envejecer_un_dia(&mut self) {
        self.edad_dias += 1;
    }

    // Funcion que nos dice cuando el animal comio algo mayor a 1 kilo que nos mande true
    // Y si no come nada en el dia se le suma un dia sin comer
    pub fn consumir_diario(&mut self) -> bool {
        if self.reserva_kg >= 1.0 {
            self.reserva_kg -= 1.0;
            self.dias_sin_comer = 0;
            true
        } else {
            self.dias_sin_comer += 1;
            false
        }
    }

    // Cuando el depredador caza se le suma el peso del animal a la reserva del depredador
    pub fn cazar(&mut self, presa: &Animal) {
        self.reserva_kg += presa.peso_kg;
    }

    // Funcion que nos dice que si cumple todas las condiciones como dias sin comer menor a 5 y las demas seguira vivo
    pub fn esta_vivo(&self) -> bool {
        self.dias_sin_comer < 5 && self.edad_dias < self.edad_maxima
    }
}

/// Parametros de las especies
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
