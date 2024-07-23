package com.example;

public class LazyHolderSingleton {

    private LazyHolderSingleton() {}

    private static class InstanceHolder {
        private static final LazyHolderSingleton INSTANCE = new LazyHolderSingleton();
    }

    public static LazyHolderSingleton getInstance() {
        return InstanceHolder.INSTANCE;
    }
}
