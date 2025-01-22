use tokio::task;
use druid::{widget::{Button, Flex, Label, Padding, TextBox}, AppLauncher,  Data, Lens,  PlatformError, Widget, WidgetExt, WindowDesc};

use crate::service;

#[derive(Clone, Data, Lens)]
struct AppState {
    url: String,
}


pub fn run() -> Result<(), PlatformError> {
    // Define a descrição da janela
    let main_window = WindowDesc::new(ui_builder())
        .title("Meu App Rust GUI")
        .window_size((400.0, 200.0));



    // Dados iniciais (usados para gerenciar o estado do app)
    let initial_data = AppState{
        url: String::new(),
    };

    // Inicia o aplicativo
    AppLauncher::with_window(main_window)
        .launch(initial_data)?;




    Ok(())
}

// Constrói a interface gráfica
fn ui_builder() -> impl Widget<AppState> {


    let label = Label::new("Digite a URL do vídeo do YouTube:");

    let textbox = TextBox::new()
    .with_placeholder("Url do vídeo")
    .fix_width(300.0)
    .lens(AppState::url);

    let button = Button::new("Baixar")
    .on_click(|_ctx, data: &mut AppState, _env| {
        let url = data.url.clone();
        task::spawn(async move{
            service::services::get_music_from_yt(&url).await;
        });
        
    });



    Flex::column()
    .with_child(Padding::new(10.0, label))
    .with_child(Padding::new(10.0, textbox))
    .with_child(Padding::new(10.0, button))


}