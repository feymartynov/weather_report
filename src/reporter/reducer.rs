use super::Report;

pub struct Reducer {
    reports: Vec<Vec<Report>>,
}

impl Reducer {
    pub fn new(days: usize) -> Self {
        Self {
            reports: vec![vec![]; days],
        }
    }

    /// Gets a vector of reports for the number of days by single provider and spreads them
    /// by days in the inner state for future reduction.
    pub fn push_provider_reports(&mut self, provider_reports: &Vec<Report>) {
        for (report, day_reports) in provider_reports.iter().zip(self.reports.iter_mut()) {
            day_reports.push(report.clone());
        }
    }

    /// Gets reports for each day for each provider and returns averaged reports for each day.
    pub fn reduce_reports(&self) -> Vec<Report> {
        self.reports
            .iter()
            .map(|day_reports| {
                let temperature_sum = day_reports.iter().map(|r| r.temperature).sum::<f32>();
                let mean_temperature = temperature_sum / self.reports.len() as f32;
                Report::new(mean_temperature)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::super::Report;
    use super::*;

    #[test]
    fn reduce_reports() {
        let provider1_reports = vec![Report::new(1.0), Report::new(2.0)];
        let provider2_reports = vec![Report::new(3.0), Report::new(4.0)];

        let mut reducer = Reducer::new(2);
        reducer.push_provider_reports(&provider1_reports);
        reducer.push_provider_reports(&provider2_reports);
        let reports = reducer.reduce_reports();

        assert_eq!(reports, vec![Report::new(2.0), Report::new(3.0)]);
    }
}
