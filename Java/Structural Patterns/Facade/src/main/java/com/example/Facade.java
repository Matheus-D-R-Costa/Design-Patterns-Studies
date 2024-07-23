package com.example;

import com.example.subsystem.cep.CepApi;
import com.example.subsystem.crm.CrmService;

public class Facade {

    public void migrateClient(String name, String cep) {
        String state = CepApi.getInstance().getState(cep);
        String city = CepApi.getInstance().getCity(cep);

        CrmService.saveClient(name, cep, state, city);
    }
}
