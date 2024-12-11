# Scrubarr

## Overview
Scrubarr is a lightweight app written in Rust with the sole purpose of cleaning up and expediting the Sonarr queue. Many anime simulcasts get stuck
in the queue as TBA for multiple hours resulting in a long delay before import, this app helps relieve that.\
This little project was mainly a playground to learn more about dockerising applications and automating those workflows; increases in scope are not to be expected.

### Available Image Tags
- `latest` - The latest stable release
- `beta` - The latest build, includes pre-releases as well as stable releases - Whichever is newest
- Semver (ie `0.2`, `0.2.1` etc)

## Features
Scans the Sonarr queue every `INTERVAL` seconds and:
- For any release stuck as TBA, if forces a series refresh to try and fetch the latest data and un-stuck it
  (by default Sonarr refreshes *all* series once every **12** hours)
- For any release marked as not an upgrade for an existing file, it removes the entry and deletes it from your download client

## Running
To run the application, you can either use the standalone binary for your system, or the Docker image

### Running the binary
The binary runs as a normal CLI application (there is no GUI).\
All settings are loaded from [env vars](#variables) or [the config file](#example-settingsjson)


### Running with Docker Compose

Example compose:

```yaml
version: "3.7"
services:
  scrubarr:
    image: ghcr.io/cpu-blanc/scrubarr:latest
    container_name: scrubarr
    environment:
      - SCRUBARR_SONARR_1_KEY=api_key #replace with your key. Required.
      - SCRUBARR_SONARR_1_URL=http://yourdomain.net #replace with the url your Sonarr instance is. Default: http://localhost:8989
      - SCRUBARR_SONARR_1_BASE=sonarr #this will result in requests to http://yourdomain.net/sonarr.
      - SCRUBARR_SONARR_2_KEY=api_key2 #for multiple instances
      - SCRUBARR_SONARR_2_URL=http://anotherdomain.com #etc
      - SCRUBARR_LOG_LEVEL=debug #will print debug logs - Can be trace, debug, info, warn, or error. Default: info 
      - SCRUBARR_INTERVAL=1200 #check the queue once every 20 minutes. Default: 600 (10 minutes)
    restart: unless-stopped

```

### Running with Docker Create
```
docker create \
  --name scrubarr \
  -e SCRUBARR_SONARR_1_KEY=api_key \
  --restart unless-stopped \
  ghcr.io/cpu-blanc/scrubarr:latest
```
### Variables


| Env var                      | Config value        | Type                                  | Default                 |
|------------------------------|---------------------|---------------------------------------|-------------------------|
| `SCRUBARR_SONARR_[int]_KEY`  | `sonarr.[int].key`  | String                                | **Required**            |
| `SCRUBARR_SONARR_[int]_URL`  | `sonarr.[int].url`  | String                                | `http://localhost:8989` |
| `SCRUBARR_SONARR_[int]_BASE` | `sonarr.[int].base` | String                                | null                    |
| `SCRUBARR_LOG_LEVEL`         | `log_level`         | `trace`,`debug`,`info`,`warn`,`error` | `info`                  |
| `SCRUBARR_INTERVAL`          | `interval`          | Int (in seconds: minimum 300)         | `600`                   |
| `SCRUBARR_VERBOSE`           | `verbose`           | Bool                                  | `false`                 |

Env vars can also be loaded from a `.env` file in the working directory

### Example settings.json

```json
{
  "log_level": "INFO",
  "interval": 600,
  "sonarr": {
    "1": {
      "base": null,
      "key": "some-api-key",
      "url": "http://localhost:8989/"
    },
    "2": {
      "base": "sonarr",
      "key": "some-other-key",
      "url": "http://another.domain"
    }
  },
  "verbose": false
}
```