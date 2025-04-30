mod cashier;
mod doctor;
mod medical;
mod reception;

pub use cashier::Cashier;
pub use doctor::Doctor;
pub use medical::Medical;
pub use reception::Reception;

use crate::patient::Patient;

pub trait Department {
    fn execute(&mut self, patient: &mut Patient) {
        self.handle(patient);
        self.next_handle(patient);
    }

    // Processa o paciente no departamento atual
    fn handle(&mut self, patient: &mut Patient);
    
    // Acessa o próximo departamento da cadeia
    fn next(&mut self) -> &mut Option<Box<dyn Department>>;
    
    // Passa o paciente para o próximo departamento, se existir
    fn next_handle(&mut self, patient: &mut Patient) {
        if let Some(next) = &mut self.next() {
            next.execute(patient);
        }
    }
    
    // Métodos específicos para cada departamento
    fn register_patient(&self, _patient: &mut Patient) {}
    fn check_patient(&self, _patient: &mut Patient) {}
    fn give_medicine_to_patient(&self, _patient: &mut Patient) {}
    fn get_money_from_patient(&self, _patient: &mut Patient) {}
}

/// Helps to wrap an object into a Boxed
pub fn into_next(department: impl Department + Sized + 'static) -> Option<Box<dyn Department>> {
    Some(Box::new(department))
}


