# 📚 Documentazione MHFZ-Launcher Linux Porting

**Benvenuto nella documentazione completa del progetto di porting Linux per MHFZ-Launcher!**

Questa cartella contiene tutta la documentazione tecnica, i piani di implementazione e le procedure di test per portare il launcher Monster Hunter Frontier Z su Linux.

---

## 📌 Documenti Disponibili

### 1. [📊 ANALYSIS.md](ANALYSIS.md)
**Analisi Dettagliata delle Modifiche**

- Panoramica completa delle modifiche ButterClient → MHFZ-Launcher
- Confronto codice originale vs modificato
- Analisi file-by-file di ogni componente
- Problematiche identificate e soluzioni proposte
- Stato attuale del porting (60% completato)

**Quando leggerlo:**
- Prima di iniziare qualsiasi modifica
- Per capire il contesto delle scelte tecniche
- Per vedere cosa è già stato fatto

---

### 2. [🛠️ IMPLEMENTATION_PLAN.md](IMPLEMENTATION_PLAN.md)
**Piano di Implementazione Step-by-Step**

- Roadmap dettagliata per Step 4-7
- Codice completo pronto da copiare
- Comandi esatti per ogni operazione
- Workflow Git con branch feature
- Template per Pull Request
- Timeline stimata: 5-6 ore totali

**Quando leggerlo:**
- Prima di implementare una nuova feature
- Per vedere il codice esatto da scrivere
- Per seguire il workflow Git corretto

---

### 3. [✅ TESTING_CHECKLIST.md](TESTING_CHECKLIST.md)
**Checklist Testing Completa**

- Test pre-commit obbligatori
- Test specifici per ogni Step (4, 5, 6)
- Regression testing automatizzato
- Performance benchmarks
- Script di test pronti all'uso
- Report template

**Quando leggerlo:**
- PRIMA di ogni commit
- Dopo aver implementato una feature
- Per verificare che nulla si sia rotto

---

## 🚀 Quick Start

### Per Nuovi Contributori

**Workflow raccomandato:**

```bash
# 1. Leggi la documentazione nell'ordine:
1. ANALYSIS.md          # Capisci il contesto
2. IMPLEMENTATION_PLAN.md  # Scegli uno Step da implementare
3. TESTING_CHECKLIST.md    # Prepara l'ambiente di test

# 2. Setup ambiente
cd ~/Progetti/MHFZ-Launcher
rustup override set nightly
export WINEPREFIX="$HOME/Games/MHFZ/pfx"

# 3. Crea branch feature
git checkout -b feature/step-X-description

# 4. Implementa seguendo IMPLEMENTATION_PLAN.md
# (copia il codice, modifica, testa)

# 5. Testa seguendo TESTING_CHECKLIST.md
./regression_test.sh

# 6. Commit e Push
git add .
git commit -m "feat(linux): description"
git push origin feature/step-X-description

# 7. Crea Pull Request su GitHub
```

---

## 🎯 Stato Attuale del Progetto

### ✅ Completato (Step 1-3)

- Conditional compilation Windows/Linux
- Struttura modulare con `#[cfg(target_os)]`
- Settings.rs con fallback Linux
- Progetto compila su Linux senza errori

### 🟬 In Corso (Step 4-5)

**Step 4: Wine/Proton Game Launcher**
- 📄 Codice pronto in IMPLEMENTATION_PLAN.md
- ⚠️ Da testare su hardware reale
- File da creare: `src-tauri/src/lib_linux.rs`

**Step 5: Server Predefinito**
- 📄 Codice pronto in IMPLEMENTATION_PLAN.md
- 🚨 CRITICO: Senza questo il launcher non è usabile
- File da modificare: `src-tauri/src/config.rs`

### 📌 Pianificato (Step 6-7)

**Step 6: INI Parser Completo**
- Lettura/scrittura `mhf.ini` su Linux
- Dipendenza: `configparser` crate
- Priorità: Media

**Step 7: Testing & Release**
- Test multi-distro (Ubuntu, Debian, Steam Deck)
- Build release con tauri-action
- Documentazione utente finale

---

## 📊 Metriche Progetto

| Metrica | Valore |
|---------|--------|
| **Completamento** | 60% |
| **File modificati** | 5 |
| **File da creare** | 1 (`lib_linux.rs`) |
| **LOC aggiunte** | ~200 |
| **Test da eseguire** | 25+ |
| **Tempo rimanente** | 5-6 ore |

---

## 🔗 Link Utili

### Documentazione Esterna

