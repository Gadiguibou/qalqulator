#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Rational {
    numerator: i128,
    denominator: i128,
}

impl Rational {
    pub fn new(numerator: i128, denominator: i128) -> Rational {
        Rational {
            numerator,
            denominator,
        }
        .reduce()
    }

    fn reduce(self) -> Rational {
        let gcd: i128 = gcd::binary_u128(
            self.numerator.abs().try_into().unwrap(),
            self.denominator.abs().try_into().unwrap(),
        )
        .try_into()
        .unwrap();

        if self.denominator < 0 {
            Rational {
                numerator: -self.numerator / gcd,
                denominator: -self.denominator / gcd,
            }
        } else {
            Rational {
                numerator: self.numerator / gcd,
                denominator: self.denominator / gcd,
            }
        }
    }

    pub fn to_f64(self) -> f64 {
        self.numerator as f64 / self.denominator as f64
    }
}

impl std::fmt::Display for Rational {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.denominator != 1 {
            write!(f, "{}/{}", self.numerator, self.denominator)
        } else {
            write!(f, "{}", self.numerator)
        }
    }
}

impl std::ops::Add for Rational {
    type Output = Rational;

    fn add(self, rhs: Rational) -> Rational {
        Rational::new(
            self.numerator * rhs.denominator + rhs.numerator * self.denominator,
            self.denominator * rhs.denominator,
        )
        .reduce()
    }
}

impl std::ops::Sub for Rational {
    type Output = Rational;

    fn sub(self, rhs: Rational) -> Rational {
        Rational::new(
            self.numerator * rhs.denominator - rhs.numerator * self.denominator,
            self.denominator * rhs.denominator,
        )
        .reduce()
    }
}

impl std::ops::Mul for Rational {
    type Output = Rational;

    fn mul(self, rhs: Rational) -> Rational {
        Rational::new(
            self.numerator * rhs.numerator,
            self.denominator * rhs.denominator,
        )
        .reduce()
    }
}

impl std::ops::Div for Rational {
    type Output = Rational;

    fn div(self, rhs: Rational) -> Rational {
        Rational::new(
            self.numerator * rhs.denominator,
            self.denominator * rhs.numerator,
        )
        .reduce()
    }
}

impl std::ops::Rem for Rational {
    type Output = Rational;

    fn rem(self, rhs: Rational) -> Rational {
        Rational::new(
            self.numerator * rhs.denominator % rhs.numerator * self.denominator,
            self.denominator * rhs.denominator,
        )
        .reduce()
    }
}

impl std::ops::Neg for Rational {
    type Output = Rational;

    fn neg(self) -> Rational {
        Rational::new(-self.numerator, self.denominator)
    }
}

impl std::cmp::PartialOrd for Rational {
    fn partial_cmp(&self, other: &Rational) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Ord for Rational {
    fn cmp(&self, other: &Rational) -> std::cmp::Ordering {
        (self.numerator * other.denominator).cmp(&(other.numerator * self.denominator))
    }
}

pub enum PowResult {
    Rational(Rational),
    FloatingPoint(f64),
}

impl Rational {
    pub fn pow(self, other: Rational) -> PowResult {
        if other.denominator == 1 {
            if other.numerator >= 0 {
                PowResult::Rational(Rational::new(
                    self.numerator.pow(other.numerator as u32),
                    self.denominator.pow(other.numerator as u32),
                ))
            } else {
                PowResult::Rational(Rational::new(
                    self.denominator.pow(-other.numerator as u32),
                    self.numerator.pow(-other.numerator as u32),
                ))
            }
        } else {
            PowResult::FloatingPoint(self.to_f64().powf(other.to_f64()))
        }
    }
}
