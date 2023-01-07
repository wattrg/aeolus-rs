use std::ops::Deref;
use std::str::FromStr;
use std::cmp::Ordering;

use super::number::Real;

use ndarray::{Array1, Array2};
use ndarray_linalg::Solve;

/// The base type representing a unit
/// The unit is stored as an array of powers
/// for the particular unit, in order [mass, length, time, temp]
/// e.g. m/s -> [0, 1, -1, 0]
pub type UnitBase = [i8; 4];

/// Represents a unit for a number
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Unit (UnitBase);

#[derive(Debug, PartialEq, Eq)]
pub struct UnitParseError{ unit: String }

impl Deref for Unit {
    type Target = UnitBase;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// A number with a unit
#[derive(Debug, PartialEq)]
pub struct UnitNum {
    pub value: Real,
    unit: Unit,
}

impl UnitNum {
    pub fn new(value: Real, unit_str: &str) -> UnitNum {
        let unit = Unit::from_str(unit_str).unwrap();
        UnitNum{value, unit}
    }

    pub fn unit(&self) -> &Unit {
        &self.unit
    }
}

impl std::ops::Add for UnitNum {
    type Output = UnitNum;

    fn add (self, other: Self) -> Self::Output {
        assert_eq!(self.unit, other.unit);
        UnitNum{value: self.value + other.value, unit: self.unit}
    }
}

impl std::ops::Sub for UnitNum {
    type Output = UnitNum;

    fn sub (self, other: Self) -> Self::Output {
        assert_eq!(self.unit, other.unit);
        UnitNum{value: self.value - other.value, unit: self.unit}
    }
}

impl std::ops::Mul for UnitNum {
    type Output = UnitNum;

    fn mul(self, rhs: Self) -> Self::Output {
        let value = self.value * rhs.value;
        let mut unit = UnitBase::default();
        for (i, elem) in unit.iter_mut().enumerate() {
            *elem = self.unit[i] + rhs.unit[i];
        }
        UnitNum{value, unit: Unit(unit)}
    }
}

impl std::ops::Div for UnitNum {
    type Output = UnitNum;

    fn div(self, rhs: Self) -> Self::Output {
        let value = self.value / rhs.value;
        let mut unit = UnitBase::default();
        for (i, elem) in unit.iter_mut().enumerate() {
            *elem = self.unit[i] - rhs.unit[i];
        }
        UnitNum{value, unit: Unit(unit)}
    }
}



impl FromStr for Unit {
    type Err = UnitParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut unit = s;
        let mut unit_rep: UnitBase = [0, 0, 0, 0];

        let mut sign = 1;
        let mut pow;
        loop {
            if unit.starts_with("kg"){
                unit = &unit[2..];
                (unit, pow) = read_and_remove_power(unit);
                unit_rep[0] += sign * pow;
            }
            else if unit.starts_with('m') {
                unit = &unit[1..];
                (unit, pow) = read_and_remove_power(unit);
                unit_rep[1] += sign * pow;
            }
            else if unit.starts_with('s') {
                unit = &unit[1..];
                (unit, pow) = read_and_remove_power(unit);
                unit_rep[2] += sign * pow;
            }
            else if unit.starts_with('K') {
                unit = &unit[1..];
                (unit, pow) = read_and_remove_power(unit);
                unit_rep[3] += sign * pow;
            }
            else if unit.starts_with('*') {
                sign = 1;
                unit = &unit[1..];
            }
            else if unit.starts_with('/') {
                sign = -1;
                unit = &unit[1..];
            }
            else {
                return Err(UnitParseError{unit: s.to_string()});
            }
            if unit.chars().count() == 0 {
                break;
            }
        }

        Ok(Unit(unit_rep))
    }
}

fn first_char_to_i8(unit: &str) -> i8 {
    unit.chars()
        .next()
        .unwrap()
        .to_string()
        .parse()
        .unwrap()
}

fn read_and_remove_power(mut unit: &str) -> (&str, i8) {
    if unit.starts_with('^') {
        unit = &unit[1..]; 
        let pow = first_char_to_i8(unit);
        unit = &unit[1..];
        return (unit, pow);
    }
    (unit, 1)
}

pub struct RefDim {
    ref_mass: Real,
    ref_length: Real,
    ref_time: Real,
    ref_temp: Real,
}

impl RefDim {
    pub fn new(reference_values: Vec<UnitNum>) -> RefDim {
        let (included_units, n_units) = RefDim::count_units(&reference_values);
        let mut a = Array2::<Real>::zeros((n_units, n_units));
        for row in 0..n_units {
            for col in 0..n_units {
                let unit_index = included_units[col];
                a[[row, col]] = reference_values[row].unit()[unit_index] as f64;
            }
        }
        let mut b = Array1::<Real>::zeros(n_units);
        for i in 0..n_units {
            b[i] = reference_values[i].value.log10();
        }
        let x_star = a.solve(&b).unwrap();
        let mut x = [0.; 4];
        for (i, x_star_i) in x_star.iter().enumerate() {
            x[included_units[i]] = 10.0_f64.powf(*x_star_i);
        }
        RefDim{
            ref_mass: x[0],
            ref_length: x[1],
            ref_time: x[2],
            ref_temp: x[3],
        }
    }

