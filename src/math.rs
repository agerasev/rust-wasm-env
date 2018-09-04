extern {
    fn js_math_exp(x: f64) -> f64;
    fn js_math_pow(x: f64, y: f64) -> f64;
    fn js_math_sin(x: f64) -> f64;
    fn js_math_cos(x: f64) -> f64;
    fn js_math_tan(x: f64) -> f64;
    fn js_math_sinh(x: f64) -> f64;
    fn js_math_cosh(x: f64) -> f64;
    fn js_math_tanh(x: f64) -> f64;
}

pub fn exp(x: f64) -> f64 {
	unsafe { js_math_exp(x) }
}
pub fn pow(x: f64, y: f64) -> f64 {
	unsafe { js_math_pow(x, y) }
}
pub fn sin(x: f64) -> f64 {
	unsafe { js_math_sin(x) }
}
pub fn cos(x: f64) -> f64 {
	unsafe { js_math_cos(x) }
}
pub fn tan(x: f64) -> f64 {
	unsafe { js_math_tan(x) }
}
pub fn sinh(x: f64) -> f64 {
	unsafe { js_math_sinh(x) }
}
pub fn cosh(x: f64) -> f64 {
	unsafe { js_math_cosh(x) }
}
pub fn tanh(x: f64) -> f64 {
	unsafe { js_math_tanh(x) }
}
