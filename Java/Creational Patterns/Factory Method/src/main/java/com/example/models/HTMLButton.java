package com.example.models;

public class HTMLButton implements Button{
    @Override
    public void render() {
        System.out.println("Renderizando um botão no estilo HTML.");
    }

    @Override
    public void onClick() {
        System.out.println("Clique no botão HTML!");
    }
}
