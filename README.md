

# BOBOBIB
          
BOuncing Balls On a BIg Ball


 -------------------------------------------------------------

## Author:

  Joris Rehm, 
  login on GitHub joris-r, 
  Email: joris.rehm@wakusei.fr

## Goal

Make spheres moving, bouncing and colliding on the
surface of a planet. I will also try this new shining
thing called Entity Component System (ECS). The main
goal is in fact to learn Rust.

 -------------------------------------------------------------
 
## What I did so far

  - a trivial way of recording (what I'm calling)
    functional components using a HasMap
  - a trivial manager for the entities
  - trivial 3D vector (no operations yet)
  - make generic the functional table
  - textual log display using Rust log system
    (to be completed for the new functions)
  - velocity (with no acceleration)
  - use (fake) gravity force to compute acceleration
  - use acceleration to compute velocity
  - substraction of vectors
  - length of a vector
  - normalization of a vector

## What's next
  - compute distance between two vector
  - compute gravity force vector from position
  - the planet definition
  - detect collision with the planet and reaction
  - create a library for the ECS
  - graphical display
  - render a plain circle
  - try multi-thread processing

 -------------------------------------------------------------

## TODO introduce things

OK let's try defining an entity component system (ECS).
See:
  - http://en.wikipedia.org/wiki/Entity_component_system
  - TODO retrouver un bon tuto/article

The components will we stored in tables a little bit like in
relational database.
We will distinguish two kind of components:
  - "functional component" when there is a 0..1 relation between an entity
    and its component.
  - "relational component" when there is any number of component (of
    a certain kind) associated to one entity.
