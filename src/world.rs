use crate::models::{Animal, Especie, crear_poblacion_inicial, EtapaDeVida};
use crate::predator::Depredador;
use rand::Rng;
use std::collections::HashMap;
use crate::models::ComportamientoAnimal;


pub struct Mundo {
    pub presas: Vec<Animal>,
    pub depredador: Depredador,
    pub dia_actual: u64,
    pub contador_id: u64,
    pub total_comidas: u64,
}

impl Mundo {
    pub fn new() -> Self {
        let mut contador_id = 1u64;
        // Poblaciones iniciales por especie (ajustables)
        let mut presas = Vec::new();
        presas.extend(crear_poblacion_inicial(30, Especie::Conejo, &mut contador_id));
        presas.extend(crear_poblacion_inicial(8, Especie::Cabra, &mut contador_id));
        presas.extend(crear_poblacion_inicial(3, Especie::Vaca, &mut contador_id));

        Self {
            presas,
            depredador: Depredador::new(5.0, 12.0), // parámetros iniciales
            dia_actual: 0,
            contador_id,
            total_comidas: 0,
        }
    }

    /// Ejecuta un día de simulación y retorna un reporte textual breve.
    pub fn paso_dia(&mut self) -> String {
        self.dia_actual += 1;
        let mut rng = rand::thread_rng();
        let mut reporte = String::new();

        // 1) Edad, peso, transición, muertes por vejez
        let mut vivos: Vec<Animal> = Vec::with_capacity(self.presas.len());
        for mut a in self.presas.drain(..) {
            a.edad_dias += 1;
            a.peso_kg = a.especie.calcular_peso(a.edad_dias);
            if a.edad_dias >= a.especie.edad_adulta() {
                a.etapa = EtapaDeVida::Adulto;
            }
            if a.edad_dias < a.especie.edad_maxima() {
                vivos.push(a);
            } else {
                reporte.push_str(&format!("Día {}: Animal ID {} de {:?} murió de vejez.\n", self.dia_actual, a.id, a.especie));
            }
        }
        self.presas = vivos;

        // 2) Reproducción: cada adulto tiene probabilidad diaria
        let mut nacimientos = Vec::new();
        for a in &self.presas {
            if a.etapa == EtapaDeVida::Adulto {
                let p = a.especie.tasa_reproduccion_diaria();
                if rng.gen_bool(p as f64) {
                    // genera entre 1 y max_crias_por_parto
                    let maxc = a.especie.max_crias_por_parto();
                    let n_crias = rng.gen_range(1..=maxc);
                    for _ in 0..n_crias {
                        let sexo = rng.gen_bool(0.5);
                        let nuevo = Animal {
                            id: self.contador_id,
                            especie: a.especie,
                            edad_dias: 0,
                            etapa: EtapaDeVida::Cria,
                            peso_kg: a.especie.calcular_peso(0),
                            sexo_macho: sexo,
                        };
                        self.contador_id += 1;
                        nacimientos.push(nuevo);
                    }
                }
            }
        }

        if !nacimientos.is_empty() {
            reporte.push_str(&format!("Día {}: Nacieron {} crías.\n", self.dia_actual, nacimientos.len()));
            let count_before = self.presas.len();
            self.presas.extend(nacimientos);
            reporte.push_str(&format!("Población pasó de {} a {} por nacimientos.\n", count_before, self.presas.len()));
        }

        // 3) Depredador: intenta cazar hasta tener su nivel óptimo o no encontrar presas elegibles.
        let mut comidas_hoy = 0u64;
        loop {
            // si ya tiene suficiente reserva para evitar enfermar, puede dejar de cazar
            if self.depredador.reserva_kg >= self.depredador.minimo_diario_kg {
                break;
            }
            // intenta cazar 1 presa (la más pesada elegible)
            match self.depredador.cazar_mas_pesada(&mut self.presas) {
                Some(presa) => {
                    comidas_hoy += 1;
                    self.total_comidas += 1;
                    reporte.push_str(&format!("Día {}: Depredador cazó ID {} ({:?}, {:.2} kg).\n", self.dia_actual, presa.id, presa.especie, presa.peso_kg));
                }
                None => {
                    // no hay presas elegibles
                    break;
                }
            }
            // evita loops infinitos: si presas vacías o ya alcanzó optimo, detener
            if self.presas.is_empty() || self.depredador.reserva_kg >= self.depredador.optimo_diario_kg {
                break;
            }
            // tope de caza por día (seguridad)
            if comidas_hoy > 50 { break; }
        }

        // 4) Consumo mínimo (si no alcanzó mínimo, el depredador "falla" y se marca en reporte)
        let sano = self.depredador.consumir_minimo();
        if sano {
            reporte.push_str(&format!("Día {}: Depredador se alimentó del mínimo requerido. Reserva ahora: {:.2} kg.\n", self.dia_actual, self.depredador.reserva_kg));
        } else {
            reporte.push_str(&format!("Día {}: Depredador NO alcanzó el mínimo y está en riesgo. Reserva: {:.2} kg.\n", self.dia_actual, self.depredador.reserva_kg));
        }
        // si sobra, intenta optimizar (opcional)
        self.depredador.intentar_optimizar_estado();

        // 5) Estadísticas por especie
        let mut mapa: HashMap<Especie, usize> = HashMap::new();
        for a in &self.presas {
            *mapa.entry(a.especie).or_insert(0) += 1;
        }

        reporte.push_str(&format!("\n=== Día {} - Resumen ===\n", self.dia_actual));
        reporte.push_str(&format!("Presas totales: {}\n", self.presas.len()));
        for sp in &[Especie::Conejo, Especie::Cabra, Especie::Vaca] {
            let c = *mapa.get(sp).unwrap_or(&0);
            reporte.push_str(&format!("- {:?}: {}\n", sp, c));
        }
        reporte.push_str(&format!("Total comidas por depredador hasta ahora: {}\n", self.total_comidas));
        reporte.push_str(&format!("Reserva del depredador: {:.2} kg\n", self.depredador.reserva_kg));

        // 6) Devolver reporte
        reporte
    }
}
