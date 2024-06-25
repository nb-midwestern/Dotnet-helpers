# Helpers

## Help

```shell
cargo run -- -h
```

## Generate TS interface from dto class

```shell
cargo run -- -p cs-dto-to-ts-interface -i "./files/dto.cs" -o output.ts
```

## Generate QueryCriteria from BaseCrudRepo

```shell
cargo run -- -i ./files/bcr.cs -o o.cs -p generate-query-criteria-from-base-crud-class -e some_entity_id
```
