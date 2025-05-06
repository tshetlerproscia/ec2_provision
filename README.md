# EC2 Provisioning App

Automates the tedious manual process of:
- Spinning up an EC2 with Ubuntu 22 using my key file and my security group settings
- Updating my local SSH config with the new public DNS to allow connecting remotely with VS Code
- Populating that instance with a valid `.env` file for a 4.3 deployment

## TODO Improvements
- Combine this with the work from the new BMS seeding app (once it's ready) to make a full deployment pipeline so that you can just run the application and check back in an hour to see everything ready to roll
- Currently this relies on a manual intervention to clone down the `concentriq-customer-hosted` repo on the machine. Ideally this could be bypassed as well
so that the entire setup process is automated
