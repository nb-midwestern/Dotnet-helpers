## Requirements

- [rust](https://rustup.rs/)

## How to use

Copy `example.env` to `.env`

fill out fields for 
`BASE_PROJECT_ROUTE=""`
`BASE_NAMESPACE=""`

run commands

## Commands

### Help

```sh
cargo run -- -h
```

### Generate QueryCriteria from Entity Name

```sh
cargo run -- -p generate-query-criterial-from-entity-name -x Customer -e CustomerId -t LastName,string FirstName,string -o o.cs
```

- `x` Entity Name
- `e` Entity Id
- `t` Sort Criteria fields and types (comma separated).
- `o` output file

### Generate TS interface from dto class

```sh
cargo run -- -p cs-dto-to-ts-interface -i "./files/dto.cs" -o output.ts
```

### Generate QueryCriteria from BaseCrudRepo

```sh
cargo run -- -i ./files/bcr.cs -o o.cs -p generate-query-criteria-from-base-crud-class -e CustomerId -t FirstName,string LastName,string
```


### Generate Service Unit test from service class
```sh
cargo run -- -p unit-test-generator -i "pathto/ContractService.cs"  -o test.cs
```
