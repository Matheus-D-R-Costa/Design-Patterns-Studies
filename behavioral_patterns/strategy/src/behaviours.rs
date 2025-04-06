#![allow(dead_code)]

pub trait Behaviour {
    fn jab(&self);
    fn cross(&self);
}

pub struct AggressiveBehaviour;
impl Behaviour for AggressiveBehaviour {
    fn jab(&self) {
        println!("Jab Agressivo")
    }

    fn cross(&self) {
        println!("Cross Agressivo")
    }
}

pub struct WeakBehaviour;
impl Behaviour for WeakBehaviour {
    fn jab(&self) {
        println!("Jab fraco")
    }

    fn cross(&self) {
        println!("Cross Fraco")
    }
}
