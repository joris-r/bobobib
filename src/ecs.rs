

struct FunctionalTable<C> {
  priv name : ~str,
  priv map : ~::std::hashmap::HashMap<Entity, C>
}

impl<C : ToStr + Clone> FunctionalTable<C> {

  pub fn new(name : ~str) -> FunctionalTable<C>{
    info!("create_table name={}",name);
    FunctionalTable{
      name : name,
      map : ~::std::hashmap::HashMap::new()
    }
  }
  
  pub fn set(&mut self, e : Entity, v : C) {
    info!("set id={} cmpt={} val={}", e, self.name, v.to_str() );
    self.map.swap(e,v);
  }
  
  /* fail if the entity e does not have a component */
  pub fn get<'a>(&'a self, e : Entity) -> &'a C {
    self.map.get(&e)
  }

  /* TODO create an own iterator type (or re-use some trait) */
  pub fn iter<'a>(&'a self) -> ::std::hashmap::HashMapIterator<'a, Entity, C> {
    self.map.iter()
  }
  
  pub fn apply(&mut self, f : &fn(Entity, &C) -> C) {
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
  let mut table : FunctionalTable<int> = FunctionalTable::new(~"test");
  let val = 1;
  let entity = 42;
  table.set(entity,val);
  assert!( val == *table.get(entity) );
}


/* We need a type for the entities, which is just an identifier. */
pub type Entity = uint;