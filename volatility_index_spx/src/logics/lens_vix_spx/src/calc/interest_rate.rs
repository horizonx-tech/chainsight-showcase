#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CmtYield {
    pub days: u32,
    pub yield_: f64,
}
pub fn interest_rate(yields: Vec<CmtYield>, t: f64) -> f64 {
    let x: Vec<f64> = yields.iter().map(|y| y.days as f64).collect();
    let y: Vec<f64> = yields.iter().map(|y| y.yield_).collect();
    let mut spline = MonotonicCubicSpline::new(&x, &y);
    let bet_y = spline.interpolate(t);
    compounded_apy_rate_from_bey_r(bet_y)
}

struct MonotonicCubicSpline {
    m_x: Vec<f64>,
    m_y: Vec<f64>,
    m_m: Vec<f64>
}
impl MonotonicCubicSpline {
    fn new(x : &Vec<f64>, y : &Vec<f64>) -> MonotonicCubicSpline {
        assert!(x.len() == y.len() && x.len() >= 2 && y.len() >= 2, "Must have at least 2 control points.");

        let n = x.len();

        let mut secants = vec![0.0 ; n - 1];
        let mut slopes  = vec![0.0 ; n];

        for i in 0..(n-1) {
            let h = *x.get(i + 1).unwrap() - *x.get(i).unwrap();
            assert!(h > 0.0, "Control points must be monotonically increasing.");
            secants[i] = (*y.get(i + 1).unwrap() - *y.get(i).unwrap()) / h;

        }

        slopes[0] = secants[0];
        for i in 1..(n-1) {
            slopes[i] = (secants[i - 1] + secants[i]) * 0.5;
        }
        slopes[n - 1] = secants[n - 2];

        for i in 0..(n-1) {
            if secants[i] == 0.0 {
                slopes[i] = 0.0;
                slopes[i + 1] = 0.0;
            } else {
                let alpha = slopes[i] / secants[i];
                let beta = slopes[i + 1] / secants[i];
                let h = alpha.hypot(beta);
                if h > 9.0 {
                    let t = 3.0 / h;
                    slopes[i] = t * alpha * secants[i];
                    slopes[i + 1] = t * beta * secants[i];
                }
            }
        }

        let spline = MonotonicCubicSpline {
            m_x: x.clone(),
            m_y: y.clone(),
            m_m: slopes
        };
        return spline;
    }

    fn hermite(point: f64, x : (f64, f64), y: (f64, f64), m: (f64, f64)) -> f64 {
        let h: f64 = x.1 - x.0;
        let t = (point - x.0) / h;
        return (y.0 * (1.0 + 2.0 * t) + h * m.0 * t) * (1.0 - t) * (1.0 - t)
            + (y.1 * (3.0 - 2.0 * t) + h * m.1 * (t - 1.0)) * t * t;
    }

    fn interpolate(&mut self, point : f64) -> f64 {
        let n = self.m_x.len();

        if point <= *self.m_x.get(0).unwrap() {
            return *self.m_y.get(0).unwrap();
        }
        if point >= *self.m_x.get(n - 1).unwrap() {
            return *self.m_y.get(n - 1).unwrap();
        }

        let mut i = 0;
        while point >= *self.m_x.get(i + 1).unwrap() {
            i += 1;
            if point == *self.m_x.get(i).unwrap() {
                return *self.m_y.get(i).unwrap();
            }
        }
        return MonotonicCubicSpline::hermite(
            point,
            (*self.m_x.get(i).unwrap(), *self.m_x.get(i+1).unwrap()),
            (*self.m_y.get(i).unwrap(), *self.m_y.get(i+1).unwrap()),
            (*self.m_m.get(i).unwrap(), *self.m_m.get(i+1).unwrap())
        );
    }

    fn partial(x: Vec<f64>, y: Vec<f64>) -> impl Fn(f64) -> f64 {
        move |p| {
            let mut spline = MonotonicCubicSpline::new(&x, &y);
            spline.interpolate(p)
        }
    }
}

// Convert BEY to APY and then to continuously compounded rate
fn compounded_apy_rate_from_bey_r(bey_r: f64) -> f64 {
    let apy_t = ((1.0 + bey_r / 2.0) as f64).powf(2.0) - 1.0;
    (1.0 + apy_t).ln()
}
