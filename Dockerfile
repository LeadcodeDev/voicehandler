# ---- Builder Stage ----
# Utilise l'image Rust officielle pour la compilation
FROM rust:1.89-bookworm as builder

# Crée un répertoire de travail
WORKDIR /usr/src/app

# Copie les fichiers de dépendances
COPY Cargo.toml Cargo.lock ./

# Crée un projet factice pour télécharger et compiler les dépendances
# Ceci permet de mettre en cache les dépendances et d'accélérer les builds suivants
RUN mkdir src && echo "fn main(){}" > src/main.rs && cargo build --release

# Copie le reste du code source
COPY src ./src

# Construit l'application en mode release
RUN cargo build --release

# ---- Final Stage ----
# Utilise une image de base Debian légère
FROM debian:bookworm-slim

# Copie le binaire compilé depuis l'étage de build
COPY --from=builder /usr/src/app/target/release/voicehanler-rs /usr/local/bin/voicehanler-rs

# Définit la commande à exécuter au démarrage du conteneur
CMD ["voicehandler-rs"]
