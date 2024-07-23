package com.example.subsystem.cep;

public class CepApi {

    private static CepApi instance = new CepApi();

    private CepApi() {}

    public static CepApi getInstance() {
        return instance;
    }

    public String getCity(String cep) {
        return "Palhoça";
    }

    public String getState(String cep) {
        return "Santa Catarina";
    }

}
