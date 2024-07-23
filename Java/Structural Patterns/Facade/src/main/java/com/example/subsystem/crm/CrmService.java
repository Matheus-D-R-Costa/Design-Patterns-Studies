package com.example.subsystem.crm;

public class CrmService {

    private CrmService() {}

    public static void saveClient(String name, String cep, String state, String city) {
        System.out.println("Cliente salvo no sistema de CRM:");
        System.out.println(name);
        System.out.println(cep);
        System.out.println(state);
        System.out.println(city);
    }
}