    pub fn mass(&self) -> Real {
        self.ref_mass
    }

    pub fn length(&self) -> Real {
        self.ref_length
    }

    pub fn time(&self) -> Real {
        self.ref_time
    }

    pub fn temp(&self) -> Real {
        self.ref_temp
    }

    pub fn velocity(&self) -> Real {
        self.ref_length / self.ref_time
    }

    pub fn density(&self) -> Real {
        self.ref_mass / self.ref_length.powi(3)
    }

    pub fn viscosity(&self) -> Real {
        self.ref_length * self.velocity()
    }

    fn count_units(reference_values: &Vec<UnitNum>) -> (Vec<usize>, usize) {
        let mut included_units = Vec::new();
        for reference_value in reference_values.iter() {
            let unit = reference_value.unit();
            for (i, val) in unit.into_iter().enumerate() {
                if val != 0 && !included_units.contains(&i) {
                    included_units.push(i);
                }
            }
        }
        included_units.sort();
        let n_units = included_units.len();
        match reference_values.len().cmp(&n_units){
            Ordering::Less => panic!("Under constrained system of reference units"),
            Ordering::Greater => panic!("Over constrained system of reference units"),
            Ordering::Equal => (included_units, n_units),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_from_string() {
        let unit = Unit::from_str("kg^2/m^3*s/K").unwrap();
        assert_eq!(unit, Unit([2, -3, 1, -1]));
    }

    #[test]
    fn unit_from_string_failure() {
        let unit = Unit::from_str("kg/m/d");
        assert_eq!(unit, Err(UnitParseError{unit: "kg/m/d".to_string()}));
    }

    #[test]
    fn add_unit_nums() {
        let num1 = UnitNum::new(1., "kg/m/s");
        let num2 = UnitNum::new(2., "kg/m/s");
        let result = UnitNum::new(3., "kg/m/s");

        assert_eq!(num1 + num2, result);
    }

    #[test]
    #[should_panic]
    fn add_incompatible_unit_nums() {
        let num1 = UnitNum::new(1., "kg/m/s");
        let num2 = UnitNum::new(2., "kg/m^3/s");
        let _result = num1 + num2;
    }

    #[test]
    fn sub_unit_nums() {
        let num1 = UnitNum::new(2., "kg/m/s");
        let num2 = UnitNum::new(1., "kg/m/s");
        let result = UnitNum::new(1., "kg/m/s");

        assert_eq!(num1 - num2, result);
    }

    #[test]
    #[should_panic]
    fn sub_incompatible_unit_nums() {
        let num1 = UnitNum::new(2., "kg/m/s");
        let num2 = UnitNum::new(1., "kg/m^2/s");
        let _result = num1 - num2;
    }

    #[test]
    fn mul_unit_nums() {
        let num1 = UnitNum::new(2., "kg/m^3");
        let num2 = UnitNum::new(3., "m^3");
        let result = UnitNum::new(6., "kg");

        assert_eq!(num1 * num2, result);
    }

    #[test]
    fn div_unit_nums() {
        let num1 = UnitNum::new(6., "kg*m^2/s");
        let num2 = UnitNum::new(3., "s");
        let result = UnitNum::new(2., "kg*m^2/s^2");

        assert_eq!(num1/num2, result);
    }

    #[test]
    fn ref_dim() {
        let length = UnitNum::new(6., "m");
        let velocity = UnitNum::new(1., "m/s");
        let density = UnitNum::new(2., "kg/m^3");
        let ref_dim = RefDim::new(vec![length, velocity, density]);

        assert!((ref_dim.length() - 6.0) < 1e-13);
        assert!((ref_dim.velocity() - 1.0) < 1e-13);
        assert!((ref_dim.density() - 2.) < 1e-13);
        assert!((ref_dim.mass() - 432.0) < 1e-13);
        assert!((ref_dim.time() - 6.0) < 1e-13);
    }

    #[test]
    fn ref_dim_temp() {
        let mass = UnitNum::new(6., "kg");
        let time = UnitNum::new(2., "s");
        let temp = UnitNum::new(3., "K");
        let ref_dim = RefDim::new(vec![mass, time, temp]);

        assert!((ref_dim.temp() - 3.) < 1e-13);
        assert!((ref_dim.mass() - 6.) < 1e-13);
        assert!((ref_dim.time() - 2.) < 1e-13);
    }

    #[test]
    #[should_panic]
    fn under_constrained_ref_dim() {
        let density = UnitNum::new(1., "kg/m^3");
        let velocity = UnitNum::new(2., "m/s");

        let _ref_dim = RefDim::new(vec![density, velocity]);
    }

    #[test]
    #[should_panic]
    fn over_constrained_ref_dim() {
        let density = UnitNum::new(1., "kg/m^3");
        let velocity = UnitNum::new(2., "m/s");
        let length = UnitNum::new(3., "m");
        let mass = UnitNum::new(4., "kg");

        let _ref_dim = RefDim::new(vec![density, velocity, length, mass]);
    }
}

