pub mod geo {
    pub fn calculate_distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
        const R: f64 = 6371000.0;
        let phi1 = lat1.to_radians();
        let phi2 = lat2.to_radians();
        let dphi = (lat2 - lat1).to_radians();
        let dlambda = (lon2 - lon1).to_radians();
        let a = (dphi / 2.0).sin().powi(2) + phi1.cos() * phi2.cos() * (dlambda / 2.0).sin().powi(2);
        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
        R * c
    }
}

