package com.example;

public class Boxer {

    private Behaviour behaviour;


    public void setBehaviour(Behaviour behaviour) {
        this.behaviour = behaviour;
    }

    public void jab() {
        behaviour.jab();
    }

    public void cross() {
        behaviour.cross();
    }
}
