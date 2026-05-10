//! Outils de parsing pour les differents formats de logs supportes.
//!
//! Le module expose :
//! - des expressions regulieres partagees pour detecter les formats connus ;
//! - la structure [`LogEntry`] qui represente une ligne normalisee ;
//! - des fonctions de parsing pour Symfony, SSH custom et syslog.

use chrono::{DateTime, Utc, NaiveDateTime};
use regex::Regex;
use once_cell::sync::Lazy;

/// Representation normalisee d'une ligne de log.
///
/// Tous les formats ne fournissent pas les memes informations.
/// Les champs sont donc optionnels et remplis quand le parseur
/// correspondant peut les extraire.
#[derive(Debug)]
pub struct LogEntry{
    /// Ligne brute telle qu'elle apparait dans le fichier source.
    pub raw: String,
    /// Date normalisee si elle a pu etre extraite.
    pub date: Option<String>,
    /// Niveau de log (`INFO`, `ERROR`, `DEBUG`, etc.).
    pub level: Option<String>,
    /// Source ou canal de la ligne (`request`, `kernel`, `ssh`, etc.).
    pub request: Option<String>,
    /// Methode HTTP quand la ligne represente une requete.
    pub httpmethod: Option<String>,
    /// Endpoint HTTP extrait de la ligne.
    pub endpoint: Option<String>,
    /// Message principal de la ligne.
    pub message: Option<String>,

}

/// Regex minimale pour extraire un endpoint HTTP depuis une ligne.
pub static RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"(?:GET|POST|PUT|DELETE|PATCH)\s+(/[^\s"]+)"#).unwrap()
});

/// Regex pour les lignes HTTP au format type Symfony/app.
///
/// Exemple :
/// `[2025-05-09 08:00:41] INFO [request] PUT /api/v1/users/42 200 262ms ...`
pub static RE_HTTP_METHOD: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"\[(?P<date>\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2})]\s+(?P<level>INFO|WARNING|ERROR|SUCCESS|DEBUG|CRITICAL)\s+\[(?P<channel>[^]]+)]\s+(?P<method>GET|POST|PUT|PATCH|DELETE|OPTIONS)\s+(?P<path>/\S+)\s+(?P<status>\d{3})"
    ).unwrap()
});

/// Regex pour les lignes Symfony generiques, avec ou sans requete HTTP.
pub static RE_SYMFONY: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"\[(?P<date>\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2})]\s+(?P<level>INFO|WARNING|ERROR|SUCCESS|DEBUG|CRITICAL)\s+\[(?P<channel>[^]]+)]\s+(?P<message>.+)$"
    ).unwrap()
});

/// Regex pour les logs SSH custom au format :
/// `2026-03-26 19:45:02 [INFO] [SFTP] message`
pub static RE_CUSTOM: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"^(?P<date>\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2})\s+\[(?P<level>DEBUG|INFO|WARNING|ERROR|CRITICAL|SUCCESS)](?:\s+\[(?P<context>[^]]+)])?\s+(?P<message>.+)$"    ).unwrap()
});

/// Regex pour les logs syslog traditionnels.
pub static RE_SYSLOG: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"^(?P<date>[A-Z][a-z]{2}\s+\d{1,2}\s+\d{2}:\d{2}:\d{2})\s+(?P<host>\S+)\s+(?P<process>[^\s:]+)(?:\[\d+])?:\s+(?P<message>.+)$"
    ).unwrap()
});

impl LogEntry {
    /// Verifie si une entree correspond a un filtre simple champ/valeur.
    ///
    /// Les champs supportes sont :
    /// - `level`
    /// - `request`
    /// - `endpoint`
    /// - `httpmethod`
    /// - `message`
    pub fn matches_filter(&self, field: &str, val: &str) -> bool {
        match field {
            "level" => opt_contains_ignore_case(self.get_level(), val),
            "request" => opt_contains_ignore_case(self.get_request(), val),
            "endpoint" => opt_contains_ignore_case(self.get_endpoint(), val),
            "httpmethod" => opt_contains_ignore_case(self.get_http_method(), val),
            "message" => opt_contains_ignore_case(self.get_message(), val),
            _ => false,
        }
    }

    /// Retourne le niveau de log si present.
    pub fn get_level(&self) -> Option<&str>{
        self.level.as_deref()
    }

    /// Retourne la source/canal si present.
    pub fn get_request(&self) -> Option<&str>{
        self.request.as_deref()
    }

    /// Retourne l'endpoint HTTP si present.
    pub fn get_endpoint(&self) -> Option<&str>{
        self.endpoint.as_deref()
    }

    /// Retourne le message si present.
    pub fn get_message(&self) -> Option<&str>{
        self.message.as_deref()
    }

    /// Retourne la methode HTTP si presente.
    pub fn get_http_method(&self) -> Option<&str>{
        self.httpmethod.as_deref()
    }


    /// Parse une ligne non reconnue en extrayant uniquement ce qui peut l'etre.
    ///
    /// Cette fonction sert de repli quand aucun format supporte ne matche.
    pub fn parse_unknown(line: &str) -> Self {
        let re = &RE;

        let re_http_method = &RE_HTTP_METHOD;
        let httpmethod = re_http_method
            .captures(line)
            .and_then(|captures| captures.get(1))
            .map(|m| m.as_str().to_string());
        let endpoint = re.captures(line)
            .and_then(|captures| captures.get(1))
            .map(|m| m.as_str().to_string());
        Self {
            raw: line.to_string(),
            date: None,
            level: None,
            request: None,
            endpoint,
            message: Option::from(line.to_string()),
            httpmethod,
        }
    }

