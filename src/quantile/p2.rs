// https://github.com/AndreyAkinshin/perfolizer/blob/master/src/Perfolizer/Perfolizer/Mathematics/QuantileEstimators/P2QuantileEstimator.cs

#[derive(Debug)]
#[cfg_attr(feature = "with_serde", derive(serde::Serialize, serde::Deserialize))]
pub struct P2 {
    p: f64,
    n: [i32; 5],  // marker positions
    ns: [f64; 5], // desired marker positions
    dns: [f64; 5],
    q: [f64; 5], // marker heights
    count: usize,
}

impl P2 {
    pub fn new(p: f64) -> Self {
        assert!(p > 0.);
        assert!(p < 1.);

        P2 {
            p,
            n: [0; 5],
            ns: [0.; 5],
            dns: [0.; 5],
            q: [0.; 5],
            count: 0,
        }
    }

    pub fn add_value(&mut self, x: f64) {
        if self.count < 5 {
            self.q[self.count] = x;
            self.count += 1;

            if self.count == 5 {
                self.q.sort_by(|a, b| a.partial_cmp(b).unwrap());

                for i in 0..5 {
                    self.n[i] = i as i32;
                }

                self.ns[0] = 0.;
                self.ns[1] = 2. * self.p;
                self.ns[2] = 4. * self.p;
                self.ns[3] = 2. + 2. * self.p;
                self.ns[4] = 4.;

                self.dns[0] = 0.;
                self.dns[1] = self.p / 2.;
                self.dns[2] = self.p;
                self.dns[3] = (1. + self.p) / 2.;
                self.dns[4] = 1.;
            }

            return;
        }

        let k = if x < self.q[0] {
            self.q[0] = x;
            0
        } else if x < self.q[1] {
            0
        } else if x < self.q[2] {
            1
        } else if x < self.q[3] {
            2
        } else if x < self.q[4] {
            3
        } else {
            self.q[4] = x;
            3
        };

        for i in (k + 1)..5 {
            self.n[i] += 1;
        }

        for i in 0..5 {
            self.ns[i] += self.dns[i];
        }

        for i in 1..=3 {
            let d = self.ns[i] - self.n[i] as f64;

            if d >= 1. && self.n[i + 1] - self.n[i] > 1
                || d <= -1. && self.n[i - 1] - self.n[i] < -1
            {
                let dInt = d.signum() as i32;
                let qs = self.parabolic(i, dInt);

                self.q[i] = if self.q[i - 1] < qs && qs < self.q[i + 1] {
                    qs
                } else {
                    self.linear(i, dInt)
                };

                self.n[i] += dInt;
            }
        }

        self.count += 1;
    }

    fn parabolic(&self, i: usize, d: i32) -> f64 {
        self.q[i]
            + d as f64 / ((self.n[i + 1] - self.n[i - 1]) as f64)
                * (((self.n[i] - self.n[i - 1] + d) as f64) * (self.q[i + 1] - self.q[i])
                    / (self.n[i + 1] - self.n[i]) as f64
                    + (self.n[i + 1] - self.n[i] - d) as f64 * (self.q[i] - self.q[i - 1])
                        / (self.n[i] - self.n[i - 1]) as f64)
    }

    fn linear(&self, i: usize, d: i32) -> f64 {
        let num = d as f64 * (self.q[(i as i32 + d) as usize] - self.q[i]);
        let denom = self.n[(i as i32 + d) as usize] - self.n[i];

        self.q[i] + (num / denom as f64)
    }

    pub fn get_quantile(&self) -> f64 {
        if self.count <= 5 {
            let mut q = self.q.clone();
            q.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let index = ((self.count - 1) as f64 * self.p).round() as usize;
            q[index]
        } else {
            self.q[2]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let data = vec![
            0.02, 0.15, 0.74, 3.39, 0.83, 22.37, 10.15, 15.43, 38.62, 15.92, 34.60, 10.28, 1.47,
            0.40, 0.05, 11.39, 0.27, 0.42, 0.09, 11.37,
        ];

        let mut estimator = P2::new(0.5);

        for value in data {
            estimator.add_value(value);
            println!("{:?}", estimator);
        }

        // The unit test in the C# project claims the expected is 4.44, but the value they're
        // getting is:
        let expected = 4.4406343532603376;

        assert!((estimator.get_quantile() - expected) < 0.00000001);
    }
}
