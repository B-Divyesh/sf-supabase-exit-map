# Demo sandbox

## CLI

Run this command from a clean checkout:

~~~sh
cargo run --bin supabase-exit-map -- --demo --format markdown
~~~

The CLI crate embeds a copy of the three sample files. The matching repository input is in examples/demo-project. On every run it copies the embedded sample to a new operating-system temporary directory, scans that copy, writes exit-map-demo.md there, prints the source and report paths, and leaves the copied files for inspection. It never reads the supplied project path in demo mode.

Use --json with the same command for machine-readable output. The JSON remains on standard output; demo paths are printed on standard error.

## Website

Open /demo/ or use the first-screen Try it with sample data action.

The page loads eleven realistic sample findings. Its only storage key begins with demo:supabase-exit-map:. The persistent banner says “Demo — sample data, nothing is saved” and includes Reset demo and Start for real. Reset deletes and recreates only the demo key. The normal Planning Room keys begin with sb_ and are not read or written by the demo page.
