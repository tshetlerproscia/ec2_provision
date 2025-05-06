use ec2_provision::instance::EC2Output;
use std::error::Error;
use std::io;
use std::thread::sleep;
use std::time::Duration;
use std::{
    fs::OpenOptions,
    io::{Read, Write},
    path::Path,
    process,
};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Failed to provide enough arguments");
        process::exit(1);
    }

    let instance_name = args.get(1).expect("Failed to read instance name");

    println!("Launching EC2...");
    match ec2_provision::launch_ec2_instance(instance_name) {
        Ok(_) => println!("Launched!"),
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    }

    const SLEEP_TIME: u64 = 5;
    println!(
        "Sleeping for {} seconds to allow EC2 to get up & running...",
        SLEEP_TIME
    );
    // // Put execution to sleep for a second to allow the instance time to get out of 'Pending' state
    sleep(Duration::new(SLEEP_TIME, 0));

    println!("Getting EC2 descriptions...");
    let describe_output = match ec2_provision::describe_ec2_instances(instance_name) {
        Ok(o) => o,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };

    let string_to_parse = match String::from_utf8(describe_output.stdout) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to parse describe output into string: {}", e);
            process::exit(1);
        }
    };

    // Parsing from JSON
    let ec2_output: EC2Output = match serde_json::from_str(&string_to_parse) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Failed to serialize describe output into JSON: {}", e);
            process::exit(1);
        }
    };

    let instance = ec2_output
        .Reservations
        .get(0)
        .unwrap()
        .Instances
        .get(0)
        .unwrap();

    println!("Writing public DNS to SSH config...");
    match ec2_provision::write_public_dns_to_ssh_config(instance_name, &instance.PublicDnsName) {
        Ok(_) => println!("Success!"),
        Err(_) => {
            eprintln!("An error occurred...");
            process::exit(1);
        }
    }

    interactive_pause(&instance.PublicDnsName);

    match ec2_provision::write_env_to_ec2(instance) {
        Ok(_) => println!("All done!"),
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    }
}

fn interactive_pause(public_dns_name: &str) {
    println!("EC2 is up and running at {}. Go into the EC2, clone down the repository, and confirm when you've done so", public_dns_name);

    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}

fn write_config_to_hosts_file(
    public_ipv4: String,
    private_dns: String,
) -> Result<(), Box<dyn Error>> {
    let hosts_file_path = Path::new("/etc/hosts");
    let mut hosts_file = OpenOptions::new()
        .append(true)
        .read(true)
        .open(hosts_file_path)?;

    // let mut file_contents = Vec::new();

    // hosts_file.rea(&mut file_contents)?;

    Ok(())
}

#[cfg(test)]
mod tests {

    #[test]
    fn output_is_valid() {
        assert!(true);
    }
}
