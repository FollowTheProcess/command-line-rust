# By default print the list of recipes
_default:
    @just --list

# Create a new command line project with a name
new name:
    cargo new {{ name }}
