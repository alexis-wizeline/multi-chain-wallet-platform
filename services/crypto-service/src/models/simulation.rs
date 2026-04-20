#[derive(Debug)]
pub struct SimulationOutcome {
    pub logs: Vec<String>,
    pub units_onsumed: Option<u64>,
}
