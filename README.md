# LogsAnalyzer

LogsAnalyzer est un petit utilitaire en ligne de commande écrit en Rust pour lire, colorer et filtrer des fichiers de logs. Il détecte plusieurs formats de logs (Symfony, syslog, format personnalisé, traces HTTP) et met en couleur les parties importantes (date, niveau, méthode HTTP, code de statut, process, host, ...). Il peut aussi suivre un fichier en mode "tail" (watch) pour afficher les nouvelles lignes en temps réel.

## En quoi consiste le projet

L'objectif est d'offrir un outil simple et rapide pour parcourir des fichiers de logs depuis un terminal, repérer rapidement les erreurs/informations importantes grâce à une coloration syntaxique et appliquer des filtres (par niveau, endpoint, méthode HTTP, message, etc.). Il est pensé pour être utilisé localement sur des fichiers de logs d'applications web (Symfony), des fichiers syslog/ssh, ou tout autre log texte contenant des timestamps et/ou méthodes HTTP.

## Liste des crates (dépendances)

Le projet utilise les crates suivantes (versions dans Cargo.toml) :

- chrono = "0.4.44" — parsing et formatage des dates
- clap = "4.6.1" (features: cargo, derive) — parsing des options CLI
- colored = "3.1.1" — coloration des sorties terminal
- notify = "8.2.0" — surveillance de fichier (tail/watch)
- once_cell = "1.21.4" — initialisation paresseuse des Regex
- regex = "1.12.3" — expressions régulières pour parser les logs

## Performance
Etant donné la possibilité de lire un fichier de logs volumineux (plus de 10go par exemple), des outils comme BuffReader ou bien once_cell sont implémentés pour : 

- BuffReader : Lire chaque ligne et ne pas stocker en mémoire
- once_cell : Eviter de recharger les Regex en mémoire
  
## Fonctionnalités

- Détection et parsing de plusieurs formats de logs :
  - Logs Symfony (format: [YYYY-MM-DD HH:MM:SS] LEVEL [channel] ...)
  - Format personnalisé avec date et niveau (ex: "2024-01-01 12:00:00 [INFO] ...")
  - Syslog standard (date host process: message)
  - Détection d'URL/méthode HTTP dans les lignes de logs
- Coloration des éléments importants : date, niveau (ERROR en rouge, WARNING en jaune, INFO en bleu, SUCCESS en vert, DEBUG en cyan), méthode HTTP (GET/POST/PUT/DELETE...), codes HTTP (200 vert, 404/500 rouge, ...), host/process, etc.
- Filtrage par champ : level, request, endpoint, httpmethod, message
- Mode "tail" : surveille un fichier et ré-affiche le contenu à chaque changement
- Lecture complète d'un fichier et sortie colorée dans le terminal

## Comment l'installer

Prérequis : avoir Rust et Cargo installés (https://www.rust-lang.org/tools/install).

Installation locale (construction) :

1. Récupérer le dépôt :

   git clone https://github.com/AxelGiff/LogsAnalyzer.git
   cd LogsAnalyzer

2. Compiler en release :

   cargo build --release

3. L'exécutable sera disponible dans `target/release/LogsAnalyzer`.

Installer globalement (optionnel) :

- Pour installer localement sur votre machine afin d'avoir la commande disponible globalement :

  cargo install --path .

Remarque : `cargo install --path .` installe le binaire dans le dossier binaire de Cargo (habituellement ~/.cargo/bin).

## Comment l'utiliser

Usage général :

- Afficher un fichier de logs coloré :

  ./target/release/LogsAnalyzer --logfile /chemin/vers/fichier.log

  ou si installé :

  LogsAnalyzer --logfile /chemin/vers/fichier.log

Options principales :

- `-l, --logfile <FILE>` : chemin vers le fichier de logs à lire (obligatoire)
- `-f, --filter <FIELD> <VALUE>` : filtrer les lignes selon un champ et une valeur (ex: `-f level ERROR`). ATTENTION : `--filter` attend 2 arguments (FIELD et VALUE)
- `-t, --tail` : activer le mode suivi (similaire à `tail -f`) — ré-affiche le fichier à chaque modification

Exemples :

- Afficher tout le fichier :

  LogsAnalyzer --logfile ./logs/app.log

- Filtrer par niveau :

  LogsAnalyzer --logfile ./logs/app.log --filter level ERROR

- Filtrer par endpoint :

  LogsAnalyzer --logfile ./logs/app.log --filter endpoint /api/users

- Suivre un fichier en temps réel :

  LogsAnalyzer --logfile ./logs/app.log --tail

- Combiner filtre + tail :

  LogsAnalyzer --logfile ./logs/app.log --filter level ERROR --tail

Champs de filtre supportés : `level`, `request`, `endpoint`, `httpmethod`, `message`.

Comportement du filtre : la recherche est insensible à la casse et accepte des sous-chaînes (contains). Par exemple `--filter level error` ou `--filter message login`.

## Détails techniques / Format des logs pris en charge

Le parser implémente plusieurs Regex pour reconnaître et extraire :

- Date (ex: `2024-01-01 12:00:00`),
- Niveau (`INFO`, `WARNING`, `ERROR`, `DEBUG`, `CRITICAL`, `SUCCESS`),
- Channel / request (ex: `[app]`),
- Méthode HTTP et endpoint (ex: `GET /api/whatever 200`),
- Syslog (ex: `Apr 10 12:00:00 hostname process[1234]: message`),
- Format personnalisé SSH/syslog.

Pour les lignes qui ne correspondent à aucun format reconnu, l'outil garde la ligne brute et tente d'extraire une URL/méthode HTTP si présente.

## Exemple concret

Supposons un fichier `app.log` contenant des lignes Symfony et des entrées HTTP :

- Affichage coloré des dates et des niveaux pour repérer rapidement les erreurs.
- Les codes HTTP 500/404/503/403 sont mis en rouge, 200/203/204 en vert.
- Les méthodes GET sont en vert, POST en bleu, PUT en jaune, DELETE en rouge, PATCH en couleur personnalisée.

Commandes :

- Voir toutes les erreurs :

  LogsAnalyzer -l app.log -f level ERROR

- Suivre le fichier en temps réel et repérer les nouvelles erreurs :

  LogsAnalyzer -l app.log -f level ERROR -t

## Limitations et améliorations possibles

- L'outil fonctionne en lecture ligne à ligne ; pour des fichiers très volumineux, la mémoire est consommée si vous utilisez `--filter` car le parser charge toutes les entrées en mémoire pour effectuer le filtrage (read_file_contents retourne un Vec<LogEntry>).
- Ajout possible : exports (CSV/JSON), interface interactive (TUI), support de patterns de filtre plus avancés (expressions régulières), ou amélioration de la configuration (fichier de configuration pour formats personnalisés).

## Contribuer

PR et issues bienvenus. N'hésitez pas à ouvrir une issue avant une grosse modification pour discuter du design.
