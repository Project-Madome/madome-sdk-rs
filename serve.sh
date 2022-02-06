cargo doc --release --all-features

if [ $? -ne 0 ]; then
    exit 1
fi

deno run --allow-net --allow-read https://deno.land/std/http/file_server.ts ./target/doc -p 3111
