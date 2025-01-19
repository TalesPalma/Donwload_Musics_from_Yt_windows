use druid::{widget::{Label, Padding}, AppLauncher, LocalizedString, PlatformError, Widget, WindowDesc};

pub fn init() -> Result<(), PlatformError> {
    // Define a descrição da janela
    let main_window = WindowDesc::new(ui_builder())
        .title("Meu App Rust GUI")
        .window_size((400.0, 200.0));

    // Dados iniciais (usados para gerenciar o estado do app)
    let initial_data = ();

    // Inicia o aplicativo
    AppLauncher::with_window(main_window)
        .launch(initial_data)?;

    Ok(())
}

// Constrói a interface gráfica
fn ui_builder() -> impl Widget<()> {
    // Um rótulo simples como exemplo
    let label = Label::new(LocalizedString::new("Hello, Rust!"));
    Padding::new(10.0, label)
}