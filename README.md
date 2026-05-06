# GW2 Fashion Exporter

CLI tool to export GW2 equipment tabs as fashion templates

# Running the CLI

Currently the CLI is very simple and will expot all characters equipments tabs as fashion template chat link in a file called fashion.csv.
It can be called with poetry:
```
GW2_API_KEY='<your-api-key-here>' poetry run python3 -m gw2fashion
```

The key can also be placed inside a .env file.