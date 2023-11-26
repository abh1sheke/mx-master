use hickory_client::client::{Client, SyncClient};
use hickory_client::op::DnsResponse;
use hickory_client::rr::{DNSClass, Name, Record, RecordType};
use hickory_client::udp::UdpClientConnection;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;
use std::thread;

#[derive(Serialize, Deserialize)]
pub struct RecordResult {
    pub mx: Vec<MXRecord>,
    pub err: Vec<ErrorRecord>,
}

impl RecordResult {
    pub fn new() -> Self {
        let mx: Vec<MXRecord> = Vec::new();
        let err: Vec<ErrorRecord> = Vec::new();
        return RecordResult { mx, err };
    }
}

#[derive(Serialize, Deserialize)]
pub struct MXRecord {
    pub domain: String,
    pub ttl: u32,
    pub priority: u16,
    pub target: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorRecord {
    pub domain: String,
    pub reason: String,
}

enum ErrorKind {
    EmptyAnswer,
    InvalidAnswer,
    TooManyRetries,
    ParseError,
    InvalidDomain,
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::EmptyAnswer => write!(f, "Did not receive an answer from domain."),
            Self::InvalidAnswer => write!(f, "Received invalid answer."),
            Self::InvalidDomain => write!(f, "Domain format is invalid."),
            Self::ParseError => write!(f, "Could not parse priority."),
            Self::TooManyRetries => write!(f, "Too many retries."),
        }
    }
}

type QueryResult = Result<Vec<MXRecord>, ErrorRecord>;

fn extract_info(res: DnsResponse, out: &mut Vec<MXRecord>, domain: &str) -> Result<(), ErrorKind> {
    let answers: &[Record] = res.answers();
    for answer in answers {
        let ttl = answer.ttl();
        let res = answer
            .data()
            .ok_or(ErrorKind::EmptyAnswer)?
            .as_mx()
            .ok_or(ErrorKind::InvalidAnswer)?
            .to_string();
        let (priority, target) = res.split_at(res.find(' ').ok_or(ErrorKind::InvalidAnswer)?);
        let target = target.trim().to_string();
        let priority = priority.parse::<u16>().map_err(|_| ErrorKind::ParseError)?;
        let rec = MXRecord {
            domain: domain.to_string(),
            ttl,
            priority,
            target,
        };
        out.push(rec);
    }
    return Ok(());
}

fn query(domain: &str) -> QueryResult {
    let mut out: Vec<MXRecord> = Vec::new();
    let address = "8.8.8.8:53".parse().unwrap();
    let conn = UdpClientConnection::new(address).unwrap();
    let client = SyncClient::new(conn);
    let name = Name::from_str(&domain);
    if let Ok(name) = name {
        let mut retries = 0;
        loop {
            if let Ok(res) = client.query(&name, DNSClass::IN, RecordType::MX) {
                if let Ok(_) = extract_info(res, &mut out, domain) {
                    return Ok(out);
                }
            }
            if retries >= 5 {
                return Err(ErrorRecord {
                    domain: domain.to_string(),
                    reason: ErrorKind::TooManyRetries.to_string(),
                });
            }
            retries += 1;
        }
    } else {
        return Err(ErrorRecord {
            domain: domain.to_string(),
            reason: ErrorKind::InvalidDomain.to_string(),
        });
    }
}

fn run_queries(
    domains: &[String],
    cache: &mut HashMap<String, ()>,
) -> (Vec<MXRecord>, Vec<ErrorRecord>) {
    let mut results: Vec<MXRecord> = Vec::with_capacity(domains.len() * 2);
    let mut errors: Vec<ErrorRecord> = Vec::with_capacity(domains.len());
    let mut jobs: Vec<thread::JoinHandle<QueryResult>> = Vec::with_capacity(domains.len() + 1);
    for domain in domains {
        let domain = domain.to_string();
        if let Some(_) = cache.get(&domain) {
            continue;
        } else {
            cache.insert(domain.clone(), ());
        }
        let job = thread::spawn(move || query(&domain));
        jobs.push(job);
    }
    for job in jobs {
        let res = job.join().unwrap();
        match res {
            Ok(m) => {
                let mut m = m;
                results.append(&mut m);
            }
            Err(e) => errors.push(e),
        };
    }
    return (results, errors);
}

#[tauri::command(async)]
pub async fn query_batcher(domains: String) -> Result<String, ()> {
    let mut ptr = 0;
    let mut res = RecordResult::new();
    let domains: Vec<String> = domains.split(",").map(|s| s.trim().to_string()).collect();
    let mut cache: HashMap<String, ()> = HashMap::new();
    while ptr < domains.len() {
        let mut top_end = ptr + 50;
        if top_end > domains.len() {
            top_end = domains.len();
        }
        let (records, errors) = &mut run_queries(&domains[ptr..top_end], &mut cache);
        if records.len() > 0 {
            res.mx.append(records);
        }
        if errors.len() > 0 {
            res.err.append(errors);
        }
        ptr = top_end;
    }
    res.err.push(ErrorRecord {
        domain: "anjali.com".into(),
        reason: ErrorKind::TooManyRetries.to_string(),
    });
    res.err.push(ErrorRecord {
        domain: "abhisheke.com".into(),
        reason: ErrorKind::InvalidAnswer.to_string(),
    });
    res.err.push(ErrorRecord {
        domain: "anjasheke.com".into(),
        reason: ErrorKind::InvalidDomain.to_string(),
    });
    println!("{:?}", res.err);
    let res = serde_json::to_string(&res).unwrap();
    return Ok(res);
}
