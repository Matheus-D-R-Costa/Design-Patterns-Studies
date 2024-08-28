package com.example.models;

public class WebDialog extends Dialog{

    @Override
    public Button createButton() {
        return new HTMLButton();
    }

}
