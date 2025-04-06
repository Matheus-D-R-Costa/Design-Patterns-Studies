#![allow(dead_code)]

use std::path::Path;

mod observer;
mod editor;

fn save_listener(file_path: (&Path, &str)) {
    let email = "admin@example.com".to_string();
    println!("Email to {}: Save file {:?}", email, file_path);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    use std::sync::Mutex;

    // Variáveis estáticas para monitorar se os listeners foram chamados.
    static LOAD_CALLED: Mutex<bool> = Mutex::new(false);
    static SAVE_CALLED: Mutex<bool> = Mutex::new(false);

    // Listener para o evento de Load que altera o flag.
    fn test_load_listener(_dir: &Path, _filename: &str) {
        let mut flag = LOAD_CALLED.lock().unwrap();
        *flag = true;
        println!("Load listener called");
    }

    // Listener para o evento de Save que altera o flag.
    fn test_save_listener(_dir: &Path, _filename: &str) {
        let mut flag = SAVE_CALLED.lock().unwrap();
        *flag = true;
        println!("Save listener called");
    }

    #[test]
    fn test_editor_load_event() {
        // Cria um Editor com valores dummy.
        let dummy_path = Path::new("dummy");
        let mut editor = editor::Editor {
            publisher: observer::Publisher::default(),
            file_path: (dummy_path, "dummy"),
        };
        // Inscreve o listener de load.
        editor.events().subscribe(observer::Event::Load, test_load_listener);

        // Garante que o flag comece false.
        *LOAD_CALLED.lock().unwrap() = false;

        // Chama o método load.
        let test_path = Path::new("dir");
        editor.load((test_path, "file.txt"));

        // Verifica se o listener foi acionado.
        assert!(*LOAD_CALLED.lock().unwrap());
    }

    #[test]
    fn test_editor_save_event() {
        // Cria um Editor com valores dummy.
        let dummy_path = Path::new("dummy");
        let mut editor = editor::Editor {
            publisher: observer::Publisher::default(),
            file_path: (dummy_path, "dummy"),
        };
        // Inscreve o listener de save.
        editor.events().subscribe(observer::Event::Save, test_save_listener);

        // Garante que o flag comece false.
        *SAVE_CALLED.lock().unwrap() = false;

        // Define o caminho do arquivo e dispara os eventos.
        let test_path = Path::new("dir");
        editor.load((test_path, "file.txt")); // Usa load para atualizar o file_path.
        editor.save(); // Dispara o evento Save.

        // Verifica se o listener de save foi acionado.
        assert!(*SAVE_CALLED.lock().unwrap());
    }

    #[test]
    fn test_unsubscribe() {
        // Cria um Editor com valores dummy.
        let dummy_path = Path::new("dummy");
        let mut editor = editor::Editor {
            publisher: observer::Publisher::default(),
            file_path: (dummy_path, "dummy"),
        };
        // Inscreve o listener de load.
        editor.events().subscribe(observer::Event::Load, test_load_listener);

        // Desinscreve o listener.
        editor.events().unsubscribe(observer::Event::Load, test_load_listener);

        // Garante que o flag comece false.
        *LOAD_CALLED.lock().unwrap() = false;

        // Chama load: não deve disparar o listener, pois foi desinscrito.
        let test_path = Path::new("dir");
        editor.load((test_path, "file.txt"));

        // Verifica que o listener não foi acionado.
        assert!(!*LOAD_CALLED.lock().unwrap());
    }
}
