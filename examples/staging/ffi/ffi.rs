use std::fmt;

// this extern block links to the libm library
#[link(name = "m")]
extern {
    // this is a foreign function
    // that computes the square root of a single precision complex number
    fn csqrtf(z: Complex) -> Complex;
}

fn main() {
    // z = -1 + 0i
    let z = Complex { re: -1., im: 0. };

    // calling a foreign function is an unsafe operation
    let z_sqrt = unsafe {
        csqrtf(z)
    };

    println!("the square root of {} is {}", z, z_sqrt);
}

// Minimal implementation of single precision complex numbers
#[repr(C)]
struct Complex {
    re: f32,
    im: f32
}

/* string conversions */
impl fmt::Show for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.im < 0. {
            write!(f, "{}-{}i", self.re, -self.im)
        } else {
            write!(f, "{}+{}i", self.re, self.im)
        }
    }
}