# Camp

CLI tool to alter the freeCodeCamp `user` collection.

## Installation

### From Source

```bash
cargo install --git https://github.com/ShaunSHamilton/camp.git
```

### Releases

https://github.com/ShaunSHamilton/camp/releases

## Usage

```bash
camp --help
A CLI tool to alter the freeCodeCamp database

Usage: camp [OPTIONS] <COMMAND>

Commands:
  claim-certs     Add all certification challenges to the user record
  add-challenges  Add selected challenges to user record
  finishFCC       Add ALL challenges to completedChallenges array
  add-users       Add <n> random user records to the database
  help            Print this message or the help of the given subcommand(s)

Options:
      --uri <URI>
          MongoDB connection string [default: mongodb://127.0.0.1:27017/freecodecamp?directConnection=true]
      --username <USERNAME>
          Username of user in the database [default: developmentuser]
  -c, --curriculum-path <CURRICULUM_PATH>
          Path to `curriculum.json` file [default: shared/config/curriculum.json]
  -h, --help
          Print help
  -V, --version
          Print version
```
