# wca_userinfo

Tiny webserver that scrapes information for a user from
[worldcubeassosiation.org](https://www.worldcubeassociation.org/).

This is meant to be used to get information for one user (me) periodically (once
a week). It's **not** meant to be used to scrape the entire website, you're much better off using the [database dump](https://www.worldcubeassociation.org/results/misc/export.html) for that.

## Install:

Requires: `cargo`

```sh
git clone https://github.com/purarue/wca_userinfo
cd wca_userinfo
cargo build --bins --release
# or, to install globally
cargo install --path .
```

You can copy the `./target/release/wca_info` binary to anywhere you wish, that
executable runs the server.

## Run:

Default port is `8010`.

The `WCA_USERINFO_PORT` can be set to change which port this is hosted on.

```sh
./target/release/wca_userinfo
```

... or `WCA_USERINFO_PORT=8050 ./run_server`

## Example

You can supply the user ID as the first argument: `wca_userinfo 2012PARK03` to just grab info for a user and then exit

For the server, provide the WCA user ID as the path:

```sh
# for https://www.worldcubeassociation.org/persons/2012PARK03
$ curl --silent "http://localhost:8010/2012PARK03" | jq
{
  "country": "United States",
  "wca_id": "2012PARK03",
  "gender": "Male",
  "competitions": 185,
  "completed_solves": 6745,
  "events": [
    {
      "name": "3x3x3 Cube",
      "single": {
        "time": "3.13",
        "national": 1,
        "continent": 1,
        "world": 1
      },
      "average": {
        "time": "4.86",
        "national": 1,
        "continent": 1,
        "world": 5
      }
    },
    {
      "name": "2x2x2 Cube",
      "single": {
        "time": "2.88",
        "national": 3978,
        "continent": 5211,
        "world": 18505
      },
      "average": {
        "time": "4.31",
        "national": 3115,
        "continent": 4017,
        "world": 14205
      }
    },
    {
      "name": "4x4x4 Cube",
      "single": {
        "time": "15.71",
        "national": 1,
        "continent": 1,
        "world": 1
      },
      "average": {
        "time": "19.38",
        "national": 1,
        "continent": 1,
        "world": 1
      }
    },
    {
      "name": "5x5x5 Cube",
      "single": {
        "time": "32.52",
        "national": 1,
        "continent": 1,
        "world": 2
      },
      "average": {
        "time": "34.76",
        "national": 1,
        "continent": 1,
        "world": 1
      }
    },
    {
      "name": "6x6x6 Cube",
      "single": {
        "time": "58.03",
        "national": 1,
        "continent": 1,
        "world": 1
      },
      "average": {
        "time": "1:05.66",
        "national": 1,
        "continent": 1,
        "world": 1
      }
    },
    {
      "name": "7x7x7 Cube",
      "single": {
        "time": "1:34.15",
        "national": 1,
        "continent": 1,
        "world": 1
      },
      "average": {
        "time": "1:39.68",
        "national": 1,
        "continent": 1,
        "world": 1
      }
    },
    {
      "name": "3x3x3 One-Handed",
      "single": {
        "time": "6.20",
        "national": 1,
        "continent": 1,
        "world": 3
      },
      "average": {
        "time": "8.62",
        "national": 1,
        "continent": 1,
        "world": 5
      }
    },
    {
      "name": "Square-1",
      "single": {
        "time": "52.69",
        "national": 4216,
        "continent": 5253,
        "world": 16053
      },
      "average": {
        "time": "1:10.20",
        "national": 3713,
        "continent": 4648,
        "world": 14204
      }
    }
  ]
}
```
