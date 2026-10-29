#[derive(Debug, Clone, Default)]
// String nie jest Copy -> nie możemy użyć derive(Copy), bez Stringa w struct Copy działa
struct NumberWithUnit{
    unit: String,
    value: f64,
}

impl NumberWithUnit {
    fn unitless(value: f64) -> Self {
        NumberWithUnit {
            unit: String::new(),
            value,
        }
    }

    fn with_unit(value: f64, unit: String) -> Self {
        NumberWithUnit {
            unit,
            value,
        }
    }

    fn with_unit_from(other: Self, value: f64) -> Self {
        NumberWithUnit {
            unit: other.unit,
            value,
        }
    }

    fn add(self, other: Self) -> Self {
        if self.unit != other.unit {
            panic!("Cannot add numbers with different units: '{}' and '{}'", self.unit, other.unit);
        }
        else {
            NumberWithUnit {
                unit: self.unit,
                value: self.value + other.value,
            }
        }
    }

    fn add_in_place(&mut self, other: &Self) {
        if self.unit != other.unit {
            panic!("Cannot add numbers with different units: '{}' and '{}'", self.unit, other.unit);
        }
        else {
            self.value += other.value;
        }
    }

    fn mul(self, other: Self) -> Self {
        NumberWithUnit { 
            unit: format!("{}*{}", self.unit, other.unit), 
            value: self.value * other.value 
        }
    }

    fn mul_in_place(&mut self, other: &Self) {
        self.unit = format!("{}*{}", self.unit, other.unit);
        self.value *= other.value;
    }

    fn div(self, other: Self) -> Self {
        NumberWithUnit { 
            unit: format!("{}/{}", self.unit, other.unit),
            value: self.value / other.value
        }
    }

    fn div_in_place(&mut self, other: &Self) {
        self.unit = format!("{}/{}", self.unit, other.unit);
        self.value /= other.value;
    }
}

#[derive(Debug)]
struct DoubleString(String, String);

impl DoubleString {
    fn from_strs(str_1: &str, str_2: &str) -> Self {
        DoubleString(str_1.to_string(), str_2.to_string())
    }

    fn from_strings(str_1: &String, str_2: &String) -> Self {
        DoubleString(str_1.clone(), str_2.clone())
    }

    fn show(&self) {
        println!("({}, {})", self.0, self.1);
    }
}

fn mul_vals(values: &[NumberWithUnit]) -> NumberWithUnit {
    let mut result = NumberWithUnit::unitless(1.0);
    for value in values {
        result.mul_in_place(value);
    }
    result
}

fn mul_vals_vec(values: Vec<NumberWithUnit>) -> NumberWithUnit {
    let mut result = NumberWithUnit::unitless(1.0);
    for value in values {
        result = result.mul(value);
    }
    result
}

fn main() {
    let a = NumberWithUnit::unitless(2.0);
    println!("{} '{}'", a.value, a.unit);

    let b = NumberWithUnit::with_unit(3.5, String::from("m"));
    println!("{} '{}'", b.value, b.unit);

    let c = NumberWithUnit::with_unit_from(b, 7.0);
    println!("{} '{}'", c.value, c.unit);
    
    let x = NumberWithUnit::default();
    let y = x.clone();
    println!("{x:?} {y:?}");

    let dist = NumberWithUnit::with_unit(10.0, String::from("m"));
    let time = NumberWithUnit::with_unit(2.0, String::from("s"));

    let speed = dist.div(time);
    println!("{:?}", speed);

    let mut d = c.clone().add(c.clone());
    d.add_in_place(&c);
    d.div_in_place(&speed);
    println!("{:?}", d);

    let values = vec![
        NumberWithUnit::with_unit(2.0, String::from("m")),
        NumberWithUnit::with_unit(3.0, String::from("s")),
        NumberWithUnit::with_unit(4.0, String::from("kg")),
    ];

    println!("{:?}", mul_vals(&values));
    println!("{:?}", mul_vals(&values));

    // println!("{:?}", mul_vals_vec(values)); <- nie działa, bo values zostało przeniesione do funkcji
    println!("{:?}", mul_vals_vec(values.clone()));
    println!("{:?}", mul_vals_vec(values));

    let string = String::from("Hello");
    let str_slice = "world";

    let ds1 = DoubleString::from_strs(&string, str_slice);
    ds1.show();

    // DoubleString::from_strings(&string, str_slice);
    let string_2 = String::from(str_slice);
    let ds2 = DoubleString::from_strings(&string, &string_2);
    ds2.show();
}
