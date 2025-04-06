pub mod behaviours;
pub mod boxer;

#[cfg(test)]
mod tests {
    use super::behaviours::{AggressiveBehaviour, WeakBehaviour};
    use super::boxer::Boxer;

    #[test]
    fn test_boxer_with_aggressive_behaviour() {
        let aggressive = AggressiveBehaviour;
        let boxer = Boxer::new(Box::new(aggressive));
        boxer.jab();
        boxer.cross();
    }

    #[test]
    fn test_boxer_with_weak_behaviour() {
        let weak = WeakBehaviour;
        let boxer = Boxer::new(Box::new(weak));
        boxer.jab();
        boxer.cross();
    }

    #[test]
    fn test_boxer_changing_behaviour() {
        let mut boxer = Boxer::new(Box::new(AggressiveBehaviour));
        boxer.cross();
        boxer.jab();

        boxer.change_behaviour(Box::new(WeakBehaviour));
        boxer.jab();
        boxer.cross();
    }
}
