use std::collections::HashMap;
use std::sync::Mutex;
use std::time::Instant;

struct Bucket {
    capacity: u32,
    tokens: f64,
    refill_rate: f64,
    last_refill: Instant,
}

pub struct RateLimiter {
    buckets: Mutex<HashMap<&'static str, Bucket>>,
}

impl RateLimiter {
    pub fn new() -> Self {
        let mut map = HashMap::new();

        let rules: &[(&str, u32, f64)] = &[
            ("screenshot", 3, 0.1),
            ("camera", 2, 0.067),
            ("shell", 5, 0.2),
            ("getfile", 3, 0.1),
            ("procs", 5, 0.2),
            ("sysinfo", 10, 0.333),
            ("netstat", 5, 0.2),
            ("shutdown", 1, 0.017),
            ("restart", 1, 0.017),
            ("listfiles", 5, 0.2),
            ("clipboard", 10, 0.333),
            ("location", 5, 0.2),
            ("wallpaper", 5, 0.2),
            ("lock", 5, 0.2),
            ("abortshutdown", 5, 0.2),
            ("run", 5, 0.2),
            ("kill", 5, 0.2),
            ("history", 10, 0.333),
            ("update", 1, 0.003),
            ("uninstall", 1, 0.003),
        ];

        for &(name, cap, rate) in rules {
            map.insert(
                name,
                Bucket {
                    capacity: cap,
                    tokens: cap as f64,
                    refill_rate: rate,
                    last_refill: Instant::now(),
                },
            );
        }

        RateLimiter {
            buckets: Mutex::new(map),
        }
    }

    pub fn check(&self, command: &str) -> Result<(), u64> {
        let mut buckets = self.buckets.lock().unwrap();

        if let Some(bucket) = buckets.get_mut(command) {
            let now = Instant::now();
            let elapsed = now.duration_since(bucket.last_refill).as_secs_f64();
            bucket.tokens =
                (bucket.tokens + elapsed * bucket.refill_rate).min(bucket.capacity as f64);
            bucket.last_refill = now;

            if bucket.tokens >= 1.0 {
                bucket.tokens -= 1.0;
                Ok(())
            } else {
                let wait_secs = ((1.0 - bucket.tokens) / bucket.refill_rate).ceil() as u64;
                Err(wait_secs.max(1))
            }
        } else {
            Ok(())
        }
    }
}
