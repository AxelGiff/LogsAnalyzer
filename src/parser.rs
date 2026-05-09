use chrono::{DateTime, Utc, NaiveDateTime};
use regex::Regex;
use once_cell::sync::Lazy;
#[derive(Debug)]
pub struct LogEntry{
    pub raw: String,
    pub date: Option<String>,
    pub level: Option<String>,
    pub request: Option<String>,
    pub httpmethod: Option<String>,
    pub endpoint: Option<String>,
    pub message: Option<String>,

}


static RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"(?:GET|POST|PUT|DELETE|PATCH)\s+(/[^\s"]+)"#).unwrap()
});

static RE_HTTP_METHOD: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\b(GET|POST|PUT|DELETE|PATCH|HEAD|OPTIONS)\b").unwrap()
});
    
static RE_CUSTOM: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^(?P<date>\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2})\s+\[(?P<level>[A-Z]+)\]\s+(?P<message>.+)$").unwrap()
});
    
static RE_SYSLOG: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^(?P<date>[A-Z][a-z]{2}\s+\d{1,2}\s+\d{2}:\d{2}:\d{2})\s+(?P<host>\S+)\s+(?P<process>[^\s:]+)(?:\[\d+\])?:\s+(?P<message>.+)$").unwrap()
});



impl LogEntry {
    // pub fn new(raw:String, date: Option<String>, level: Option<String>, request: Option<String>, endpoint: Option<String>, message: Option<String>, httpmethod: Option<String>) -> LogEntry {
    //   Self {
    //       raw,
    //       date,
    //       level,
    //       request,
    //       endpoint,
    //       message,
    //       httpmethod,
    //   }
    // }

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

    pub fn get_date(&self) -> Option<&str> {
        self.date.as_deref()
    }
    pub fn get_level(&self) -> Option<&str>{
        self.level.as_deref()
    }

    pub fn get_request(&self) -> Option<&str>{
        self.request.as_deref()
    }
    pub fn get_endpoint(&self) -> Option<&str>{
        self.endpoint.as_deref()
    }
    pub fn get_message(&self) -> Option<&str>{
        self.message.as_deref()
    }
    pub fn get_http_method(&self) -> Option<&str>{
        self.httpmethod.as_deref()
    }



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
            endpoint: endpoint,
            message: Option::from(line.to_string()),
            httpmethod: httpmethod,
        }
    }

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
