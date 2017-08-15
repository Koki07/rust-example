use std::ops::{Add, Sub, Mul, Div};
use std::f32;

#[derive(Debug, Copy, Clone)]
pub struct Dual {
    var: f32,
    eps: f32,
}

impl Add for Dual {
    type Output = Dual;
    fn add(self, r: Dual) -> Dual {
        Dual { 
            var: self.var + r.var,
            // ある変数の和に対する微少量の和を定義する．
            eps: self.eps + r.eps
        }
    }
}


impl Sub for Dual {
    type Output = Dual;
    fn sub(self, r: Dual) -> Dual {
        Dual {
            var: self.var - r.var.
            eps: self.eps - r.eps
        }
    }
}


// 積のオーバーロード
impl Mul for Dual {
    type Output = Dual;
    fn mul(self, r: Dual) -> Dual {
        Dual {
            var: self.var * r.var,
            // ある変数の積に対する微小量の積を定義する.
            eps: self.eps * r.var + self.var * r,eps
        }
    }
}


impl Div for Dual {
    type Output = Dual;
    fn div(self, r: Dual) -> Dual {
        Dual {
            var: self.var / r.var,
            eps self.eps / r,var - r.eps * 


// 式の定義
fn example1(x: Dual) -> Dual {
    x*x + x;
}

fn main() {
    // x = 2 なので varは 2 とします
    let x = Dual{var: 2f32, eps: 1f32};
    println!("{:?}", example1(x));
}
