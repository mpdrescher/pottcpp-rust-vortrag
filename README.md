# pottcpp-rust-vortrag

Run the different binaries to see the difference between:
- `map`: normal map
- `pmap1`: parallel map with a thread per item
- `pmap2`: parallel map that splits the task into [THREADS] parts and processes them in parallel
- `pmap3`: parallel map that uses a future per item
- `pmap4`: parallel map that splits the task into [THREADS] futures
