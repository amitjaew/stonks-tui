#[derive(Debug, Default)]
pub struct AppState {
    pub should_quit: bool,
    pub chart: ChartData
}


#[derive(Debug, Default)]
pub struct ChartData {
    pub name: String,
    pub points: Vec<(f64, f64)>
}


impl AppState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tick(&self) {}

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn update_chart(&mut self, new_chart:ChartData){
        self.chart = new_chart;
    }
}
