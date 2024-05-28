use std::{f64::INFINITY, fmt};

#[derive(Debug, Clone, Copy)]
pub struct Fraction {
    pub sign: bool,
    pub numerator: u64,
    pub denominator: u64,
}

const EPSILON: f64 = 0.0000001;

fn integral(num: f64) -> bool {
    let n = num - num.trunc();
    n.abs() <= EPSILON || (1.0 - n).abs() <= EPSILON
}

fn divisible(num: u64, by: u64) -> bool {
    (num / by) * by == num
}

fn lcm(a: u64, b: u64) -> u64 {
    let (x, y) = simplify(a, b);
    u64::max(x, y) * u64::min(a, b)
}

fn simplify(numer: u64, denom: u64) -> (u64, u64) {
    let max = u64::max(numer, denom) / 2;
    for i in 2..=max {
        // Divisible
        if divisible(numer, i) && divisible(denom, i) {
            return simplify(numer / i, denom / i);
        }
    }
    (numer, denom)
}

impl Fraction {
    pub fn new(sign: bool, numerator: u64, denominator: u64) -> Fraction {
        let (n, d) = simplify(numerator, denominator);

        Fraction {
            sign,
            numerator: n,
            denominator: d,
        }
    }

    pub fn pos(numerator: u64, denominator: u64) -> Fraction {
        Fraction::new(true, numerator, denominator)
    }

    pub fn neg(numerator: u64, denominator: u64) -> Fraction {
        Fraction::new(false, numerator, denominator)
    }

    pub fn from(decimal: f64) -> Fraction {
        match decimal {
            x if x == 0.0 => Fraction {
                sign: true,
                numerator: 0,
                denominator: 1,
            },
            x if integral(x) => Fraction {
                sign: decimal >= 0.0,
                numerator: decimal.abs() as u64,
                denominator: 1,
            },
            x if x.is_nan() => Fraction {
                sign: true,
                numerator: 0,
                denominator: 0,
            },
            x if x.is_infinite() => Fraction {
                sign: x == INFINITY,
                numerator: 1,
                denominator: 0,
            },
            _ => {
                let mut i = 1;
                loop {
                    let defrac = (decimal * i as f64).abs();
                    if integral(defrac) {
                        break;
                    }
                    i += 1;

                    if i > 1 << 6 {
                        break;
                    }
                }

                Fraction {
                    sign: decimal >= 0.0,
                    numerator: ((decimal * i as f64).abs() - EPSILON) as u64 + 1,
                    denominator: i,
                }
            }
        }
    }

    pub fn reciprocal(self: &Self) -> Fraction {
        Fraction {
            sign: self.sign,
            numerator: self.denominator,
            denominator: self.numerator,
        }
    }

    pub fn value(self: &Self) -> f64 {
        let mag = self.numerator as f64 / self.denominator as f64;
        if self.sign {
            mag
        } else {
            -mag
        }
    }
}

impl fmt::Display for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.numerator == 0 {
            write!(f, "{}", if self.denominator == 0 { "nan" } else { "0" })
        } else {
            let sign = if self.sign { "" } else { "-" };

            if self.denominator == 0 {
                write!(f, "{}inf", sign)
            } else if self.denominator == 1 {
                write!(f, "{}{}", sign, self.numerator)
            } else {
                write!(f, "{}{}/{}", sign, self.numerator, self.denominator)
            }
        }
    }
}

fn sign_sub(a: u64, b: u64) -> (u64, bool) {
    if a > b {
        (a - b, true)
    } else {
        (b - a, false)
    }
}

impl std::ops::Add for Fraction {
    type Output = Fraction;

    fn add(self, rhs: Self) -> Self::Output {
        let denom = lcm(self.denominator, rhs.denominator);

        let numer_left = self.numerator * (denom / self.denominator);
        let numer_right = rhs.numerator * (denom / rhs.denominator);

        let (numer, sign) = match (self.sign, rhs.sign) {
            (true, true) => (numer_left + numer_right, true),
            (true, false) => sign_sub(numer_left, numer_right),
            (false, true) => sign_sub(numer_right, numer_left),
            (false, false) => (numer_left + numer_right, false),
        };

        Fraction {
            sign,
            numerator: numer,
            denominator: denom,
        }
    }
}

impl std::ops::Sub for Fraction {
    type Output = Fraction;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl std::ops::Mul for Fraction {
    type Output = Fraction;

    fn mul(self, rhs: Self) -> Self::Output {
        let numer = self.numerator * rhs.numerator;
        let denom = self.denominator * rhs.denominator;
        let sign = self.sign == rhs.sign;

        let (numerator, denominator) = simplify(numer, denom);

        Fraction {
            sign,
            numerator,
            denominator,
        }
    }
}

impl std::ops::Div for Fraction {
    type Output = Fraction;

    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.reciprocal()
    }
}

impl std::ops::Neg for Fraction {
    type Output = Fraction;

    fn neg(self) -> Self::Output {
        Fraction {
            sign: !self.sign,
            numerator: self.numerator,
            denominator: self.denominator,
        }
    }
}

impl std::ops::Add<f64> for Fraction {
    type Output = Fraction;

    fn add(self, rhs: f64) -> Self::Output {
        self + Fraction::from(rhs)
    }
}

impl std::ops::Sub<f64> for Fraction {
    type Output = Fraction;

    fn sub(self, rhs: f64) -> Self::Output {
        self - Fraction::from(rhs)
    }
}

impl std::ops::Mul<f64> for Fraction {
    type Output = Fraction;

    fn mul(self, rhs: f64) -> Self::Output {
        self * Fraction::from(rhs)
    }
}

impl std::ops::Div<f64> for Fraction {
    type Output = Fraction;

    fn div(self, rhs: f64) -> Self::Output {
        self / Fraction::from(rhs)
    }
}
