pub mod logging{
    use std::{fs::OpenOptions, io::Write, sync::Mutex, time::Instant};

    use actix_web::{body::MessageBody, dev::{ServiceRequest, ServiceResponse}, middleware::Next, Error};
    use chrono::Local;

    pub async fn log_middleware(req: ServiceRequest, next: Next<impl MessageBody>) -> Result<ServiceResponse<impl MessageBody>, Error>{
        let method = req.method().to_string();
        let path = req.uri().to_string();
        let start = Instant::now();

        let peer_addr = req
                .peer_addr()
                .map(|addr| addr.to_string())
                .unwrap_or_else(|| "unknown".into());
        
        let mut log_entry = format!(
            "[{}] {} {} {}\n",
            Local::now().format("%Y-%m-%d %H:%M:%S"),
            peer_addr,
            method,
            path,
        );

        let res = next.call(req).await?;
        if true {
            let status = res.status().as_u16();
            let duration = start.elapsed();
            let now = Local::now().format("%Y-%m-%d %H:%M:%S");

            // Log to file
            log_entry = format!(
                "[{}] {} {} {} {} {}ms\n",
                now,
                peer_addr,
                method,
                path,
                status,
                duration.as_millis(),
            );
        }


        let log_file_name = Local::now().format("%Y-%m-%d.log").to_string();

        // Use a Mutex to safely write to the file
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(format!("logs/{}", log_file_name))
            .expect("Failed to open log file");

        let file = Mutex::new(file);
        let mut file = file.lock().unwrap();
        file.write_all(log_entry.as_bytes())
            .expect("Failed to write to log file");

        Ok(res)
    }
}