use std::ops::Deref;
use std::str::FromStr;

pub type Number = f64;

/// A number with a unit
#[derive(Debug, PartialEq)]
pub struct UnitNum {
    pub value: Number,
    unit: Unit,
}

impl UnitNum {
    pub fn new(value: Number, unit_str: &str) -> Result<UnitNum, UnitParseError> {
        let unit = Unit::from_str(unit_str)?;
        Ok(UnitNum{value, unit})
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
        let mut unit = UnitType::default();
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
        let mut unit = UnitType::default();
        for (i, elem) in unit.iter_mut().enumerate() {
            *elem = self.unit[i] - rhs.unit[i];
        }
        UnitNum{value, unit: Unit(unit)}
    }
}

pub type UnitType = [i8; 3];

/// Represents a unit for a number
// The unit is stored as an array of powers
// for the particular unit, in order [mass, length, time]
// e.g. m/s -> [1, 0, -1]
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Unit (UnitType);

#[derive(Debug, PartialEq, Eq)]
pub struct UnitParseError{ unit: String }

impl Deref for Unit {
    type Target = UnitType;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


impl FromStr for Unit {
    type Err = UnitParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut unit = s;
        let mut unit_rep: UnitType = [0, 0, 0];

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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_from_string() {
        let unit = Unit::from_str("kg^2/m^3*s").unwrap();
        assert_eq!(unit, Unit([2, -3, 1]));
    }

    #[test]
    fn unit_from_string_failure() {
        let unit = Unit::from_str("kg/m/d");
        assert_eq!(unit, Err(UnitParseError{unit: "kg/m/d".to_string()}));
    }

    #[test]
    fn add_unit_nums() {
        let num1 = UnitNum::new(1., "kg/m/s").unwrap();
        let num2 = UnitNum::new(2., "kg/m/s").unwrap();
        let result = UnitNum::new(3., "kg/m/s").unwrap();

        assert_eq!(num1 + num2, result);
    }

    #[test]
    #[should_panic]
    fn add_incompatible_unit_nums() {
        let num1 = UnitNum::new(1., "kg/m/s").unwrap();
        let num2 = UnitNum::new(2., "kg/m^3/s").unwrap();
        let _result = num1 + num2;
    }

    #[test]
    fn sub_unit_nums() {
        let num1 = UnitNum::new(2., "kg/m/s").unwrap();
        let num2 = UnitNum::new(1., "kg/m/s").unwrap();
        let result = UnitNum::new(1., "kg/m/s").unwrap();

        assert_eq!(num1 - num2, result);
    }

    #[test]
    #[should_panic]
    fn sub_incompatible_unit_nums() {
        let num1 = UnitNum::new(2., "kg/m/s").unwrap();
        let num2 = UnitNum::new(1., "kg/m^2/s").unwrap();
        let _result = num1 - num2;
    }

    #[test]
    fn mul_unit_nums() {
        let num1 = UnitNum::new(2., "kg/m^3").unwrap();
        let num2 = UnitNum::new(3., "m^3").unwrap();
        let result = UnitNum::new(6., "kg").unwrap();

        assert_eq!(num1 * num2, result);
    }

    #[test]
    fn div_unit_nums() {
        let num1 = UnitNum::new(6., "kg*m^2/s").unwrap();
        let num2 = UnitNum::new(3., "s").unwrap();
        let result = UnitNum::new(2., "kg*m^2/s^2").unwrap();

        assert_eq!(num1/num2, result);
    }
}

