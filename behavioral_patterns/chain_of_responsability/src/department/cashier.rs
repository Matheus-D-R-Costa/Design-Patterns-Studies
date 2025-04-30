use super::{Department, Patient};

#[derive(Default)]
pub struct Cashier {
    next: Option<Box<dyn Department>>,
}

impl Department for Cashier {
    
    fn handle(&mut self, patient: &mut Patient) {
        match patient.payment_done {
            true => println!("Payment done"),
            false => self.get_money_from_patient(patient),
        }
    }
   
    fn get_money_from_patient(&self, patient: &mut Patient) {
        println!("Cashier: Getting money from patient");
        patient.payment_done = true;
    }

    fn next(&mut self) -> &mut Option<Box<dyn Department>> {
        &mut self.next
    }
}