    /// Parse une ligne au format Symfony/app.
    ///
    /// Retourne :
    /// - `Ok(Some(LogEntry))` si le format Symfony est reconnu ;
    /// - `Ok(None)` si la ligne ne ressemble pas a une ligne Symfony ;
    /// - `Err` si la ligne ressemble au format mais est mal formee.
    pub fn parse_symfony(line: &str) -> Result<Option<Self>, String> {
        let re =&RE;
        let re_http_method = &RE_HTTP_METHOD;
        if !line.starts_with('[') {
            return Ok(None);
        }

        let end_date = line
            .find(']')
            .ok_or_else(|| "date fermante manquante".to_string())?;
        let raw_date = line
            .strip_prefix('[')
            .ok_or_else(|| "date ouvrante manquante".to_string())?
            .get(..end_date - 1)
            .ok_or_else(|| "date invalide".to_string())?;

        let rest = line
            .get(end_date + 1..)
            .ok_or_else(|| "ligne invalide".to_string())?
            .trim();

        let mut parts = rest.splitn(3, ' ');
        let level = parts
            .next()
            .ok_or_else(|| "level manquant".to_string())?
            .to_string();
        let request_part = parts
            .next()
            .ok_or_else(|| "source manquante".to_string())?;
        let message = parts
            .next()
            .ok_or_else(|| "message manquant".to_string())?
            .to_string();

        let request = request_part
            .trim_start_matches('[')
            .trim_end_matches(']')
            .to_string();

        let naive = NaiveDateTime::parse_from_str(raw_date, "%Y-%m-%d %H:%M:%S")
            .map_err(|e| format!("date invalide: {e}"))?;

        let date = DateTime::<Utc>::from_naive_utc_and_offset(naive, Utc);
        let endpoint = re.captures(line)
            .and_then(|captures| captures.get(1))
            .map(|m| m.as_str().to_string());
        let httpmethod = re_http_method
            .captures(line)
            .and_then(|captures| captures.get(1))
            .map(|m| m.as_str().to_string());

        Ok(Some(Self {
            raw: line.to_string(),
            date: Some(date.to_rfc3339()),
            level: Some(level),
            request: Some(request),
            endpoint,
            message: Some(message),
            httpmethod,
        }))
    }

    /// Parse une ligne SSH custom ou syslog.
    ///
    /// Le parseur tente d'abord le format custom, puis un format syslog classique.
    pub fn parse_ssh(line: &str) -> Result<Option<Self>, String> {
        let re_custom = &RE_CUSTOM;

        let re_syslog = &RE_SYSLOG;

        if let Some(captures) = re_custom.captures(line) {
            let raw_date = captures
                .name("date")
                .ok_or_else(|| "date ssh manquante".to_string())?
                .as_str()
                .to_string();
            let level = captures
                .name("level")
                .ok_or_else(|| "niveau ssh manquant".to_string())?
                .as_str()
                .to_string();
            let message = captures
                .name("message")
                .ok_or_else(|| "message ssh manquant".to_string())?
                .as_str()
                .to_string();

            return Ok(Some(Self {
                raw: line.to_string(),
                date: Some(raw_date),
                level: Some(level),
                request: Some("ssh".to_string()),
                endpoint: None,
                message: Some(message),
                httpmethod: None,
            }));
        }

        let Some(captures) = re_syslog.captures(line) else {
            return Ok(None);
        };

        let raw_date = captures
            .name("date")
            .ok_or_else(|| "date ssh manquante".to_string())?
            .as_str()
            .to_string();
        let request = captures
            .name("process")
            .ok_or_else(|| "process ssh manquant".to_string())?
            .as_str()
            .to_string();
        let message = captures
            .name("message")
            .ok_or_else(|| "message ssh manquant".to_string())?
            .as_str()
            .to_string();

        let lower_message = message.to_lowercase();
        let level = if lower_message.contains("error")
            || lower_message.contains("failed")
            || lower_message.contains("invalid")
        {
            Some("ERROR".to_string())
        } else if lower_message.contains("accepted")
            || lower_message.contains("opened")
            || lower_message.contains("started")
        {
            Some("INFO".to_string())
        } else {
            None
        };

        Ok(Some(Self {
            raw: line.to_string(),
            date: Some(raw_date),
            level,
            request: Some(request),
            endpoint: None,
            message: Some(message),
            httpmethod: None,
        }))
    }


    /// Parse une ligne en essayant les formats connus dans l'ordre.
    ///
    /// Ordre courant :
    /// 1. Symfony/app
    /// 2. SSH custom/syslog
    /// 3. Fallback inconnu
    pub fn parse(line: &str) -> Self {
        match Self::parse_symfony(line) {
            Ok(Some(entry)) => return entry,
            Ok(None) => {}
            Err(err) => {
                eprintln!("Ligne ignoree (symfony): {err} | {line}");
                return Self::parse_unknown(line);
            }
        }

        match Self::parse_ssh(line) {
            Ok(Some(entry)) => return entry,
            Ok(None) => {}
            Err(err) => {
                eprintln!("Ligne ignoree (ssh): {err} | {line}");
                return Self::parse_unknown(line);
            }
        }

        Self::parse_unknown(line)
    }
}


/// Compare une valeur optionnelle avec une recherche insensible a la casse.
///
/// Les crochets eventuels sont ignores pour simplifier les filtres
/// sur les champs du type `[request]` ou `[INFO]`.
fn opt_contains_ignore_case(value: Option<&str>, needle: &str) -> bool {
    let needle = needle.trim().trim_matches(['[', ']']).to_lowercase();
    value
        .as_deref()
        .is_some_and(|s| {
            s.trim()
                .trim_matches(['[', ']'])
                .to_lowercase()
                .contains(&needle)
        })
}
