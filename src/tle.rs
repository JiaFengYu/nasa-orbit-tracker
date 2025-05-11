use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use anyhow::Result;
use reqwest::Client;
use sgp4::{parse_3les, Elements};

pub struct TleStore {
    last_refresh: Instant,
    data: HashMap<u32, Elements>,   // ← store parsed structs
}

impl TleStore {
    pub async fn new() -> Result<Self> {
        let mut me = Self {
            last_refresh: Instant::now() - Duration::from_secs(86_400),
            data: HashMap::new(),
        };
        me.refresh().await?;
        Ok(me)
    }

    pub fn get(&self, id: u32) -> Option<Elements> {
        self.data.get(&id).cloned()          // ← clone out
    }

    pub async fn refresh_if_stale(&mut self) -> Result<()> {
        if self.last_refresh.elapsed() > Duration::from_secs(43_200) {
            self.refresh().await?;
        }
        Ok(())
    }

    async fn refresh(&mut self) -> Result<()> {
        const URL: &str = "https://celestrak.com/NORAD/elements/gp.php?GROUP=active&FORMAT=tle";
        let text = reqwest::get(URL).await?.text().await?;
    
        let mut map = HashMap::new();
        let mut lines = text.lines();
    
        while let (Some(name), Some(l1), Some(l2)) = (lines.next(), lines.next(), lines.next()) {
            println!("name = {name:?} len1={} len2={}", l1.len(), l2.len());
            // Clean CR-LF endings
            let l1 = l1.trim_end_matches('\r');
            let l2 = l2.trim_end_matches('\r');
    
            // Enforce 69-char rule (SGP4 requirement)
            if l1.len() < 69 || l2.len() < 69 {
                continue;                    // skip malformed record
            }
    
            if let Ok(el) = Elements::from_tle(Some(name.to_string()), l1.as_bytes(), l2.as_bytes()) {
                map.insert(el.norad_id as u32, el);
            }
        }
    
        self.data = map;
        self.last_refresh = Instant::now();
        println!("TLE cache refreshed ({} sats)", self.data.len());
        Ok(())
    }
    }
