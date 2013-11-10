/*
                    License stuff
----------------------------------------------------------
Copyright 2013 Joris Rehm

Licensed under the Apache License, Version 2.0.
Licensed under The MIT License.

See joined files "LICENSE-APACHE" and "LICENSE-MIT".
----------------------------------------------------------
*/

extern mod sdl;

/* ----------------------------------------------------------- */

struct FunctionalTable<C> {
  priv name : ~str,
  priv map : ~std::hashmap::HashMap<Entity, C> 
}

impl<C : ToStr + Clone> FunctionalTable<C> {

  fn new(name : ~str) -> FunctionalTable<C>{
    info!("create_table name={}",name);
    FunctionalTable{
      name : name,
      map : ~std::hashmap::HashMap::new()
    }
  }
  
  fn set(&mut self, e : Entity, v : C) {
    info!("set id={} cmpt={} val={}", e, self.name, v.to_str() );
    self.map.swap(e,v);
  }
  
  /* fail if the entity e does not have a component */
  fn get<'a>(&'a self, e : Entity) -> &'a C {
    self.map.get(&e)
  }

  /* TODO create an own iterator type (or re-use some trait) */
  fn iter<'a>(&'a self) -> std::hashmap::HashMapIterator<'a, Entity, C> {
    self.map.iter()
  }
  
  fn apply(&mut self, f : &fn(Entity, &C) -> C) {
    let copy = self.map.clone();
    // TODO do I really need to copy all the map ?
    for (&e,c) in copy.iter() {
      let new_c = f(e, c);
      info!("set id={} cmpt={} val={}", e, self.name, new_c.to_str() );
      self.map.swap(e, new_c);
    }
  }
  
}

#[test]
fn test_FunctionalTable_get_set(){
  let mut table : FunctionalTable<Vec> = FunctionalTable::new(~"test");
  let vec = Vec(12., 56., 3.);
  let e1 = 42;
  table.set(e1,vec);
  assert!( vec == *table.get(e1) );
}


/* ----------------------------------------------------------- */


/* We need a type for the entities, which is just an identifier. */
type Entity = uint;

/* Let's defining a data structure for managing our ECS. */
struct Manager {
  /* There is a data to remember the numbers of created entities. */
  priv entities_numbers : uint
}

impl Manager {
  /* Inializing an empty ECS */ 
  fn new() -> ~Manager {
    ~Manager {
      entities_numbers : 0
    }
  }
  
  /* Creating an new entity is just taking a unused id. */
  fn new_entity(&mut self) -> Entity {
    let res = self.entities_numbers;
    self.entities_numbers = self.entities_numbers + 1;
    info!("create_entity id={}", res);
    res
  }
  
  /* TODO: how to manage the deletion of entities ?
     It depend if we iterate on entity id or on component tables.
     Maybe it's enought to delete the associated components.
     We may use a pool of unused id for recycling (probably not
     very needed) or just search for not associated id.*/

} /* impl Manager */


#[test]
fn test_entity_management() {
  let mut mng = Manager::new();
  let e1 = mng.new_entity();
  assert!( e1 == 0 );
  let e2 = mng.new_entity();
  assert!( e2 == 1 );
}


/* ----------------------------------------------------------- */


/* We're using double size float */
type MyFloat = f64;

/* Simple vector type for 3D position */
#[deriving(Eq,Clone)]
struct Vec(MyFloat, MyFloat, MyFloat);

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
  fn scale(&self, a: MyFloat) -> Vec {
    let Vec(sx, sy, sz) = *self;
    Vec(sx*a, sy*a, sz*a)
  }

  fn length(&self) -> MyFloat {
    let Vec(sx, sy, sz) = *self;
    (sx*sx + sy*sy + sz*sz).sqrt()
  }
  
  fn normalize(&self) -> Vec {
    let l = self.length();
    self.scale(1. / l)
  }
  
  fn is_nan(&self) -> bool {
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

/* ----------------------------------------------------------- */


fn test(){
  let mut mng = Manager::new();
  let mut table_position = FunctionalTable::new(~"position");
  let mut table_velocity = FunctionalTable::new(~"velocity");
  let mut table_acceleration = FunctionalTable::new(~"acceleration");
  let mut table_mass = FunctionalTable::new(~"mass");
  let zero = Vec(0., 0., 0.);
  let vec_1x = Vec(1.,0.,0.);
  let vec_1y = Vec(0.,1.,0.);
  let delta_time = 1./24. as MyFloat;
  let earth_gravity = 9.80665; // in m/s²
  
  // TODO I'm not very happy to use lambda just for
  // accessing the data.
  
  let gravity_force_at = |pos: Vec| {
    let planet_center = Vec(0.,0.,0.);
    
    // the vector from planet center to the pos
    let pos_from_planet = pos - planet_center;
    // TODO and yes it's useless :)
    
    let dist = pos_from_planet.length();
    
    let gravity_force =
      pos_from_planet.normalize(
      ).scale(- earth_gravity
      ).scale(1./(dist*dist));
    
    if gravity_force.is_nan() {
      Vec(0.,0.,0.)
    } else {
      gravity_force
    }
    
  };
  
  // Having an acceleration imply having a velocity!
  // Having an acceleration imply having a mass!
  let compute_acceleration = || {
    let fun = |e, accel: &Vec| {
      // note: accel not used
      let pos = table_position.get(e);
      let force = gravity_force_at(*pos);
      let mass = *table_mass.get(e);
      force.scale(1./mass)
    };
    table_acceleration.apply(fun);
  };
  
  // TODO define an integration function and refactor
  // compute_velocity and compute_position.
  
  // integration of acceleration
  // having a acceleration imply having a velocity
  let compute_velocity = |dt| {
    for x in table_acceleration.iter() {
      let (&e,accel) : (&Entity,&Vec) = x;
      let vel : Vec = *table_velocity.get(e);
      let new_vel = vel + accel.scale(dt);
      table_velocity.set(e, new_vel);
    }
  };
  
  /* Move all the entities with the velocity applied during dt time.
   Having a velocity imply having a position! */
  let compute_position = |dt| {
    for x in table_velocity.iter() {
      let (&e,vel) : (&Entity,&Vec) = x;
      let pos = *table_position.get(e);
      let new_pos = pos + vel.scale(dt);
      table_position.set(e, new_pos);
    }
  };
  
  let entity1 = mng.new_entity();
  table_position.set(entity1, vec_1y.scale(10.) );
  table_velocity.set(entity1, vec_1x.scale(10.) );
  table_acceleration.set(entity1, zero);
  table_mass.set(entity1, 1.);
  
  println("use RUST_LOG=3 to see log");
  for cycle in range(1u, 25) {
    println!("start_cycle {}",cycle);
    compute_acceleration();
    compute_velocity(delta_time);
    compute_position(delta_time);
  }
  
}


#[main]
pub fn main() {
    sdl::init([sdl::InitVideo]);
    sdl::wm::set_caption("Bouncing Balls On a Big Ball", "Bobobib");

    let screen = match sdl::video::set_video_mode
      (800, 600, 32, [sdl::video::HWSurface],[sdl::video::DoubleBuf]) {
        Ok(screen) => screen,
        Err(err) => fail!("Impossible to open screen: {}", err)
    };

    'main : loop {
        'event : loop {
            match sdl::event::poll_event() {
                sdl::event::QuitEvent => break 'main,
                sdl::event::NoEvent => break 'event,
                sdl::event::KeyEvent(k, _, _, _)
                    if k == sdl::event::EscapeKey
                        => break 'main,
                _ => {}
            }
        }
        screen.flip();
    }

    sdl::quit();
}
