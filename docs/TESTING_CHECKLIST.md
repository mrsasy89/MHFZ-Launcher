# ✅ Checklist Testing Completa

**Progetto**: MHFZ-Launcher Linux Porting  
**Scopo**: Verificare ogni modifica PRIMA del commit  
**Approccio**: Test incrementali + regression testing

---

## 📋 Indice

1. [Setup Ambiente di Test](#setup-ambiente-di-test)
2. [Test Pre-Commit Obbligatori](#test-pre-commit-obbligatori)
3. [Test per Step 4 (Wine Launcher)](#test-step-4-wine-launcher)
4. [Test per Step 5 (Server Config)](#test-step-5-server-config)
5. [Test per Step 6 (INI Parser)](#test-step-6-ini-parser)
6. [Regression Testing](#regression-testing)
7. [Performance Testing](#performance-testing)

---

## 🛠️ Setup Ambiente di Test

### Prerequisiti Sistema

```bash
# Verifica versioni
rustc --version   # Deve essere nightly
node --version    # 16+
npm --version     # 8+
wine --version    # 9.0+

# Output attesi:
# rustc 1.75.0-nightly (hash date)
# node v18.x.x (o superiore)
# npm 9.x.x
# wine-9.0
```

### Configurazione Wine

```bash
# Crea Wine prefix se non esiste
export WINEPREFIX="$HOME/Games/MHFZ/pfx"
mkdir -p "$WINEPREFIX"

# Inizializza prefix
WINEARCH=win32 wineboot --init

# Installa dipendenze
winetricks dotnet48 vcrun2019 d3dx9 d3dcompiler_47
winetricks corefonts allfonts

# Verifica configurazione
winecfg  # Dovrebbe aprirsi senza errori
```

### Configurazione Progetto

```bash
# Clona/vai al progetto
cd ~/Progetti/MHFZ-Launcher

# Set nightly toolchain
rustup override set nightly

# Installa dipendenze Node
npm install

# Verifica build iniziale
cargo build --release
```

**✅ Checklist Setup:**
- [ ] Wine installato e funzionante
- [ ] Wine prefix configurato
- [ ] Progetto compila senza errori
- [ ] Dependencies Node installate

---

## 🔴 Test Pre-Commit Obbligatori

**IMPORTANTE**: Eseguire TUTTI questi test prima di ogni commit!

### Test 1: Compilazione

```bash
# Clean build
cargo clean
cargo build --release
```

**Criteri successo:**
- ✅ Compilazione completa senza errori
- ✅ Massimo 5 warning (non critici)
- ✅ Nessun errore di linking

**Tempo atteso:** 3-5 minuti (prima volta), 30s (incrementale)

---

### Test 2: Linter

```bash
# Rust clippy
cargo clippy -- -D warnings

# Format check
cargo fmt -- --check
```

**Criteri successo:**
- ✅ Nessun warning da clippy
- ✅ Codice formattato correttamente

**Se fallisce:**
```bash
# Auto-fix formatting
cargo fmt

# Rivedi warning clippy manualmente
cargo clippy
```

---

### Test 3: Unit Tests (se presenti)

```bash
# Run unit tests
cargo test --lib

# Run integration tests
cargo test --test '*'
```

**Criteri successo:**
- ✅ Tutti i test passano
- ✅ Nessun test ignorato senza motivo

---

### Test 4: Dev Mode Smoke Test

```bash
# Avvia in dev mode
export RUST_LOG=info
export WINEPREFIX="$HOME/Games/MHFZ/pfx"
npm run tauri:dev
```

**Criteri successo:**
- ✅ Launcher si apre entro 10 secondi
- ✅ UI carica completamente
- ✅ Nessun errore in console JavaScript
- ✅ Nessun panic Rust

**Test interattivi:**
1. Apri dropdown server → deve mostrare server
2. Cambia tema (se disponibile) → deve applicarsi
3. Apri Settings → deve caricare
4. Chiudi launcher → deve chiudersi pulito

---

## 🞼 Test Step 4: Wine Launcher

### Setup File di Test

```bash
# Crea struttura game folder di test
mkdir -p ~/Games/MHFZ-Test
touch ~/Games/MHFZ-Test/mhf.exe
touch ~/Games/MHFZ-Test/mhf.ini

# Crea INI minimale
cat > ~/Games/MHFZ-Test/mhf.ini << 'EOF'
[VIDEO]
GRAPHICS_VER=1

[SCREEN]
FULLSCREEN_MODE=0
WINDOW_RESOLUTION_W=1280
WINDOW_RESOLUTION_H=720

[SOUND]
SOUND_VOLUME=100
EOF
```

---

### Test 4.1: Wine Detection

```bash
# Test script standalone
cat > test_wine_detection.sh << 'EOF'
#!/bin/bash

# Test 1: Wine installato
if command -v wine &> /dev/null; then
    echo "✅ wine found: $(wine --version)"
else
    echo "❌ wine NOT found"
    exit 1
fi

# Test 2: Wine64 installato
if command -v wine64 &> /dev/null; then
    echo "✅ wine64 found: $(wine64 --version)"
else
    echo "⚠️ wine64 NOT found (fallback to wine)"
fi

# Test 3: WINEPREFIX configurato
if [ -d "$WINEPREFIX" ]; then
    echo "✅ WINEPREFIX exists: $WINEPREFIX"
else
    echo "❌ WINEPREFIX NOT configured"
    exit 1
fi

echo "✅ All Wine checks passed!"
EOF

chmod +x test_wine_detection.sh
./test_wine_detection.sh
```

**Criteri successo:**
- ✅ Tutti i check passano
- ✅ Wine prefix esiste

---

### Test 4.2: Executable Detection

**Test A: mhf.exe (F5 version)**
```bash
cd ~/Progetti/MHFZ-Launcher
export RUST_LOG=debug
npm run tauri:dev

# Nel launcher:
# 1. Settings → Game Folder → Select ~/Games/MHFZ-Test
# 2. Verifica log:
```

**Log attesi:**
```
[Linux] Found game executable: /home/salvatore/Games/MHFZ-Test/mhf.exe
```

**Test B: mhfo.dll (ZZ SD)**
```bash
# Rename executable
mv ~/Games/MHFZ-Test/mhf.exe ~/Games/MHFZ-Test/mhfo.dll

# Riavvia launcher e verifica log
```

**Log attesi:**
```
[Linux] Found game executable: /home/salvatore/Games/MHFZ-Test/mhfo.dll
```

**Test C: mhfo-hd.dll (ZZ HD)**
```bash
mv ~/Games/MHFZ-Test/mhfo.dll ~/Games/MHFZ-Test/mhfo-hd.dll
# Verifica come sopra
```

---

### Test 4.3: Process Spawning

**Test manuale completo:**

```bash
# 1. Avvia launcher
export RUST_LOG=info
export WINEPREFIX="$HOME/Games/MHFZ/pfx"
npm run tauri:dev

# 2. Login al server Erupe
# Username: testuser
# Password: testpass

# 3. Seleziona personaggio
# 4. Osserva log nel terminale
```

**Log attesi (sequenza completa):**
```
[INFO] Login successful
[INFO] Characters loaded: 1
[INFO] Selected character: TestChar (ID: 12345)
[Linux] Starting MHFZ launcher
[Linux] Game folder: /home/salvatore/Games/MHFZ
[Linux] Using WINEPREFIX: /home/salvatore/Games/MHFZ/pfx
[Linux] Found game executable: /home/salvatore/Games/MHFZ/mhf.exe
[Linux] Using Wine: wine64
[Linux] Launching game...
[Linux] Game process started (PID: 12345)
```

**✅ Checklist:**
- [ ] Processo Wine parte (verifica con `ps aux | grep wine`)
- [ ] PID è valido e positivo
- [ ] Nessun errore "Permission denied"
- [ ] Nessun errore "File not found"

---

### Test 4.4: Exit Handling

**Test A: Exit normale**
```bash
# Nel gioco: chiudi con X o ESC
# Verifica log:
```

**Log attesi:**
```
[Linux] Game exited successfully
```

**Test B: Exit con errore**
```bash
# Termina processo manualmente:
kill -9 $(ps aux | grep mhf.exe | awk '{print $2}')

# Verifica log:
```

**Log attesi:**
```
[WARN] Game exited with code: -1
```

**Test C: Restart (code 102)**
```bash
# Questo richiede che il gioco ritorni codice 102
# Test solo se implementata logica restart
```

---

## 🞼 Test Step 5: Server Config

### Test 5.1: Server Predefinito Visibile

```bash
# Clean build per forzare reload config
cargo clean
cargo build --release

# Avvia launcher
npm run tauri:dev
```

**UI Checks:**
- [ ] Dropdown server mostra "Avalanche MHFZ (Erupe)"
- [ ] "Avalanche MHFZ" è la PRIMA opzione (preselezionata)
- [ ] "Offline-Mode" è disponibile come seconda opzione

**Screenshot da verificare:**
```bash
# Cattura schermata dropdown
import -window root ~/test-server-dropdown.png
```

---

### Test 5.2: Connessione Server Erupe

**Prerequisito:** Server Erupe deve essere online.

```bash
# Verifica connettività server
ping -c 3 avalanchemhfz.ddns.net

# Verifica porte
nc -zv avalanchemhfz.ddns.net 8094  # Patch server
nc -zv avalanchemhfz.ddns.net 53310 # Game server
```

**Output attesi:**
```
Connection to avalanchemhfz.ddns.net 8094 port [tcp/*] succeeded!
Connection to avalanchemhfz.ddns.net 53310 port [tcp/*] succeeded!
```

**Test login completo:**
```bash
# 1. Launcher già con "Avalanche MHFZ" selezionato
# 2. Inserisci credenziali valide
# 3. Click Login
```

**Criteri successo:**
- [ ] Nessun errore "Connection refused"
- [ ] Nessun errore "Invalid endpoint"
- [ ] Lista personaggi caricata entro 5 secondi
- [ ] UI mostra character selection screen

---

### Test 5.3: Persistenza Configurazione

**Test sequenza:**
```bash
# 1. Apri launcher
# 2. Seleziona "Avalanche MHFZ"
# 3. Login e character select
# 4. Chiudi launcher (X)
# 5. Riapri launcher
```

**Verifica:**
- [ ] "Avalanche MHFZ" è ancora selezionato
- [ ] Username è ricordato (se "Remember me" checked)
- [ ] Ultimo personaggio selezionato è evidenziato

**Config file check:**
```bash
# Verifica file config
cat ~/Games/MHFZ/ButterClient/config.json | jq '.current_endpoint'

# Output atteso:
{
  "name": "Avalanche MHFZ (Erupe)",
  "url": "avalanchemhfz.ddns.net",
  "launcher_port": 8094,
  "game_port": 53310,
  ...
}
```

---

## 🞼 Test Step 6: INI Parser

### Test 6.1: Lettura INI

**Prepara file di test:**
```bash
cat > ~/Games/MHFZ/mhf.ini << 'EOF'
[VIDEO]
GRAPHICS_VER=1

[SCREEN]
FULLSCREEN_MODE=1
FULLSCREEN_RESOLUTION_W=2560
FULLSCREEN_RESOLUTION_H=1440
WINDOW_RESOLUTION_W=1920
WINDOW_RESOLUTION_H=1080

[SOUND]
SOUND_VOLUME=80
SOUND_VOLUME_INACTIVITY=50
SOUND_VOLUME_MINIMIZE=0
EOF
```

**Test nel launcher:**
```bash
npm run tauri:dev

# Apri Settings → Graphics
```

**Verifica valori caricati:**
- [ ] HD Version: Checked (GRAPHICS_VER=1)
- [ ] Fullscreen: Checked (FULLSCREEN_MODE=1)
- [ ] Fullscreen Resolution: 2560x1440
- [ ] Window Resolution: 1920x1080
- [ ] Sound Volume: 80

---

### Test 6.2: Scrittura INI

**Test interattivo:**
```bash
# Nel launcher Settings:
# 1. Cambia "Sound Volume" da 80 a 100
# 2. Click "Save"
# 3. Chiudi launcher
```

**Verifica file INI:**
```bash
cat ~/Games/MHFZ/mhf.ini | grep SOUND_VOLUME

# Output atteso:
SOUND_VOLUME=100
```

**Test valori multipli:**
```bash
# Cambia 5 impostazioni diverse
# Verifica che tutte vengano salvate correttamente
```

---

## ♻️ Regression Testing

**IMPORTANTE**: Eseguire dopo OGNI modifica per verificare che nulla si sia rotto.

### Test Regressione Completo

```bash
#!/bin/bash
# regression_test.sh

set -e  # Exit on error

echo "=== REGRESSION TEST SUITE ==="

# Test 1: Build
echo "[1/6] Testing build..."
cargo build --release 2>&1 | tee build.log
if [ ${PIPESTATUS[0]} -eq 0 ]; then
    echo "✅ Build: PASS"
else
    echo "❌ Build: FAIL"
    exit 1
fi

# Test 2: Clippy
echo "[2/6] Testing clippy..."
cargo clippy -- -D warnings 2>&1 | tee clippy.log
if [ ${PIPESTATUS[0]} -eq 0 ]; then
    echo "✅ Clippy: PASS"
else
    echo "❌ Clippy: FAIL"
    exit 1
fi

# Test 3: Format
echo "[3/6] Testing format..."
cargo fmt -- --check
if [ $? -eq 0 ]; then
    echo "✅ Format: PASS"
else
    echo "❌ Format: FAIL (run: cargo fmt)"
    exit 1
fi

# Test 4: Unit tests
echo "[4/6] Testing units..."
cargo test --lib 2>&1 | tee test.log
if [ ${PIPESTATUS[0]} -eq 0 ]; then
    echo "✅ Unit Tests: PASS"
else
    echo "❌ Unit Tests: FAIL"
    exit 1
fi

# Test 5: Wine detection
echo "[5/6] Testing Wine..."
if command -v wine &> /dev/null; then
    echo "✅ Wine: PASS"
else
    echo "❌ Wine: NOT FOUND"
    exit 1
fi

# Test 6: Dev mode smoke test
echo "[6/6] Testing dev mode (manual)..."
echo "  ⚠️ Manual step: Run 'npm run tauri:dev' and verify UI loads"

echo ""
echo "=== REGRESSION TEST COMPLETE ==="
echo "✅ All automated tests passed!"
echo "⚠️ Remember to test dev mode manually"
```

**Esegui:**
```bash
chmod +x regression_test.sh
./regression_test.sh
```

---

## ⏱️ Performance Testing

### Benchmark Startup Time

```bash
# Test 1: Cold start
time npm run tauri:dev

# Target: < 5 secondi per UI visibile
```

### Benchmark Build Time

```bash
# Clean build
time cargo clean && cargo build --release

# Target: < 5 minuti (prima volta)

# Incremental build
touch src-tauri/src/main.rs
time cargo build --release

# Target: < 30 secondi
```

### Memory Usage

```bash
# Avvia launcher in background
npm run tauri:dev &
LAUNCHER_PID=$!

# Monitora memoria ogni 5 secondi
for i in {1..12}; do
    ps -o rss,vsz,cmd -p $LAUNCHER_PID
    sleep 5
done

# Target: RSS < 200MB idle
```

---

## 📋 Report Template

**Dopo ogni sessione di test, compila:**

```markdown
## Test Report - [DATA]

### Environment
- OS: Arch Linux 6.x
- Wine: 9.0
- Rust: 1.75-nightly
- Commit: [SHA]

### Tests Executed
- [x] Build: PASS
- [x] Clippy: PASS  
- [x] Format: PASS
- [x] Wine Detection: PASS
- [x] Game Launch: PASS
- [x] Server Config: PASS
- [x] INI Parser: PASS (se implementato)

### Issues Found
- Nessuno / [Descrizione issue]

### Performance
- Build time (clean): 4m 23s
- Build time (incr): 18s
- Startup time: 3.2s
- Memory (idle): 145 MB

### Notes
[Note aggiuntive]

### Ready for Commit?
- [x] YES / [ ] NO

**Tester**: @mrsasy89
```

---

**Ultima revisione**: 11 Dicembre 2025  
**Prossimo aggiornamento**: Dopo Step 4 completo
