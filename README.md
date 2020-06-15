# wca_userinfo

Tiny webserver that scrapes information for a user from
[worldcubeassosiation.org](https://www.worldcubeassociation.org/).

This is meant to be used to get information for one user (me) periodically (once
a week). It's **not** meant to be used to scrape the entire website, you're much better off using the [database dump](https://www.worldcubeassociation.org/results/misc/export.html) for that.

## Install:

Reqiures: `cargo`

```sh
git clone https://gitlab.com/seanbreckenridge/wca_userinfo
cd wca_userinfo
cargo build --bins --release
```

You can copy the `./target/release/wca_info` binary to anywhere you wish, that
executable runs the server.

## Run:

Default port is `8010`.

The `WCA_USERINFO_PORT` can be set to change which port this is hosted on.

``` sh
./target/release/wca_userinfo
```

... or `WCA_USERINFO_PORT=8050 ./run_server`

## Example

Provide the WCA user ID as the path:

``` sh
# for https://www.worldcubeassociation.org/persons/2017BREC02
curl --silent "http://localhost:8010/2017BREC02" | jq
{
  "country": "United States",
  "wca_id": "2017BREC02",
  "gender": "Male",
  "competitions": 4,
  "completed_solves": 59,
  "events": [
    {
      "name": "3x3x3 Cube",
      "single": {
        "time": "13.22",
        "national": 3512,
        "continent": 4880,
        "world": 21046
      },
      "average": {
        "time": "16.70",
        "national": 4055,
        "continent": 5622,
        "world": 24396
      }
    },
    {
      "name": "2x2x2 Cube",
      "single": {
        "time": "5.75",
        "national": 7933,
        "continent": 10933,
        "world": 40840
      },
      "average": {
        "time": "6.65",
        "national": 5415,
        "continent": 7492,
        "world": 28038
      }
    },
    {
      "name": "4x4x4 Cube",
      "single": {
        "time": "1:46.07",
        "national": 5684,
        "continent": 7908,
        "world": 30942
      },
      "average": {
        "time": null,
        "national": null,
        "continent": null,
        "world": null
      }
    },
    {
      "name": "3x3x3 One-Handed",
      "single": {
        "time": "30.44",
        "national": 2644,
        "continent": 3942,
        "world": 17675
      },
      "average": {
        "time": "33.06",
        "national": 2045,
        "continent": 3055,
        "world": 14093
      }
    },
    {
      "name": "Pyraminx",
      "single": {
        "time": "15.21",
        "national": 9576,
        "continent": 12883,
        "world": 48183
      },
      "average": {
        "time": "18.71",
        "national": 8524,
        "continent": 11494,
        "world": 42674
      }
    },
    {
      "name": "Skewb",
      "single": {
        "time": "7.94",
        "national": 2663,
        "continent": 3571,
        "world": 12845
      },
      "average": {
        "time": "15.34",
        "national": 4079,
        "continent": 5410,
        "world": 18359
      }
    }
  ]
}
```
