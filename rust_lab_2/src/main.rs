/*
Questions:
1.  Using the derive attribute, implement the Debug, Clone, and Default traits for NumberWithUnit.
    Can you also derive Copy? What if you remove the String field from the structure? Why is this the
    case?
Answer:
    We can't derive Copy for the structure as it is now. The Copy trait requires that all fields of
    the structure also implement Copy. String does not implement Copy. Why? String manages
    heap-allocated memory, so copying it requires allocating new memory (that's what Clone does).
    Copy is for simple bitwise copies only. That's why we can derive Copy after removing String.

2.  Why second call pf mul_vals_vec in
    mul_vals(&x[0..2]); mul_vals_vec(x); mul_vals_vec(x);
    where x is a Vec won't compile?
Answer:
    The mul_vals function takes a slice &[x], which is just a reference (borrow) to the data. After
    calling it, x still exists and can be used again. However, mul_vals_vec takes Vec<x> by value,
    which means it takes ownership of the vector. When you call it the first time, x is moved into
    the function and no longer exists. The second call fails because x has already been moved and is
    no longer available. That's why cloning in first call is used.

3.  Why does from_strs(&string, str_slice) compile but from_strings(&string, str_slice) doesn't?
Answer:
    from_strs expects two &str parameters, from_strings expects two &String parameters but key is in
    type conversion. &String can be automatically coerced (converted) to &str through deref
    coercion, but &str cannot be converted to &String (we can't create a reference to a String that
    doesn't exist!).

4.  Why does string need & but str_slice doesn't in the first case?
Answer:
    string is of type String, so we need &string to get &String, which then coerces to &str and
    str_slice is already &str, so we pass it directly.
*/

fn main() {
    // Ex. 5-7
    let a = NumberWithUnit::unitless(12.5);
    let mut s1 = NumberWithUnit::with_unit(13.0, String::from("km"));
    let c = NumberWithUnit::with_unit_from(s1.clone(), 14.0);
    println!("a: {:?}", a);
    println!("s1: {:?}", s1);
    println!("b: {:?}", c);

    let s2 = NumberWithUnit::with_unit(17.0, String::from("km"));
    let t = NumberWithUnit::with_unit(2.0, String::from("h"));

    let mut s3 = NumberWithUnit::with_unit(10.0, String::from("cm"));
    let mut s4 = NumberWithUnit::with_unit(2.0, String::from("cm"));
    s3 = s3.add(s4.clone());
    NumberWithUnit::div_in_place(&mut s3, &t);
    s4 = s4.mul(s3.clone());
    NumberWithUnit::mul_in_place(&mut s4, &t);
    println!("s4: {:?}", s4);


    s1.add_in_place(&s2);
    println!("Po add in place dla s1: {:?}", s1);
    let v = s1.div(t);
    println!("Speed: {:?}", v);

    // Ex. 8-10
    let measurements = Vec::from(
        [NumberWithUnit::with_unit(5.5, String::from("m")),
            NumberWithUnit::with_unit(3.0, String::from("m")),
            NumberWithUnit::with_unit(10.0, String::from("m"))
        ]);

    println!("{:?}", mul_vals(&measurements[0..2]));
    println!("{:?}", mul_vals_vec(measurements.clone()));
    println!("{:?}", mul_vals_vec(measurements));

    // Ex. 11-15
    let string = String::from("hello");
    let str_slice: &str = "world";

    // 15 (a)
    let double_string_1 = DoubleString::from_strs(&string, str_slice);
    double_string_1.show();

    // 15 (b)
    //let double_string_2 = DoubleString::from_strings(&string, str_slice); // Error
    let double_string_2 =DoubleString::from_strings(&string, &str_slice.to_string());
    double_string_2.show();
}

#[derive(Debug, Clone, Default)]
struct NumberWithUnit {
    unit: String,
    value: f64,
}

impl NumberWithUnit {
    fn unitless(value: f64) -> Self {
        Self { value, unit: String::new() }
    }

    fn with_unit(value: f64, unit: String) -> Self {
        Self {value, unit}
    }

    fn with_unit_from(other: Self, value: f64) -> Self {
        Self {value, unit: other.unit.clone()}
    }

    fn add(self, other: Self) -> Self {
        if self.unit == other.unit {
            let val = self.value + other.value;
            NumberWithUnit::with_unit_from(self, val)
        }
        else {
            panic!();
        }
    }

    fn mul(self, other: Self) -> Self {
        Self {value: self.value * other.value, unit: [self.unit, other.unit].join("*")}
    }

    fn div(self, other: Self) -> Self {
        Self {value: self.value / other.value, unit: [self.unit, other.unit].join("/")}
    }

    fn add_in_place(&mut self, other: &Self) {
        if self.unit == other.unit {
            self.value += other.value;
        }
        else {
            panic!();
        }
    }

    fn mul_in_place(&mut self, other: &Self) {
        self.value *= other.value;
        self.unit = [self.unit.clone(), other.unit.clone()].join("*");
    }

    fn div_in_place(&mut self, other: &Self) {
        self.value /= other.value;
        self.unit = [self.unit.clone(), other.unit.clone()].join("/");
    }
}

fn mul_vals(slice: &[NumberWithUnit]) -> NumberWithUnit {
    if slice.is_empty() {
        return NumberWithUnit::default();
    }

    let product = slice.iter()
        .map(|x| x.value)
        .product();

    let combined = slice.iter()
        .map(|n| n.unit.as_str())
        .collect::<Vec<&str>>()
        .join("*");

    NumberWithUnit {
        value: product,
        unit: combined
    }
}

fn mul_vals_vec(numbers: Vec<NumberWithUnit>) -> NumberWithUnit {
    if numbers.is_empty() {
        return NumberWithUnit::default();
    }

    let product = numbers.iter()
        .map(|x| x.value)
        .product();

    let combined = numbers.iter()
        .map(|n| n.unit.as_str())
        .collect::<Vec<&str>>()
        .join("*");

    NumberWithUnit {
        value: product,
        unit: combined
    }
}

struct DoubleString(String, String);

impl DoubleString {
    fn from_strs(str_1: &str, str_2: &str) -> Self {
        Self(str_1.to_string(), str_2.to_string())
    }

    fn from_strings(str_1: &String, str_2: &String) -> Self {
        Self(str_1.clone(), str_2.clone())
    }

    fn show(&self) {
        println!("({}, {})", self.0, self.1);
    }
}