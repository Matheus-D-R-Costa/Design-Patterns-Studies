package com.example;

import com.example.models.Dialog;
import com.example.models.WebDialog;
import com.example.models.WindowsDialog;

public class Main {
    public static void main(String[] args) {
        Dialog dialog;

        dialog = new WindowsDialog();
        dialog.render();

        dialog = new WebDialog();
        dialog.render();
    }
}