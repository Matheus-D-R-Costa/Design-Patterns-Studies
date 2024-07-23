package com.example;

public class Test {

    public static void main(String[] args) {

        // strategy

        Behaviour weak = new WeakBehaviour();
        Behaviour normal = new NormalBehaviour();
        Behaviour strong = new StrongBehaviour();

        Boxer boxer = new Boxer();

        boxer.setBehaviour(normal);
        boxer.jab();
        boxer.cross();

        boxer.setBehaviour(weak);
        boxer.jab();
        boxer.jab();

        boxer.setBehaviour(strong);
        boxer.cross();
        boxer.cross();

    }
}
