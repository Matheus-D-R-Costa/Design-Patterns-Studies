#![allow(dead_code)]

use crate::behaviours::Behaviour;

pub struct Boxer {
    behaviour: Box<dyn Behaviour>,
}

impl Boxer {
    pub fn new(behaviour: Box<dyn Behaviour>) -> Self {
        Boxer { behaviour }
    }

    pub fn change_behaviour(&mut self, behaviour: Box<dyn Behaviour>) {
        self.behaviour = behaviour
    }

    pub fn jab(&self) {
        self.behaviour.jab();
    }

    pub fn cross(&self) {
        self.behaviour.cross();
    }
}
