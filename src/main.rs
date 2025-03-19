pub mod time_series {
    /// Estrutura para o modelo de regressão linear
    pub struct LinearRegression {
        pub slope: f64,      // Tornando público
        pub intercept: f64,  // Tornando público
    }

    impl LinearRegression {
        /// Ajusta o modelo de regressão linear aos dados
        pub fn fit(x: &[f64], y: &[f64]) -> Result<Self, &'static str> {
            if x.len() != y.len() || x.is_empty() {
                return Err("Os vetores x e y devem ter o mesmo tamanho e não podem estar vazios");
            }

            let n = x.len() as f64;
            let mut sum_x = 0.0;
            let mut sum_y = 0.0;
            let mut sum_xy = 0.0;
            let mut sum_xx = 0.0;

            for i in 0..x.len() {
                sum_x += x[i];
                sum_y += y[i];
                sum_xy += x[i] * y[i];
                sum_xx += x[i] * x[i];
            }

            let slope = (n * sum_xy - sum_x * sum_y) / (n * sum_xx - sum_x * sum_x);
            let intercept = (sum_y - slope * sum_x) / n;

            Ok(LinearRegression { slope, intercept })
        }

        /// Realiza previsões
        pub fn predict(&self, x: f64) -> f64 {
            self.intercept + self.slope * x
        }

        /// Calcula o R²
        pub fn r_squared(&self, x: &[f64], y: &[f64]) -> f64 {
            let n = x.len() as f64;
            let mut sum_y = 0.0;
            let mut ss_tot = 0.0;
            let mut ss_res = 0.0;

            for &val in y {
                sum_y += val;
            }
            let y_mean = sum_y / n;

            for i in 0..x.len() {
                let y_pred = self.predict(x[i]);
                ss_tot += (y[i] - y_mean) * (y[i] - y_mean);
                ss_res += (y[i] - y_pred) * (y[i] - y_pred);
            }

            if ss_tot == 0.0 { 1.0 } else { 1.0 - (ss_res / ss_tot) }
        }

        /// Calcula o MSE
        pub fn mean_squared_error(&self, x: &[f64], y: &[f64]) -> f64 {
            let n = x.len() as f64;
            let mut sum_sq_error = 0.0;

            for i in 0..x.len() {
                let y_pred = self.predict(x[i]);
                sum_sq_error += (y[i] - y_pred) * (y[i] - y_pred);
            }

            sum_sq_error / n
        }
    }
}

fn main() {
    let x = vec![1.0, 2.0, 4.0, 7.0, 5.0];
    let y = vec![3.1, 4.0, 5.2, 9.1, 10.0];

    match time_series::LinearRegression::fit(&x, &y) {
        Ok(model) => {
            println!("Inclinação: {:.4}", model.slope);
            println!("Intercepto: {:.4}", model.intercept);
            println!("Previsão para x=6: {:.4}", model.predict(6.0));
            println!("R²: {:.4}", model.r_squared(&x, &y));
            println!("MSE: {:.4}", model.mean_squared_error(&x, &y));
        }
        Err(e) => println!("Erro: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::time_series::LinearRegression;

    #[test]
    fn test_fit() {
        let x = vec![1.0, 2.0, 3.0];
        let y = vec![2.0, 4.0, 6.0];
        let model = LinearRegression::fit(&x, &y).unwrap();
        assert!((model.slope - 2.0).abs() < 1e-10, "Inclinação esperada: 2.0");
        assert!((model.intercept - 0.0).abs() < 1e-10, "Intercepto esperado: 0.0");
    }

    #[test]
    fn test_predict() {
        let x = vec![1.0, 2.0, 3.0];
        let y = vec![2.0, 4.0, 6.0];
        let model = LinearRegression::fit(&x, &y).unwrap();
        let pred = model.predict(4.0);
        assert!((pred - 8.0).abs() < 1e-10, "Previsão esperada: 8.0");
    }

    #[test]
    fn test_r_squared() {
        let x = vec![1.0, 2.0, 3.0];
        let y = vec![2.0, 4.0, 6.0];
        let model = LinearRegression::fit(&x, &y).unwrap();
        let r2 = model.r_squared(&x, &y);
        assert!((r2 - 1.0).abs() < 1e-10, "R² esperado: 1.0");
    }

    #[test]
    fn test_mse() {
        let x = vec![1.0, 2.0, 3.0];
        let y = vec![2.0, 4.0, 6.0];
        let model = LinearRegression::fit(&x, &y).unwrap();
        let mse = model.mean_squared_error(&x, &y);
        assert!(mse < 1e-10, "MSE esperado: próximo de 0");
    }

    #[test]
    fn test_invalid_input() {
        let x = vec![1.0, 2.0];
        let y = vec![1.0];
        let result = LinearRegression::fit(&x, &y);
        assert!(result.is_err(), "Deve falhar com tamanhos diferentes");
    }
}