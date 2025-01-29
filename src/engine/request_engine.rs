use std::{
    sync::{
        atomic::{AtomicI32, Ordering},
        Arc,
    },
    thread::spawn,
    time::{Duration, Instant},
};

use super::request::EngineRequest;
use fxhash::FxHashMap;
use reqwest::blocking::Client;

pub struct Engine {
    request: EngineRequest,
    has_limit: bool,
    num_requests: Arc<AtomicI32>,
}

impl Engine {
    pub fn new(req: EngineRequest, limit: Option<i32>) -> Self {
        Self {
            request: req,
            has_limit: limit.is_some(),
            num_requests: Arc::new(AtomicI32::new(limit.unwrap_or(0))),
        }
    }
    pub fn new_threads(req: EngineRequest, limit: Option<i32>) -> Self {
        Self {
            request: req,
            has_limit: limit.is_some(),
            num_requests: Arc::new(AtomicI32::new(limit.unwrap_or(0))),
        }
    }

    pub fn run(&self, num_threads: u16) {
        let mut handles = Vec::with_capacity(num_threads as usize);
        let start = Instant::now();
        let start_num_requests = self.num_requests.load(Ordering::Acquire);
        println!("Starting requests...");

        for thread_id in 1..=num_threads {
            let thread_request = self.request.clone();
            let thread_has_limit = self.has_limit;
            let thread_num_requests = Arc::clone(&self.num_requests);
            let handle = spawn(move || {
                Self::worker_thread(
                    thread_id,
                    thread_request,
                    thread_has_limit,
                    thread_num_requests,
                )
            });
            handles.push(handle);
        }
        let mut total_successes = 0;
        let mut total_failures = 0;
        let mut total_code_map = FxHashMap::default();

        for handle in handles {
            let (fails, successes, result_code_map) = handle.join().unwrap();
            total_successes += successes;
            total_failures += fails;
            for (code, count) in result_code_map {
                *total_code_map.entry(code).or_insert(0) += count;
            }
        }
        let end = start.elapsed();
        self.num_requests
            .store(start_num_requests, Ordering::Relaxed);

        println!("Successes: {total_successes}");
        println!("Failures: {total_failures}");
        println!("Status Codes:");
        for code in total_code_map.keys() {
            println!(" {code}: {}", total_code_map.get(code).unwrap());
        }

        println!(
            "Took {:?}. or {:?} per request.",
            end,
            end / (start_num_requests as u32)
        );
    }
    pub fn worker_thread(
        _thread_id: u16,
        request_parameters: EngineRequest,
        has_limit: bool,
        num_requests: Arc<AtomicI32>,
    ) -> (i32, i32, FxHashMap<u16, i32>) {
        let client = Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .unwrap();
        let cookie_str = request_parameters
            .cookies
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join(" ");
        let mut count_successes = 0;
        let mut count_failures = 0;
        let mut map = FxHashMap::default();
        loop {
            if has_limit && num_requests.fetch_sub(1, Ordering::AcqRel) <= 0 {
                break;
            }
            let mut client_request = client.get(request_parameters.url.to_url());
            for header in request_parameters.headers.iter() {
                client_request = client_request.header(&header.key, &header.val);
            }
            client_request = client_request.header("Cookie", &cookie_str);
            client_request = client_request.body(request_parameters.body.clone());
            let raw_response = client_request.send();
            if raw_response.is_ok() {
                *map.entry(raw_response.unwrap().status().as_u16())
                    .or_insert(0) += 1;
                count_successes += 1;
            } else {
                count_failures += 1;
            }
        }
        (count_failures, count_successes, map)
    }
}
