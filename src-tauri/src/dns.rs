use hickory_client::client::{Client, SyncClient};
use hickory_client::op::DnsResponse;
use hickory_client::rr::{DNSClass, Name, Record, RecordType};
use hickory_client::udp::UdpClientConnection;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;
use std::thread;

#[derive(Serialize, Deserialize)]
pub struct MXRecord {
    pub domain: String,
    pub ttl: u32,
    pub priority: u16,
    pub target: String,
}

fn extract_info(res: DnsResponse, out: &mut Vec<MXRecord>, domain: &str) -> Result<(), ()> {
    let answers: &[Record] = res.answers();
    for answer in answers {
        let ttl = answer.ttl();
        let res = answer.data().ok_or(())?.as_mx().ok_or(())?.to_string();
        let (priority, target) = res.split_at(res.find(' ').ok_or(())?);
        let target = target.trim().to_string();
        let priority = priority.parse::<u16>().map_err(|_| ())?;
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

fn query(domain: &str) -> Vec<MXRecord> {
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
                    break;
                }
            }
            if retries >= 5 {
                out.push(MXRecord {
                    domain: domain.into(),
                    ttl: 0,
                    priority: 0,
                    target: "FAILED".into(),
                });
                break;
            }
            retries += 1;
        }
        return out;
    } else {
        return vec![MXRecord {
            domain: domain.into(),
            ttl: 0,
            priority: 0,
            target: "FAILED".into(),
        }];
    }
}

fn run_queries(domains: &[String], cache: &mut HashMap<String, ()>) -> Vec<MXRecord> {
    let mut results: Vec<MXRecord> = Vec::with_capacity(domains.len() * 2);
    let mut jobs: Vec<thread::JoinHandle<Vec<MXRecord>>> = Vec::with_capacity(domains.len() + 1);
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
        let mut res = job.join().unwrap();
        results.append(&mut res);
    }
    return results;
}

#[tauri::command(async)]
pub async fn query_batcher(domains: String) -> Result<String, ()> {
    let mut ptr = 0;
    let mut res: Vec<MXRecord> = Vec::with_capacity(domains.len());
    let domains: Vec<String> = domains.split(",").map(|s| s.trim().to_string()).collect();
    let mut cache: HashMap<String, ()> = HashMap::new();
    while ptr < domains.len() {
        let mut top_end = ptr + 50;
        if top_end > domains.len() {
            top_end = domains.len();
        }
        res.append(&mut run_queries(&domains[ptr..top_end], &mut cache));
        ptr = top_end;
    }
    let res = serde_json::to_string(&res).unwrap();
    return Ok(res);
}
