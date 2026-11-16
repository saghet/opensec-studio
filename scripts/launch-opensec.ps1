$ErrorActionPreference = "Stop"
$RepoDir = "c:\Users\saghe\Documents\opensec-studio"
Set-Location $RepoDir

try {
    if (-not (Test-Path ".env")) {
        Copy-Item ".env.example" ".env"
        Write-Host "[OpenSec] .env creado desde .env.example." -ForegroundColor Yellow
    }

    Write-Host "[OpenSec] Iniciando servicios Docker (Control Plane + Wireless Security Center)..." -ForegroundColor Cyan
    docker compose -f docker-compose.yml -f docker-compose.dev.yml up -d --build
    if ($LASTEXITCODE -ne 0) {
        throw "docker compose up falló con código $LASTEXITCODE"
    }

    $portLine = Select-String -Path ".env" -Pattern "^OPENSEC_WEB_PORT=(\d+)" | Select-Object -First 1
    $port = if ($portLine) { $portLine.Matches[0].Groups[1].Value } else { "8088" }
    $healthUrl = "http://localhost:$port/healthz"
    $appUrl = "http://localhost:$port"

    Write-Host "[OpenSec] Esperando a que Wireless Security Center esté listo en $appUrl..." -ForegroundColor Cyan
    $ready = $false
    for ($i = 0; $i -lt 45; $i++) {
        try {
            $resp = Invoke-WebRequest -UseBasicParsing -Uri $healthUrl -TimeoutSec 2
            if ($resp.StatusCode -eq 200) {
                $ready = $true
                break
            }
        } catch {
            Start-Sleep -Seconds 2
        }
    }

    if (-not $ready) {
        throw "Timeout esperando a $healthUrl. Revisa 'docker compose logs opensec-web opensec-api'."
    }

    Write-Host "[OpenSec] Abriendo Wireless Security Center en $appUrl" -ForegroundColor Green
    Start-Process $appUrl
} catch {
    Write-Host "[ERROR] $_" -ForegroundColor Red
    Read-Host "Presiona Enter para cerrar"
    exit 1
}
