package com.example;

public class Test {
    public static void main(String[] args) {

        // Testes relacionados ao Design Pattern Singleton:

        LazySingleton lazy = LazySingleton.getInstance();
        System.out.println(lazy);
        lazy = LazySingleton.getInstance();
        System.out.println(lazy);
        System.out.println();

        EagerSingleton eager = EagerSingleton.getInstance();
        System.out.println(eager);
        eager = EagerSingleton.getInstance();
        System.out.println(eager);
        System.out.println();

        LazyHolderSingleton LazyHolder = LazyHolderSingleton.getInstance();
        System.out.println(LazyHolder);
        LazyHolder = LazyHolderSingleton.getInstance();
        System.out.println(LazyHolder);
        System.out.println();

    }
}
