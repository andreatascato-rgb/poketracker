# Reinstalla Rust/cargo nel profilo utente (C:\Users\vndry\.cargo e .rustup).
# Prima rimuovi C:\_Main\_app\.cargo\bin dal PATH, poi scarica e lancia rustup-init.
# Dopo: elimina manualmente C:\_Main\_app\.cargo e C:\_Main\_app\.rustup (se esistono).

$PathToRemove = "C:\_Main\_app\.cargo\bin"
$userPath = [Environment]::GetEnvironmentVariable("Path", "User")
$entries = $userPath -split ';' | Where-Object { $_.Trim() -ne $PathToRemove }
$newPath = ($entries | Where-Object { $_.Trim() -ne '' }) -join ';'
[Environment]::SetEnvironmentVariable("Path", $newPath, "User")
Write-Host "Rimosso dal PATH utente: $PathToRemove"

# Assicurati che la cartella user .cargo\bin sia nel PATH (di solito gia' c'e')
$cargoUser = "$env:USERPROFILE\.cargo\bin"
if ($newPath -notlike "*$cargoUser*") {
    [Environment]::SetEnvironmentVariable("Path", "$newPath;$cargoUser", "User")
    Write-Host "Aggiunto al PATH utente: $cargoUser"
}

# Rimuovi vecchie cartelle rotte nel profilo utente (opzionale: reinstallazione pulita)
$rustupHome = "$env:USERPROFILE\.rustup"
$cargoHome = "$env:USERPROFILE\.cargo"
if (Test-Path $rustupHome) {
    Write-Host "Rimuovo $rustupHome (reinstallazione pulita)..."
    Remove-Item -Recurse -Force $rustupHome -ErrorAction SilentlyContinue
}
if (Test-Path $cargoHome) {
    Write-Host "Rimuovo $cargoHome (reinstallazione pulita)..."
    Remove-Item -Recurse -Force $cargoHome -ErrorAction SilentlyContinue
}

# Scarica rustup-init
$rustupUrl = "https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe"
$rustupExe = "$env:TEMP\rustup-init.exe"
Write-Host "Scarico rustup-init..."
Invoke-WebRequest -Uri $rustupUrl -OutFile $rustupExe -UseBasicParsing

Write-Host "Eseguo rustup-init (opzioni default, -y)..."
& $rustupExe -y

Write-Host "Fatto. Riavvia Cursor o apri un nuovo terminale e prova: cargo --version"
Write-Host "Poi elimina manualmente le cartelle da C:\_Main\_app : .cargo e .rustup (se presenti)."
