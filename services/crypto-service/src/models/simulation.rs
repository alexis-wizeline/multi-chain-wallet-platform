#[derive(Debug)]
pub struct SimulationOutcome {
    pub logs: Vec<String>,
    pub units_consumed: Option<u64>,
}
