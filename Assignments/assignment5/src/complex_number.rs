// structure containing four variants as complex numbers
pub struct ComplexNumbers {
    pub real_number1: i32,
    pub real_number2: i32,
    pub imaginary_number1: i32,
    pub imaginary_number2: i32,
}
// function addition performing addition operation on complex numbers
//
// #Arguemnts
//
// self as a parameter from complex structure
impl ComplexNumbers {
    pub fn addition(&self) -> String {
        let add_real_values: i32 = self.real_number1 + self.real_number2;
        let add_imaginary_value: i32 = self.imaginary_number1 + self.imaginary_number2;
        format!("{}+i{}", add_real_values, add_imaginary_value)
    }

    // function subtraction performing subtraction operation on complex numbers
    pub fn subtraction(&self) -> String {
        let sub_real_values: i32 = self.real_number1 + self.real_number2;
        let sub_imaginary_values: i32 = self.imaginary_number1 + self.imaginary_number2;
        format!("{}+i{}", sub_real_values, sub_imaginary_values)
    }

    // function multiplication performing multiplication operation on complex numbers
    pub fn multiplication(&self) -> String {
        let mul_real_value: i32 = (self.real_number1 * self.real_number2)
            - (self.imaginary_number1 * self.imaginary_number2);
        let mul_imaginary_value: i32 = (self.real_number1 * self.imaginary_number2)
            + (self.imaginary_number1 * self.real_number2);
        format!("{}+i{}", mul_real_value, mul_imaginary_value)
    }
}
