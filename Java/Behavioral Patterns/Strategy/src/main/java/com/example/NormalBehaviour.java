package com.example;

public class NormalBehaviour implements Behaviour{

    @Override
    public void jab() {
        System.out.println("JAB...");
    }

    @Override
    public void cross() {
        System.out.println("DIRETO...");
    }
}
