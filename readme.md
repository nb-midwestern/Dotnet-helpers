# Helpers

## Generate TS interface from dto class

```shell
cargo run -- csharp_dto_to_ts_interface "./files/dto.cs"  output.ts
```

## Generate QueryCriteria from BaseCrudRepo

```shell
cargo run -- gen_query "./files/bcr.cs" out.cs
```
