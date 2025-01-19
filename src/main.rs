use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    let url = "https://www.youtube.com/watch?v=j4_gC_E_Yu4&list=PLdKWRMG0YzNsmNhOCKK2jrOY_zd-Mru7a&ab_channel=SIKKOO%EC%8B%9D%EA%B5%AC"; // Substitua pela URL real
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
        return;
    }

    println!("Músicas baixadas e salvas em {:?}", pasta_saida);
}
