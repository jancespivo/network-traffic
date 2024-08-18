use std::io::{Read, Seek};

const SIZE_UNITS: [char; 4] = ['B', 'K', 'M', 'G'];

fn human_readable(num_bytes: u64) -> String {
    let idx = num_bytes.checked_ilog10().unwrap_or(0) / 3;
    let rounder = 1000_u64.pow(idx);
    let rounded_bytes = (num_bytes as f64 / rounder as f64).round() as u64;
    format!("{}{}", rounded_bytes, SIZE_UNITS[idx as usize])
}


fn main() -> std::io::Result<()> {
    let (mut previous_receive, mut previous_transmit) = (0, 0);
    let mut file = std::fs::File::open("/proc/net/dev")?;

    let addr = "8.8.8.8".parse().unwrap();
    let data = [1];  // ping data
    let timeout = std::time::Duration::from_secs(1);
    let options = ping_rs::PingOptions { ttl: 128, dont_fragment: true };
    loop {
        let mut file_contents = String::new();
        file.rewind().unwrap();
        file.read_to_string(&mut file_contents).unwrap();
        let mut lines = file_contents.lines();
        let _ = lines.next();
        let _ = lines.next();
        let mut sleep_time = std::time::Duration::from_secs(1);
        if let Some((receive, transmit)) = lines.map(
            |line| {
                let cols: Vec<_> = line.split_whitespace().collect();
                (cols[1].parse::<u64>().unwrap(), cols[9].parse::<u64>().unwrap())
            }
        ).reduce(|(previous_recieve, previous_transmit), (receive, transmit)| (previous_recieve + receive, previous_transmit + transmit)) {
            let (delta_receive, delta_transmit) = (receive - previous_receive, transmit - previous_transmit);

            let mut is_online = true;
            if delta_receive == 0 {
                // possible broken connection
                let start = std::time::Instant::now();
                let result = ping_rs::send_ping(&addr, timeout, &data, Some(&options));
                let time_elapsed = std::time::Instant::now() - start;
                if let Err(_) = result {
                    is_online = false;
                }
                sleep_time =  sleep_time.checked_sub(time_elapsed).unwrap_or(std::time::Duration::ZERO)
            }

            let mut receive_txt = "X".to_string();
            let mut transmit_txt = "X".to_string();

            if is_online {
                receive_txt = human_readable(delta_receive);
                transmit_txt = human_readable(delta_transmit);
            }

            println!("{{\"text\": \"{:>4}⇣ {:>4}⇡\"}}", receive_txt, transmit_txt);

            // TODO color formatting when disconnected
            (previous_receive, previous_transmit) = (receive, transmit);
        }
        std::thread::sleep(sleep_time);
    }
}
