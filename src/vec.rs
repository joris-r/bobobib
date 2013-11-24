

/* We're using double size float */
pub type MyFloat = f64;

/* Simple vector type for 3D position */
#[deriving(Eq,Clone)]
pub struct Vec(MyFloat, MyFloat, MyFloat);

impl ToStr for Vec {
  fn to_str(&self) -> ~str {
    let &Vec(x,y,z) = self;
    format!("({},{},{})", x, y, z)
  }
}

impl Add<Vec, Vec> for Vec {
  fn add(&self, rhs: &Vec) -> Vec {
    let &Vec(ax, ay, az) = self;
    let &Vec(bx, by, bz) = rhs;
    Vec(bx+ax, by+ay, bz+az)
  }
}

impl Sub<Vec, Vec> for Vec {
  fn sub(&self, rhs: &Vec) -> Vec{
    let &Vec(ax, ay, az) = self;
    let &Vec(bx, by, bz) = rhs;
    Vec(ax-bx, ay-by, az-bz)
  }
}

#[test]
fn test_add_vector(){
  let a = Vec(1., 2., 3.);
  let b = Vec(3., 2., 1.);
  let c = Vec(4., 4., 4.);
  assert!( a + b == c );
}

#[test]
fn test_sub_vector(){
  let a = Vec(1., 2., 3.);
  let b = Vec(3., 2., 1.);
  let c = Vec(4., 4., 4.);
  assert!( c - b == a);
}

impl Vec{
  // multiply by a scalar
  pub fn scale(&self, a: MyFloat) -> Vec {
    let Vec(sx, sy, sz) = *self;
    Vec(sx*a, sy*a, sz*a)
  }

  pub fn length(&self) -> MyFloat {
    let Vec(sx, sy, sz) = *self;
    (sx*sx + sy*sy + sz*sz).sqrt()
  }
  
  pub fn normalize(&self) -> Vec {
    let l = self.length();
    self.scale(1. / l)
  }
  
  pub fn is_nan(&self) -> bool {
    let Vec(sx, sy, sz) = *self;
    sx.is_nan() ||
    sy.is_nan() ||
    sz.is_nan()
  }
  
}

#[test]
fn test_scale_vector(){
  let a = Vec(1., 2., 3.);
  let b : MyFloat = 10.;
  let c = Vec(10., 20., 30.);
  assert!( a.scale(b) == c );
}

#[test]
fn test_length_vector(){
  let a = Vec(2., 2., 1.);
  assert!( a.length() == 3. );
}

#[test]
fn test_norm_vector(){
  let a = Vec(0., 2., 0.);
  let b = Vec(0., 1., 0.);
  assert!( a.normalize() == b );
}
