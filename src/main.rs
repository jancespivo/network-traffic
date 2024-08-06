const SIZE_UNITS: [char; 4] = ['B', 'K', 'M', 'G'];

fn human_readable(num_bytes: u64) -> String {
    let idx = num_bytes.checked_ilog10().unwrap_or(0) / 3;
    let rounder = 1000_u64.pow(idx);
    let rounded_bytes = (num_bytes as f64 / rounder as f64).round() as u64;
    format!("{}{}", rounded_bytes, SIZE_UNITS[idx as usize])
}


fn main() -> std::io::Result<()> {
    let (mut previous_receive, mut previous_transmit) = (0, 0);
    loop {
        let file_contents = std::fs::read_to_string("/proc/net/dev")?;
        let mut lines = file_contents.lines();
        let _ = lines.next();
        let _ = lines.next();
        if let Some((receive, transmit)) = lines.map(
            |line| {
                let cols: Vec<_> = line.split_whitespace().collect();
                (cols[1].parse::<u64>().unwrap(), cols[9].parse::<u64>().unwrap())
            }
        ).reduce(|(previous_recieve, previous_transmit), (receive, transmit)| (previous_recieve + receive, previous_transmit + transmit)) {
            let (delta_receive, delta_transmit) = (receive - previous_receive, transmit - previous_transmit);

            println!("{{\"text\": \"{:>4}⇣ {:>4}⇡\"}}", human_readable(delta_receive), human_readable(delta_transmit));

            (previous_receive, previous_transmit) = (receive, transmit);
        }
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