- [Wine HQ](https://wiki.winehq.org/) - Documentazione Wine ufficiale
- [GE-Proton Releases](https://github.com/GloriousEggroll/proton-ge-custom/releases) - Proton ottimizzato per gaming
- [Tauri Docs](https://tauri.app/v1/guides/) - Framework Tauri
- [Rust std::process](https://doc.rust-lang.org/std/process/) - Process management in Rust

### Repository Correlati

- [ButterClient (upstream)](https://github.com/RuriYoshinova/ButterClient) - Launcher originale Windows
- [Erupe Server](https://github.com/ErupeServer/Erupe) - Server privato MHFZ

---

## 🤝 Come Contribuire

### Contribuzioni Benvenute

**Aree dove servono contributori:**

1. **Testing multi-distro**
   - Test su Ubuntu 22.04 / 24.04
   - Test su Debian 12
   - Test su Steam Deck

2. **Implementazione features**
   - Step 4: Wine launcher (priorità alta)
   - Step 5: Server config (priorità critica)
   - Step 6: INI parser (priorità media)

3. **Documentazione**
   - Guida utente finale
   - Troubleshooting comune
   - Screenshot e video demo

### Workflow Contributore

1. **Fork** il repository
2. **Leggi** ANALYSIS.md per capire il contesto
3. **Scegli** uno Step da IMPLEMENTATION_PLAN.md
4. **Implementa** seguendo il codice fornito
5. **Testa** con TESTING_CHECKLIST.md
6. **Apri** Pull Request con descrizione dettagliata

### Standard di Qualità

**Ogni PR deve:**
- ✅ Passare tutti i test di TESTING_CHECKLIST.md
- ✅ Includere commit messages in formato Conventional Commits
- ✅ Avere descrizione dettagliata con screenshot/log
- ✅ Non rompere funzionalità Windows esistenti
- ✅ Essere testata manualmente su almeno 1 distro Linux

---

## 🐛 Issue Tracker

**Segnala bug o problemi:**

[GitHub Issues](https://github.com/mrsasy89/MHFZ-Launcher/issues)

**Template issue:**
```markdown
## Descrizione
[Descrivi il problema]

## Environment
- OS: Arch Linux / Ubuntu / ...
- Wine: 9.0
- Commit: [SHA]

## Passi per Riprodurre
1. ...
2. ...

## Comportamento Atteso
[Cosa dovrebbe succedere]

## Comportamento Attuale
[Cosa succede invece]

## Log
```
[Incolla log qui]
```

## Screenshot
[Se disponibili]
```

---

## ❓ FAQ

### Q: Devo conoscere Rust per contribuire?
A: Per implementare Step 4-5, il codice completo è già scritto in IMPLEMENTATION_PLAN.md. Basta copiarlo e testarlo. Per Step 6 serve conoscenza base di Rust.

### Q: Quanto tempo serve per completare il porting?
A: ~5-6 ore per Step 4-6, se si seguono i piani. Il testing richiede altre 2-3 ore.

### Q: Funziona su Steam Deck?
A: Non ancora testato, ma dovrebbe funzionare. Usa Proton GE al posto di Wine.

### Q: Posso usare il launcher su Windows dopo le modifiche?
A: Sì! Tutte le modifiche usano conditional compilation. La versione Windows rimane identica.

### Q: Dove trovo il server Erupe?
A: `avalanchemhfz.ddns.net:53310` (sarà preconfigurato dopo Step 5)

---

## 📞 Contatti

**Maintainer**: [@mrsasy89](https://github.com/mrsasy89)

**Per domande tecniche**: Apri una issue su GitHub  
**Per discussioni generali**: [Discord Erupe](https://discord.gg/erupe) (se disponibile)

---

## 📝 License

Questo progetto è un fork di [ButterClient](https://github.com/RuriYoshinova/ButterClient).

Vedi [LICENSE](../LICENSE) per dettagli.

---

**Ultima revisione**: 11 Dicembre 2025  
**Prossimo aggiornamento**: Dopo completamento Step 4-5

---

## 🎉 Inizia Ora!

**Sei pronto per contribuire?**

1. Leggi [ANALYSIS.md](ANALYSIS.md) (10 minuti)
2. Apri [IMPLEMENTATION_PLAN.md](IMPLEMENTATION_PLAN.md) e scegli uno Step
3. Segui [TESTING_CHECKLIST.md](TESTING_CHECKLIST.md) per testare
4. Apri la tua prima Pull Request!

**Buon coding! 🚀**
