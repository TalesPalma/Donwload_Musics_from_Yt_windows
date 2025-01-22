use std::{fs, path::Path, process::Command};



pub async fn get_music_from_yt(url : &str){
    let pasta_saida = "./Music"; // Substitua pela pasta de saída desejada

    // Cria o diretório de saída, se não existir
    if !Path::new(pasta_saida).exists() {
        fs::create_dir_all(pasta_saida).expect("Falha ao criar a pasta de saída");
    }

    // Passo 1: Baixar e converter o áudio diretamente para MP3 usando yt-dlp
    let status = Command::new("./yt-dlp.exe")
        .args([
            url,
            "--extract-audio",             // Extrair apenas o áudio
            "--audio-format", "mp3",       // Converter diretamente para MP3
            "-o", &format!("{}/%(title)s.%(ext)s", pasta_saida), // Salvar diretamente na pasta de saída
        ])
        .status()
        .expect("Falha ao executar yt-dlp.exe");

    if !status.success() {
        eprintln!("Falha ao baixar ou converter o áudio");
        return; // retorne um erro aqui, se necessário
    }

    println!("Músicas baixadas e salvas em {:?}", pasta_saida);
}