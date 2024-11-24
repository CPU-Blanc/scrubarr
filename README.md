# Scrubarr

## Overview
Scrubarr is a lightweight app written in Rust with the sole purpose of cleaning up and expediting the Sonarr queue. Many anime simulcasts get stuck
in the queue as TBA for multiple hours resulting in a long delay before import, this app helps relieve that.\
This little project was mainly a playground to learn more about dockerising applications and automating those workflows; increases in scope are not to be expected.

## Features
Scans the Sonarr queue every `INTERVAL` seconds and:
- For any release stuck as TBA, if forces a series refresh to try and fetch the latest data and un-stuck it
  (by default Sonarr refreshes *all* series once every **12** hours)
- For any release marked as not an upgrade for an existing file, it removes the entry and deletes it from your download client

## Running
To run the application, you can either use the standalone binary for your system, or the Docker image

### Running the binary
The binary runs as a normal CLI application (there is no GUI).\
Usage: `scrubarr [options] <Sonarr API key>`\
See [here](#variables) for all possible options.


### Running with Docker Compose

Example compose:

```yaml
version: "3.7"
services:
  scrubarr:
    image: ghcr.io/cpu-blanc/scrubarr:latest
    container_name: scrubarr
    environment:
      - SCRUBARR_SONARR_API_KEY=api_key #replace with your key. Required.
      - SCRUBARR_SONARR_URL=http://yourdomain.net #replace with the url your Sonarr instance is. Default: http://localhost:8989
      - SCRUBARR_SONARR_BASE_PATH=sonarr #this will result in requests to http://yourdomain.net/sonarr.
      - SCRUBARR_LOG_LEVEL=debug #will print debug logs - Can be trace, debug, info, warn, or error. Default: info 
      - SCRUBARR_INTERVAL=1200 #check the queue once every 20 minutes. Default: 600 (10 minutes)
    restart: unless-stopped

```

### Running with Docker Create
```
docker create \
  --name scrubarr \
  -e SCRUBARR_SONARR_API_KEY=api_key \
  --restart unless-stopped \
  ghcr.io/cpu-blanc/scrubarr:latest
```
### Variables


| CLI argument        | Env var                     | Type                                  | Default                 |
|---------------------|-----------------------------|---------------------------------------|-------------------------|
| N/A                 | `SCRUBARR_SONARR_API_KEY`   | String                                | **Required**            |
| `-l`, `--log`       | `SCRUBARR_LOG_LEVEL`        | `trace`,`debug`,`info`,`warn`,`error` | `info`                  |
| `-u`, `--url`       | `SCRUBARR_SONARR_URL`       | String                                | `http://localhost:8989` |
| `-b`, `--base-path` | `SCRUBARR_SONARR_BASE_PATH` | String                                | null                    |
| `-i`, `--interval`  | `SCRUBARR_INTERVAL`         | Int (in seconds: minimum 300)         | `600`                   |

Env vars can also be loaded from a `.env` file in the working directory