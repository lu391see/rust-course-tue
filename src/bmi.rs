#[derive(Debug, PartialEq)]
pub enum BmiCategory {
    // SevereUnderweight,
    ModerateUnderweight,
    // MildUnderweight,
    NormalRange,
    Overweight,
    ObeseClass1,
    // ObeseClass2,
    // ObeseClass3,
}

#[derive(Debug, PartialEq)]
pub struct BodyMassIndex {
    value: f64,
    category: BmiCategory,
}

impl BodyMassIndex {
    pub fn new(bmi: f64) -> BodyMassIndex {
        let range = match bmi {
            x if x < 18.5 => BmiCategory::ModerateUnderweight,
            x if x < 25.0 => BmiCategory::NormalRange,
            x if x < 30.0 => BmiCategory::Overweight,
            _ => BmiCategory::ObeseClass1,
        };

        BodyMassIndex {
            value: (bmi),
            category: (range),
        }
    }

    pub fn value(&self) -> f64 {
        self.value
    }

    pub fn category(&self) -> &BmiCategory {
        &self.category // because it is so small, one can instead use Clone and Copy trait for enum -> then, no reference is needed
    }
}
