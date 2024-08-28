package com.example.models;

public abstract class Dialog {

    public void render() {
        Button button1 = createButton();
        button1.onClick();
        button1.render();
    }

    public abstract Button createButton();

}
