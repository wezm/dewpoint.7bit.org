default:
    @just --list

alias ack := acknowledgements

# Print acknowledgements
acknowledgements:
    cargo metadata --format-version 1 --filter-platform x86_64-unknown-linux-gnu \
    | jq --raw-output \
        '.packages | sort_by(.name)[] | "<li><a href=\"" + (.homepage // .repository // .documentation) + "\">" + .name + "</a> — " + .license + "</li>"' \
    | rg -v '>dewpoint<'
