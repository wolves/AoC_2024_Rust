set dotenv-load

create day:
    cargo generate --path ./daily-template --name {{day}}
    # just get-input {{day}}

get-input day:
  ./scripts/get-aoc-input.rs --day {{day}} --current-working-directory {{justfile_directory()}}
