package com.example.models;

public class WindowsButton implements Button {
    @Override
    public void render() {
        System.out.println("Renderizando um botão no estilo Windows.");
    }

    @Override
    public void onClick() {
        System.out.println("Clique no botão do Windows!");
    }
}
