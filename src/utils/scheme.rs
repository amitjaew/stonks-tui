use serde::Deserialize;
use chrono::{ DateTime, Utc, Datelike };
use std::cmp::min;


#[derive(Deserialize, Debug, Clone)]
pub struct ClpUsdItem {
    fecha: DateTime<Utc>,
    valor: f64
}


#[derive(Deserialize, Debug)]
pub struct ClpUsdHistory {
    codigo: String,
    nombre: String,
    unidad_medida: String,
    serie: Vec<ClpUsdItem>
}


#[derive(Deserialize, Debug)]
pub struct CriptoUsdHistory {
    prices: Vec<[f64; 2]>,
    market_caps: Vec<[f64; 2]>,
    total_volumes: Vec<[f64; 2]>,
}


#[derive(Deserialize, Debug)]
pub struct ChileanIndicators {
    pub fecha: DateTime<Utc>,
    pub uf: ChileanIndicator,
    pub ivp: ChileanIndicator,
    pub dolar: ChileanIndicator,
    pub dolar_intercambio: ChileanIndicator,
    pub euro: ChileanIndicator,
    pub ipc: ChileanIndicator,
    pub utm: ChileanIndicator,
    pub imacec: ChileanIndicator,
    pub tpm: ChileanIndicator,
    pub libra_cobre: ChileanIndicator,
    pub tasa_desempleo: ChileanIndicator,
    pub bitcoin: ChileanIndicator,
}


#[derive(Deserialize, Debug)]
pub struct ChileanIndicator {
    pub codigo: String,
    pub nombre: String,
    pub unidad_medida: String,
    pub fecha: DateTime<Utc>,
    pub valor: f64,
}


#[derive(Deserialize, Debug)]
pub struct CriptoIndicator {
    id: String,
    symbol: String,
    name: String,
    current_price: f64,
    market_cap: f64,
    market_cap_rank: f64,
    total_volume: f64,
    high_24h: f64,
    low_24h: f64,
    price_change_24h: f64,
    price_change_percentage_24h: f64,
    last_updated: DateTime<Utc>,
}


impl ClpUsdItem {
    fn to_chart_point(
        &self,
    ) -> (f64, f64) {
        (self.fecha.timestamp() as f64, self.valor)
    }
}


impl CriptoUsdHistory {
    pub fn get_points(&self) -> Vec<(f64, f64)> {
        let mut temp = self.prices.to_vec();
        temp.sort_by_key(|item| item[0] as u64);
        temp
            .iter()
            .map(|item| (
                    *item.get(0).unwrap(),
                    *item.get(1).unwrap()
                )
            )
            .collect()
    }
    pub fn get_date_labels(&self) -> Vec<String> {
        let mut dates: Vec<String> = Vec::new();
        let points = self.get_points();
        let max_iter = min(points.len(), 4);
        for i in 0..max_iter {
            let index = min(
                ((points.len() as f32/ 3.0) * i as f32) as usize,
                (points.len() - 1) as usize
            ) as usize;
            let date = DateTime::from_timestamp_millis(
                points[index].0.round() as i64
            ).unwrap();
            let date_string = String::from(
                format!( "{}-{}", date.day(), date.month() )
            );
            dates.push(date_string);
        }
        dates
    }
}


impl ClpUsdHistory {
    pub fn get_points(&self) -> Vec<(f64, f64)> {
        let mut temp = self.serie.to_vec();
        temp.sort_by_key(|item| item.fecha);
        temp.iter()
            .map(|item| item.to_chart_point())
            .collect()
    }

    pub fn get_date_labels(&self) -> Vec<String> {
        let mut dates: Vec<String> = Vec::new();
        let points = self.get_points();
        let max_iter = min(points.len(), 4);
        for i in 0..max_iter {
            let index = min(
                ((points.len() as f32/ 3.0) * (i+1) as f32) as usize,
                (points.len() - 1) as usize
            ) as usize;
            let date = DateTime::from_timestamp(
                points[index].0.round() as i64, 0
            ).unwrap();
            let date_string = String::from(
                format!( "{}-{}", date.day(), date.month() )
            );
            dates.push(date_string);
        }
        dates
    }
}
