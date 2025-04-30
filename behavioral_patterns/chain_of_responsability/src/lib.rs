mod department;
mod patient;

use department::{Cashier, Department, Doctor, Medical, Reception};
use patient::Patient;

/// Demonstração do padrão Chain of Responsibility para um paciente em um hospital
pub fn process_patient(patient_name: &'static str) -> Patient {
    // Cria a cadeia de responsabilidade (da última para a primeira)
    let cashier = Cashier::default();
    let medical = Medical::new(cashier);
    let doctor = Doctor::new(medical);
    let mut reception = Reception::new(doctor);
    
    // Cria um paciente
    let mut patient = Patient {
        name: patient_name,
        ..Default::default()
    };
    
    // Inicia o processo na recepção
    reception.execute(&mut patient);
    
    patient
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::department::{Cashier, Department, Doctor, Medical, Reception};
    use crate::patient::Patient;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    
    #[test]
    fn test_process_patient_demo() {
        let patient = process_patient("Paciente Demo");
        assert!(patient.registration_done);
        assert!(patient.doctor_check_up_done);
        assert!(patient.medicine_done);
        assert!(patient.payment_done);
    }

    #[test]
    fn test_full_hospital_chain() {
        // Criar a cadeia de responsabilidade em ordem inversa
        let cashier = Cashier::default();
        let medical = Medical::new(cashier);
        let doctor = Doctor::new(medical);
        let mut reception = Reception::new(doctor);

        // Criar um paciente
        let mut patient = Patient {
            name: "João",
            ..Default::default()
        };

        // Iniciar o processamento na recepção
        reception.execute(&mut patient);

        // Verificar se todas as etapas foram concluídas
        assert!(patient.registration_done, "Registro não foi concluído");
        assert!(patient.doctor_check_up_done, "Consulta médica não foi realizada");
        assert!(patient.medicine_done, "Medicamentos não foram entregues");
        assert!(patient.payment_done, "Pagamento não foi efetuado");
    }

    #[test]
    fn test_patient_with_partial_process_done() {
        // Criar a cadeia de responsabilidade em ordem inversa
        let cashier = Cashier::default();
        let medical = Medical::new(cashier);
        let doctor = Doctor::new(medical);
        let mut reception = Reception::new(doctor);

        // Criar um paciente que já fez o registro
        let mut patient = Patient {
            name: "Maria",
            registration_done: true,
            ..Default::default()
        };

        // Iniciar o processamento na recepção
        reception.execute(&mut patient);

        // Verificar se as etapas restantes foram concluídas
        assert!(patient.registration_done, "Registro não foi mantido");
        assert!(patient.doctor_check_up_done, "Consulta médica não foi realizada");
        assert!(patient.medicine_done, "Medicamentos não foram entregues");
        assert!(patient.payment_done, "Pagamento não foi efetuado");
    }

    #[test]
    fn test_individual_department_functionality() {
        // Testar cada departamento individualmente
        
        // Recepção
        let mut reception = Reception::default();
        let mut patient = Patient {
            name: "Pedro",
            ..Default::default()
        };
        reception.handle(&mut patient);
        assert!(patient.registration_done, "Recepção não registrou o paciente");
        
        // Médico
        let mut doctor = Doctor::new(Cashier::default());
        doctor.handle(&mut patient);
        assert!(patient.doctor_check_up_done, "Médico não consultou o paciente");
        
        // Medicação
        let mut medical = Medical::new(Cashier::default());
        medical.handle(&mut patient);
        assert!(patient.medicine_done, "Medicação não foi entregue ao paciente");
        
        // Caixa
        let mut cashier = Cashier::default();
        cashier.handle(&mut patient);
        assert!(patient.payment_done, "Pagamento não foi processado");
    }
    
    #[test]
    fn test_skip_completed_steps() {
        // Verificar se departamentos pulam etapas já concluídas
        let mut patient = Patient {
            name: "Ana",
            registration_done: true,
            doctor_check_up_done: true,
            ..Default::default()
        };
        
        // Apenas os próximos passos devem ser executados
        let cashier = Cashier::default();
        let medical = Medical::new(cashier);
        let doctor = Doctor::new(medical);
        let mut reception = Reception::new(doctor);
        
        reception.execute(&mut patient);
        
        assert!(patient.medicine_done, "Medicação não foi entregue");
        assert!(patient.payment_done, "Pagamento não foi processado");
    }
}
