# Tradgance-API

Une API Rust pour l'automatisation du trading de cryptomonnaies avec Binance, intégrant la surveillance d'emails pour les signaux de trading.

## 📋 Description

Tradgance-API est un système de trading automatisé qui :

- Surveille les emails entrants pour détecter les signaux de trading
- Parse automatiquement les signaux (entry, take profit, stop loss)
- Calcule la gestion des risques (money management)
- Interface avec l'API Binance pour exécuter les trades
- Fournit une API REST pour le monitoring

## 🚀 Fonctionnalités

### 🔍 Surveillance Email

- Connexion IMAP automatique avec retry
- Surveillance en temps réel des nouveaux emails (IDLE)
- Parsing intelligent des signaux de trading

### 📊 Gestion des Trades

- Analyse automatique des signaux (entry, TP, SL)
- Détection automatique de la direction (Buy/Sell)
- Calcul automatique de la quantité basé sur le risk management
- Validation de cohérence des signaux

### 💰 Money Management

- Calcul automatique du montant à risquer (2% du capital libre)
- Récupération des balances Binance en temps réel
- Gestion sécurisée des positions

### 🌐 API REST

- Interface web pour monitoring
- Endpoints pour status et contrôle

## 🛠️ Technologies Utilisées

- **Rust** - Langage principal
- **Axum** - Framework web async
- **Tokio** - Runtime async
- **Binance SDK** - Interface API Binance
- **IMAP** - Surveillance des emails
- **Dotenv** - Gestion des variables d'environnement

## 📦 Installation

### Prérequis

- Rust 1.70+ avec edition 2024
- Compte Binance avec API activée
- Accès IMAP à votre boîte email

### 1. Cloner le projet

```bash
git clone https://github.com/WalsonK/Tradgance-API.git
cd Tradgance-API
```

### 2. Configuration des variables d'environnement

Créez un fichier `.env` à la racine du projet :

```env
BINANCE_API_KEY=votre_api_key_binance
BINANCE_PASS=votre_secret_binance

# Configuration IMAP (à adapter selon votre fournisseur)
IMAP_SERVER=imap.gmail.com
IMAP_PORT=993
IMAP_USERNAME=votre_email@gmail.com
IMAP_PASSWORD=votre_mot_de_passe_app
```

### 3. Installation des dépendances

```bash
cargo build
```

## 🚀 Utilisation

### Lancement de l'application

```bash
cargo run
```

L'application démarre :

- Le serveur web sur `http://127.0.0.1:3000`
- La surveillance email en arrière-plan

### Format des signaux email

Les emails de signaux doivent contenir :

```
entry:45000 take_profit:47000 stop_loss:43000
```

ou

```
entry: 45000; take_profit: 47000; stop_loss: 43000
```

### API Endpoints

- `GET /` - Status de l'application

## 🏗️ Architecture

```
src/
├── main.rs              # Point d'entrée et orchestration
├── binance/            # Module Binance
│   ├── mod.rs
│   ├── tools.rs        # Configuration et money management
│   └── wallet.rs       # Gestion des balances
├── mail/               # Module Email
│   ├── mod.rs
│   ├── mailer.rs       # Connexion IMAP
│   ├── monitor.rs      # Surveillance IDLE
│   └── tools.rs        # Parsing des signaux
├── models/             # Structures de données
│   ├── mod.rs
│   ├── direction.rs    # Enum Buy/Sell
│   └── trade.rs        # Structure TradeSignal
├── routes/             # Endpoints API
│   ├── mod.rs
│   └── hello.rs
└── tester/             # Tests et utilitaires
    ├── mod.rs
    └── tradesignal.rs
```

## ⚙️ Configuration

### Paramètres de Risk Management

- **Risque par trade** : 2% du capital libre (configurable dans `binance/tools.rs`)
- **Devise de base** : USDC (configurable)

### Paramètres Email

- **Timeout IDLE** : 25 minutes
- **Reconnexion automatique** en cas de déconnexion

## 🔒 Sécurité

- Les clés API sont stockées dans les variables d'environnement
- Validation des signaux avant exécution
- Gestion d'erreurs robuste
- Logs détaillés pour audit

## 🧪 Tests

```bash
# Tests unitaires
cargo test

# Tests d'intégration
cargo test --test integration_tests
```

## 📈 Monitoring

L'application fournit des logs détaillés :

- Connexions IMAP
- Signaux détectés
- Balances récupérées
- Calculs de risk management
- Erreurs et reconnexions

## 🤝 Contribution

1. Fork le projet
2. Créez une branche feature (`git checkout -b feature/nouvelle-fonctionnalite`)
3. Committez vos changements (`git commit -am 'Ajout nouvelle fonctionnalité'`)
4. Poussez la branche (`git push origin feature/nouvelle-fonctionnalite`)
5. Ouvrez une Pull Request

## ⚠️ Avertissements

- **Trading à risque** : Ce logiciel peut exécuter des trades réels avec de l'argent réel
- **Tests requis** : Testez toujours en mode paper trading avant d'utiliser avec de vrais fonds
- **Surveillance** : Supervisez toujours le système pendant son fonctionnement
- **Pas de garantie** : Aucune garantie de profit, tradez à vos risques et périls

## 📜 Licence

Ce projet est sous licence MIT. Voir le fichier `LICENSE` pour plus de détails.

## 📞 Support

- Issues GitHub pour les bugs et demandes de fonctionnalités
- Documentation Binance API : https://binance-docs.github.io/apidocs/
- Documentation Rust : https://doc.rust-lang.org/

---

**⚡ Made with ❤️ and Rust**
