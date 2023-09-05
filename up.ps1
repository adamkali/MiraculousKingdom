
# Load environment variables from .env file using dotenv
npm install -g dotenv-cli
try {
    dotenv 

    if ((Test-Path -Path ".\Cargo.toml") -and ((Get-Content -Path ".\Cargo.toml" -Raw) -match "miraculous_kingdom")) {
        docker-compose up -d;
        $containerName = "miraculouskingdom-surrealdb-1";  # Replace with the actual container name or ID
        $command = "surreal import --conn http://localhost:8000 --user $DB_USER --pass $DB_PASS --ns $DB_NS --db $DB_DB /migrations.surql";
        docker exec -it $containerName -c "apk add --no-cache sh";
        if ($LASTEXITCODE -ne 0) {
            throw "surreal import failed with exit code: $LASTEXITCODE"
        }
        docker exec -it $containerName sh -c $command;
        if ($LASTEXITCODE -ne 0) {
            throw "surreal import failed with exit code: $LASTEXITCODE"
        }
        $buildLevel = $env:MK_LEVEL
        if ($buildLevel -eq "PROD") {
            $buildOptions = "--release"
        } elseif ($buildLevel -eq "DEV") {
            $buildOptions = ""
        } else {
            throw "Invalid MK_LEVEL value. Use 'PROD' or 'DEV'."
        }
        Invoke-Expression "cargo build $buildOptions"
        if ($LASTEXITCODE -ne 0) {
            throw "Rust compilation failed with exit code: $LASTEXITCODE"
        }
    } else {
        throw "The Cargo.toml file does not exist or does not contain 'miraculous_kingdom'"
    }
} catch {
    Write-Host "Error: $_"
    exit 1  # Exit with a non-zero status code to indicate an error
}


