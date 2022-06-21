use bevy::prelude::*;

pub type Demando<'world, 'state, Q, F = ()> = Query<'world, 'state, Q, F>;
pub type Ento = Entity;
pub type Parenco = Parent;
pub type Transformo = Transform;
pub type Valoraĵoj<T> = Assets<T>;
pub type Maŝo = Mesh;

//It is a joke XD
