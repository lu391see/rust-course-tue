#[cfg(test)]
mod tests {
    // keine Netzwerkzugriffe, Filesystezugriffe oder Environment Variablen setzen/ändern
    use crate::{calc_bmi, BmiCategory, Height, Weight, BmiError};

    #[test]
    fn test_bmi_obese() {
        assert_eq!(
            calc_bmi(Weight(66.6f64), Height(1.42f64)).unwrap().category,
            BmiCategory::ObeseClass1
        )
    }

    /*
    #[test]
    #[should_panic]
    fn test_should_panic() {
        panic!()
    } */

    #[test]
    fn test_bmi_value() {
        let bmi = calc_bmi(Weight(12.3f64), Height(1.42f64));
        assert_eq!(bmi.unwrap().value, 6.09998)
    }

    #[test]
    fn test_bmi_underweight() {
        assert_eq!(
            calc_bmi(Weight(12.3f64), Height(1.42f64)).unwrap().category,
            BmiCategory::ModerateUnderweight
        )
    }

    #[test]
    fn test_height_zero() {
        let res = calc_bmi(Weight(12.3), Height(0.0));
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), BmiError::HeightCannotBeZeroOrSmaller)
    }

    #[test]
    fn test_weight_smaller_zero() {
        let res = calc_bmi(Weight(-12.3), Height(1.0));
        assert!(res.is_err());
        // assert!(res.map_err(|e| e == BmiError::WeightCannotBeZeroOrSmaller).unwrap_or(false))
        assert_eq!(res.unwrap_err(), BmiError::WeightCannotBeZeroOrSmaller)
    }

    // TODO: test negative inputs
}