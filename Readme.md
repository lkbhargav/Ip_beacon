Deployment steps:
1. Run the make command `make pi` and that uses `Cross` to build a binary for raspberry pi
2. Move the binary to the server and move it to `/usr/local/bin` directory
3. Update the environment variables if necessary from the Supervisor config
4. Run `reread` and `update` commands on the supervisorctl