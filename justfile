default:
    just --list

# https://studio.apollographql.com/public/Linear-API/variant/current/schema/sdl?selectedSchema=%23%40%21api%21%40%23
# update-graphql:
#    get-graphql-schema https://api.linear.app/graphql > graphql/linear-api.graphql

run *args="":
    cargo run -- {{ args }}
