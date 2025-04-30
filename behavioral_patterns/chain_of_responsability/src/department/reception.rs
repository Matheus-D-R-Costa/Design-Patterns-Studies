use super::{into_next, Department, Patient};

#[derive(Default)]
pub struct Reception {
    next: Option<Box<dyn Department>>,
}

impl Reception {
    pub fn new(next: impl Department + 'static) -> Self {
        Self {
            next: into_next(next),
        }
    }
}

impl Department for Reception {
    fn handle(&mut self, patient: &mut Patient) {
        match patient.registration_done {
            true => println!("Patient registration is already done"),
            false => self.register_patient(patient),
        }
    }

    fn register_patient(&self, patient: &mut Patient) {
        println!("Reception registering a patient {}", patient.name);
        patient.registration_done = true;
    }

    fn next(&mut self) -> &mut Option<Box<dyn Department>> {
        &mut self.next
    }
}