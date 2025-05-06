use std::{error::Error, fs::{File, OpenOptions}, io::Write, path::Path, process::{self, Command, Stdio}};

use instance::Instance;

pub mod instance;

pub fn launch_ec2_instance(instance_name: &str) -> Result<process::Output, Box<dyn Error>> {
    let tag_specs = &format!(
        "{{\"ResourceType\":\"instance\",\"Tags\":[{{\"Key\":\"Name\",\"Value\":\"{}\"}}]}}",
        instance_name
    );
    let launch_args = vec![
        "ec2",
        "run-instances",
        "--image-id", "ami-0f9de6e2d2f067fca",
        "--instance-type", "t2.large",
        "--key-name", "tom-c4r",
        "--block-device-mappings", "{\"DeviceName\":\"/dev/sda1\",\"Ebs\":{\"Encrypted\":false,\"DeleteOnTermination\":true,\"SnapshotId\":\"snap-0706476dde26d8e82\",\"VolumeSize\":150,\"VolumeType\":\"gp2\"}}",
        "--network-interfaces", "{\"AssociatePublicIpAddress\":true,\"DeviceIndex\":0,\"Groups\":[\"sg-02bd20ae73bf7c97b\"]}",
        "--credit-specification", "{\"CpuCredits\":\"standard\"}",
        "--tag-specifications", tag_specs,
        "--metadata-options", "{\"HttpEndpoint\":\"enabled\",\"HttpPutResponseHopLimit\":2,\"HttpTokens\":\"required\"}",
        "--private-dns-name-options", "{\"HostnameType\":\"ip-name\",\"EnableResourceNameDnsARecord\":true,\"EnableResourceNameDnsAAAARecord\":false}",
        "--count", "1", "--profile", "UE1-Developer-976326599866"
    ];

    let launch_output = Command::new("aws")
        .args(launch_args)
        .output()?;
    
    Ok(launch_output)
}

pub fn describe_ec2_instances(instance_name: &str) -> Result<process::Output, Box<dyn Error>> {
    let output = Command::new("aws")
        .args([
            "ec2",
            "describe-instances",
            "--filters",
            &format!("Name=tag:Name,Values={}", instance_name),
            "Name=instance-state-name,Values=running",
            "--profile",
            "UE1-Developer-976326599866",
        ])
        .output()?;

    Ok(output)
}

pub fn write_public_dns_to_ssh_config(
    instance_name: &str,
    public_dns: &str,
) -> Result<(), Box<dyn Error>> {
    let config_file_path = Path::new("/home/tshetlerproscia/.ssh/config");
    let mut config_file = OpenOptions::new()
        .append(true)
        .read(true)
        .open(config_file_path)?;

    let config_content = format!(
        "\n# {} \nHost {}\n  HostName {}\n  User ubuntu\n",
        instance_name, public_dns, public_dns
    );

    config_file.write_all(config_content.as_bytes())?;
    Ok(())
}

pub fn write_env_to_ec2(instance: &Instance) -> Result<(), Box<dyn Error>> {
    let env_file_path = Path::new("sample.env");
    let env_file_contents =
        std::fs::read_to_string(env_file_path).expect("Failed to read from ENV file");

    let contents = env_file_contents.replace("<PRIVATE_DNS>", &instance.PrivateDnsName).replace("<PRIVATE_IP>", &instance.PrivateIpAddress.clone().unwrap());

    match std::fs::write(".env", contents) {
        Ok(_) => println!("Successfully wrote ENV file"),
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    }

    let file = File::open(".env").expect("Failed to open .env");

    let mut ssh_child = Command::new("ssh")
        .args([
            "-i",
            "/home/tshetlerproscia/tom-c4r.pem",
            &format!("ubuntu@{}", &instance.PublicDnsName),
            "cat > /home/ubuntu/concentriq-customer-hosted/c4r/.env",
        ])
        .stdin(Stdio::from(file))
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("Failed to start ssh process");

    let status = ssh_child.wait()?;

    if status.success() {
        println!("Successfully uploaded .env to remote machine.");
    } else {
        eprintln!("ssh process failed with status: {:?}", status);
    }

    Ok(())
}