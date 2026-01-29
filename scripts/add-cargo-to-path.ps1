# Aggiunge C:\_Main\_app\.cargo\bin al PATH utente in modo permanente (solo se non gia' presente).
# Esegui in PowerShell: .\scripts\add-cargo-to-path.ps1
# Poi riavvia Cursor o apri un nuovo terminale.

$CargoBin = "C:\_Main\_app\.cargo\bin"
$UserPath = [Environment]::GetEnvironmentVariable("Path", "User")

if ($UserPath -notlike "*$CargoBin*") {
    [Environment]::SetEnvironmentVariable("Path", "$UserPath;$CargoBin", "User")
    Write-Host "Aggiunto al PATH utente: $CargoBin"
    Write-Host "Riavvia Cursor o apri un nuovo terminale per usare 'cargo'."
} else {
    Write-Host "Il path era gia' presente nel PATH utente."
}
