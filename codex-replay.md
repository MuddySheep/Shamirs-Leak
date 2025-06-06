# Codex Replay Log

`CodexResearcher` records every candidate mnemonic in this markdown file and a
companion CSV named `codex-replay.csv`.

CSV columns are:

```
attempt,prefix_len,hamming_distance,similarity
```

Use `--progress` on the CLI to print live updates whenever a better zpub
similarity is found. Progress output disables Rayon parallelism for clarity.
